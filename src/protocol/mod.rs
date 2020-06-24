use chrono::prelude::TimeZone;
use chrono::{DateTime, Utc};
use core::fmt::Debug;
use log::warn;
use std::error::Error;
use std::fmt;
use std::io::Write;
use xml::writer::{EmitterConfig, XmlEvent};
#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
#[cfg(test)]
use rand::seq::SliceRandom;

fn bool2str(b: bool) -> &'static str {
    return if b { "1" } else { "0" };
}
fn str2bool(s: &str) -> bool {
    return if s == "1" { true } else { false };
}

fn write_simple<W: Write>(
    writer: &mut xml::EventWriter<W>,
    name: &str,
    value: &String,
) -> Result<(), GenerateError> {
    writer.write(XmlEvent::start_element(name))?;
    writer.write(&value[..])?;
    writer.write(XmlEvent::end_element())?;
    Ok(())
}

fn write_empty_tag<W: Write>(
    writer: &mut xml::EventWriter<W>,
    name: &str,
) -> Result<(), GenerateError> {
    writer.write(XmlEvent::start_element(name))?;
    writer.write(XmlEvent::end_element())?;
    Ok(())
}

fn write_fault_struct<W: Write>(
    writer: &mut xml::EventWriter<W>,
    fault: &FaultStruct,
) -> Result<(), GenerateError> {
    writer.write(XmlEvent::start_element("FaultStruct"))?;
    write_simple(writer, "FaultCode", &fault.code.to_string())?;
    write_simple(writer, "FaultString", &fault.string)?;
    writer.write(XmlEvent::end_element())?;
    Ok(())
}

fn write_fault<W: Write>(
    writer: &mut xml::EventWriter<W>,
    fault: &FaultStruct,
) -> Result<(), GenerateError> {
    writer.write(XmlEvent::start_element("Fault"))?;
    write_fault_struct(writer, fault)?;
    writer.write(XmlEvent::end_element())?;
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ID {
    must_understand: bool,
    id: String,
}

impl ID {
    pub fn new(must_understand: bool, id: String) -> Self {
        ID {
            must_understand: must_understand,
            id: id,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(
            XmlEvent::start_element(&cwmp_prefix(has_cwmp, "ID")[..])
                .attr("mustUnderstand", bool2str(self.must_understand)),
        )?;
        writer.write(&self.id[..])?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

#[cfg(test)]
impl Arbitrary for ID {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        ID::new(bool::arbitrary(g), String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.must_understand.clone(), self.id.clone())
                .shrink()
                .map(|(m, i)| ID {
                    must_understand: m,
                    id: i,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct HoldRequests {
    must_understand: bool,
    hold: bool,
}

impl HoldRequests {
    pub fn new(must_understand: bool, hold: bool) -> Self {
        HoldRequests {
            must_understand: must_understand,
            hold: hold,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(
            XmlEvent::start_element(&cwmp_prefix(has_cwmp, "HoldRequests")[..])
                .attr("mustUnderstand", bool2str(self.must_understand)),
        )?;

        writer.write(bool2str(self.hold))?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

#[cfg(test)]
impl Arbitrary for HoldRequests {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        HoldRequests::new(bool::arbitrary(g), bool::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.must_understand.clone(), self.hold.clone())
                .shrink()
                .map(|(m, h)| HoldRequests {
                    must_understand: m,
                    hold: h,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SessionTimeout {
    must_understand: bool,
    timeout: u32,
}

impl SessionTimeout {
    pub fn new(must_understand: bool, timeout: u32) -> Self {
        SessionTimeout {
            must_understand: must_understand,
            timeout: timeout,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(
            XmlEvent::start_element(&cwmp_prefix(has_cwmp, "SessionTimeout")[..])
                .attr("mustUnderstand", bool2str(self.must_understand)),
        )?;
        writer.write(&self.timeout.to_string()[..])?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

#[cfg(test)]
impl Arbitrary for SessionTimeout {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        SessionTimeout::new(bool::arbitrary(g), u32::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.must_understand.clone(), self.timeout.clone())
                .shrink()
                .map(|(m, t)| SessionTimeout {
                    must_understand: m,
                    timeout: t,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct NoMoreRequests {
    must_understand: bool,
    value: u8,
}

impl NoMoreRequests {
    pub fn new(must_understand: bool, value: u8) -> Self {
        NoMoreRequests {
            must_understand: must_understand,
            value: value,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(
            XmlEvent::start_element(&cwmp_prefix(has_cwmp, "NoMoreRequests")[..])
                .attr("mustUnderstand", bool2str(self.must_understand)),
        )?;
        writer.write(&self.value.to_string()[..])?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

#[cfg(test)]
impl Arbitrary for NoMoreRequests {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        NoMoreRequests::new(bool::arbitrary(g), u8::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.must_understand.clone(), self.value.clone())
                .shrink()
                .map(|(m, v)| NoMoreRequests {
                    must_understand: m,
                    value: v,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SupportedCWMPVersions {
    must_understand: bool,
    value: String,
}

impl SupportedCWMPVersions {
    pub fn new(must_understand: bool, value: String) -> Self {
        SupportedCWMPVersions {
            must_understand: must_understand,
            value: value,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(
            XmlEvent::start_element(&cwmp_prefix(has_cwmp, "SupportedCWMPVersions")[..])
                .attr("mustUnderstand", bool2str(self.must_understand)),
        )?;
        writer.write(&self.value.to_string()[..])?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

#[cfg(test)]
impl Arbitrary for SupportedCWMPVersions {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        SupportedCWMPVersions::new(bool::arbitrary(g), String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.must_understand.clone(), self.value.clone())
                .shrink()
                .map(|(m, v)| SupportedCWMPVersions {
                    must_understand: m,
                    value: v,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct UseCWMPVersion {
    must_understand: bool,
    value: String,
}

impl UseCWMPVersion {
    pub fn new(must_understand: bool, value: String) -> Self {
        UseCWMPVersion {
            must_understand: must_understand,
            value: value,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(
            XmlEvent::start_element(&cwmp_prefix(has_cwmp, "UseCWMPVersion")[..])
                .attr("mustUnderstand", bool2str(self.must_understand)),
        )?;
        writer.write(&self.value.to_string()[..])?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

#[cfg(test)]
impl Arbitrary for UseCWMPVersion {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        UseCWMPVersion::new(bool::arbitrary(g), String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.must_understand.clone(), self.value.clone())
                .shrink()
                .map(|(m, v)| UseCWMPVersion {
                    must_understand: m,
                    value: v,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum HeaderElement {
    ID(ID),
    HoldRequests(HoldRequests),
    SessionTimeout(SessionTimeout),
    NoMoreRequests(NoMoreRequests),
    SupportedCWMPVersions(SupportedCWMPVersions),
    UseCWMPVersion(UseCWMPVersion),
}

#[cfg(test)]
impl Arbitrary for HeaderElement {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let vals = vec![
            HeaderElement::ID(ID::arbitrary(g)),
            HeaderElement::HoldRequests(HoldRequests::arbitrary(g)),
            HeaderElement::SessionTimeout(SessionTimeout::arbitrary(g)),
            HeaderElement::NoMoreRequests(NoMoreRequests::arbitrary(g)),
            HeaderElement::SupportedCWMPVersions(SupportedCWMPVersions::arbitrary(g)),
            HeaderElement::UseCWMPVersion(UseCWMPVersion::arbitrary(g)),
        ];
        vals.choose(g).unwrap().clone()
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        match self {
            &HeaderElement::ID(ref x) => Box::new(x.shrink().map(|s| HeaderElement::ID(s))),
            &HeaderElement::HoldRequests(ref x) => {
                Box::new(x.shrink().map(|s| HeaderElement::HoldRequests(s)))
            }
            &HeaderElement::SessionTimeout(ref x) => {
                Box::new(x.shrink().map(|s| HeaderElement::SessionTimeout(s)))
            }
            &HeaderElement::NoMoreRequests(ref x) => {
                Box::new(x.shrink().map(|s| HeaderElement::NoMoreRequests(s)))
            }
            &HeaderElement::SupportedCWMPVersions(ref x) => {
                Box::new(x.shrink().map(|s| HeaderElement::SupportedCWMPVersions(s)))
            }
            &HeaderElement::UseCWMPVersion(ref x) => {
                Box::new(x.shrink().map(|s| HeaderElement::UseCWMPVersion(s)))
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct FaultStruct {
    code: u32,
    string: String,
}

impl FaultStruct {
    pub fn new(code: u32, string: String) -> Self {
        FaultStruct {
            code: code,
            string: string,
        }
    }
    pub fn set_code(&mut self, code: u32) {
        self.code = code;
    }
    pub fn set_string(&mut self, string: &str) {
        self.string = string.to_string();
    }
}

#[cfg(test)]
impl Arbitrary for FaultStruct {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        FaultStruct::new(u32::arbitrary(g), String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.code.clone(), self.string.clone())
                .shrink()
                .map(|(c, s)| FaultStruct { code: c, string: s }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct AddObjectResponse {
    instance_number: u32,
    status: String,
}

impl AddObjectResponse {
    pub fn new(instance_number: u32, status: String) -> Self {
        AddObjectResponse {
            instance_number: instance_number,
            status: status,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "AddObjectResponse")[..],
        ))?;

        writer.write(XmlEvent::start_element("InstanceNumber"))?;
        writer.write(&self.instance_number.to_string()[..])?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::start_element("Status"))?;
        writer.write(&self.status[..])?;
        writer.write(XmlEvent::end_element())?;

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["AddObjectResponse", "InstanceNumber"] => {
                self.instance_number = characters.parse().unwrap();
            }
            ["AddObjectResponse", "Status"] => {
                self.status = characters.to_string();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for AddObjectResponse {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        AddObjectResponse::new(u32::arbitrary(g), String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.instance_number.clone(), self.status.clone())
                .shrink()
                .map(|(i, s)| AddObjectResponse {
                    instance_number: i,
                    status: s,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct AddObject {
    object_name: String,
    parameter_key: String,
}

impl AddObject {
    pub fn new(object_name: String, parameter_key: String) -> Self {
        AddObject {
            object_name: object_name,
            parameter_key: parameter_key,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "AddObject")[..],
        ))?;
        write_simple(writer, "ObjectName", &self.object_name)?;
        write_simple(writer, "ParameterKey", &self.parameter_key)?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["AddObject", "ObjectName"] => {
                self.object_name = characters.to_string();
            }
            ["AddObject", "ParameterKey"] => {
                self.parameter_key = characters.to_string();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for AddObject {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        AddObject::new(String::arbitrary(g), String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.object_name.clone(), self.parameter_key.clone())
                .shrink()
                .map(|(o, p)| AddObject {
                    object_name: o,
                    parameter_key: p,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct AutonomousDUStateChangeCompleteResponse;

impl AutonomousDUStateChangeCompleteResponse {
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        write_empty_tag(
            writer,
            &cwmp_prefix(has_cwmp, "AutonomousDUStateChangeCompleteResponse")[..],
        )?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct AutonOpResult {
    uuid: String,
    deployment_unit_ref: String,
    version: String,
    current_state: String,
    resolved: String,
    execution_unit_ref_list: String,
    start_time: Option<DateTime<Utc>>,
    complete_time: Option<DateTime<Utc>>,
    fault: FaultStruct,
    operation_performed: String,
}

impl AutonOpResult {
    pub fn new(
        uuid: String,
        deployment_unit_ref: String,
        version: String,
        current_state: String,
        resolved: String,
        execution_unit_ref_list: String,
        start_time: DateTime<Utc>,
        complete_time: DateTime<Utc>,
        fault_code: u32,
        fault_string: String,
        operation_performed: String,
    ) -> Self {
        AutonOpResult {
            uuid: uuid.to_string(),
            deployment_unit_ref: deployment_unit_ref.to_string(),
            version: version.to_string(),
            current_state: current_state.to_string(),
            resolved: resolved.to_string(),
            execution_unit_ref_list: execution_unit_ref_list.to_string(),
            start_time: Some(start_time),
            complete_time: Some(complete_time),
            fault: FaultStruct::new(fault_code, fault_string),
            operation_performed: operation_performed.to_string(),
        }
    }
}

#[cfg(test)]
impl Arbitrary for AutonOpResult {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let bogus_st = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let bogus_ct = Utc.ymd(2014, 11, 29).and_hms(12, 0, 9);

        AutonOpResult::new(
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            bogus_st,
            bogus_ct,
            u32::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        // we will remove times from shrinking since qc only supports a
        // tuple size of max 8, and then make the times constant across
        // arbitration
        Box::new(
            (
                self.uuid.clone(),
                self.deployment_unit_ref.clone(),
                self.version.clone(),
                self.current_state.clone(),
                self.resolved.clone(),
                self.execution_unit_ref_list.clone(),
                // only 8 elements allowed by quickcheck in a tuple
                // self.start_time.clone(),
                // self.complete_time.clone(),
                self.fault.clone(),
                self.operation_performed.clone(),
            )
                .shrink()
                .map(|(uuid, dur, ver, cs, res, eurl, f, op)| AutonOpResult {
                    uuid: uuid,
                    deployment_unit_ref: dur,
                    version: ver,
                    current_state: cs,
                    resolved: res,
                    execution_unit_ref_list: eurl,
                    start_time: Some(Utc.ymd(2014, 11, 28).and_hms(12, 0, 9)),
                    complete_time: Some(Utc.ymd(2014, 11, 29).and_hms(12, 0, 9)),
                    fault: f,
                    operation_performed: op,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct AutonomousDUStateChangeComplete {
    results: Vec<AutonOpResult>,
}

impl AutonomousDUStateChangeComplete {
    pub fn new(results: Vec<AutonOpResult>) -> Self {
        AutonomousDUStateChangeComplete { results: results }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        let ss = if has_cwmp {
            writer.write(XmlEvent::start_element(
                "cwmp:AutonomousDUStateChangeComplete",
            ))?;
            format!("cwmp:AutonOpResultStruct[{}]", self.results.len())
        } else {
            writer.write(XmlEvent::start_element("AutonomousDUStateChangeComplete"))?;

            format!("AutonOpResultStruct[{}]", self.results.len())
        };

        writer.write(XmlEvent::start_element("Results").attr("SOAP-ENC:arrayType", &ss[..]))?;

        for r in self.results.iter() {
            writer.write(XmlEvent::start_element("AutonOpResultStruct"))?;

            write_simple(writer, "UUID", &r.uuid)?;
            write_simple(writer, "DeploymentUnitRef", &r.deployment_unit_ref)?;
            write_simple(writer, "Version", &r.version)?;
            write_simple(writer, "CurrentState", &r.current_state)?;
            write_simple(writer, "Resolved", &r.resolved)?;
            write_simple(writer, "ExecutionUnitRefList", &r.execution_unit_ref_list)?;
            match r.start_time {
                None => {}
                Some(dt) => write_simple(writer, "StartTime", &dt.to_rfc3339())?,
            }
            match r.complete_time {
                None => {}
                Some(dt) => write_simple(writer, "CompleteTime", &dt.to_rfc3339())?,
            }
            write_fault(writer, &r.fault)?;
            write_simple(writer, "OperationPerformed", &r.operation_performed)?;

            // AutonOpResultStruct
            writer.write(XmlEvent::end_element())?;
        }
        // Results
        writer.write(XmlEvent::end_element())?;

        // cwmp:AutonomousDUStateChangeComplete
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["AutonomousDUStateChangeComplete", "Results", "AutonOpResultStruct"] => {
                self.results.push(AutonOpResult::default());
            }
            _ => {}
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["AutonomousDUStateChangeComplete", "Results", "AutonOpResultStruct", key] => {
                let last = self.results.last_mut();
                match last {
                    Some(e) => match key {
                        "UUID" => e.uuid = characters.to_string(),
                        "DeploymentUnitRef" => e.deployment_unit_ref = characters.to_string(),
                        "Version" => e.version = characters.to_string(),
                        "CurrentState" => e.current_state = characters.to_string(),
                        "Resolved" => e.resolved = characters.to_string(),
                        "ExecutionUnitRefList" => {
                            e.execution_unit_ref_list = characters.to_string()
                        }
                        "StartTime" => match characters.parse::<DateTime<Utc>>() {
                            Ok(dt) => e.start_time = Some(dt),
                            _ => {}
                        },
                        "CompleteTime" => match characters.parse::<DateTime<Utc>>() {
                            Ok(dt) => e.complete_time = Some(dt),
                            _ => {}
                        },
                        "OperationPerformed" => e.operation_performed = characters.to_string(),
                        _ => {}
                    },
                    None => {}
                }
            }
            ["AutonomousDUStateChangeComplete", "Results", "AutonOpResultStruct", "Fault", "FaultStruct", key] =>
            {
                let last = self.results.last_mut();
                match last {
                    Some(e) => match key {
                        "FaultCode" => match characters.parse::<u32>() {
                            Ok(parsed) => e.fault.set_code(parsed),
                            _ => {}
                        },
                        "FaultString" => e.fault.set_string(&characters[..]),
                        _ => {}
                    },
                    None => {}
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for AutonomousDUStateChangeComplete {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        AutonomousDUStateChangeComplete::new(Vec::<AutonOpResult>::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.results.clone())
                .shrink()
                .map(|r| AutonomousDUStateChangeComplete { results: r }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct AutonomousTransferCompleteResponse;

impl AutonomousTransferCompleteResponse {
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        write_empty_tag(
            writer,
            &cwmp_prefix(has_cwmp, "AutonomousTransferCompleteResponse")[..],
        )?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct AutonomousTransferComplete {
    announce_url: String,
    transfer_url: String,
    is_download: u8,
    file_type: String,
    file_size: u32,
    target_filename: String,
    fault: FaultStruct,
    start_time: Option<DateTime<Utc>>,
    complete_time: Option<DateTime<Utc>>,
}

impl AutonomousTransferComplete {
    pub fn new(
        announce_url: String,
        transfer_url: String,
        is_download: u8,
        file_type: String,
        file_size: u32,
        target_filename: String,
        fault: FaultStruct,
        start_time: DateTime<Utc>,
        complete_time: DateTime<Utc>,
    ) -> Self {
        AutonomousTransferComplete {
            announce_url: announce_url,
            transfer_url: transfer_url,
            is_download: is_download,
            file_type: file_type,
            file_size: file_size,
            target_filename: target_filename,
            fault: fault,
            start_time: Some(start_time),
            complete_time: Some(complete_time),
        }
    }

    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "AutonomousTransferComplete")[..],
        ))?;
        write_simple(writer, "AnnounceURL", &self.announce_url)?;
        write_simple(writer, "TransferURL", &self.transfer_url)?;
        write_simple(writer, "IsDownload", &self.is_download.to_string())?;
        write_simple(writer, "FileType", &self.file_type)?;
        write_simple(writer, "FileSize", &self.file_size.to_string())?;
        write_simple(writer, "TargetFileName", &self.target_filename)?;
        write_fault_struct(writer, &self.fault)?;
        match self.start_time {
            None => {}
            Some(dt) => write_simple(writer, "StartTime", &dt.to_rfc3339())?,
        }
        match self.complete_time {
            None => {}
            Some(dt) => write_simple(writer, "CompleteTime", &dt.to_rfc3339())?,
        }
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }

    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["AutonomousTransferComplete", "AnnounceURL"] => {
                self.announce_url = characters.to_string()
            }
            ["AutonomousTransferComplete", "TransferURL"] => {
                self.transfer_url = characters.to_string()
            }
            ["AutonomousTransferComplete", "IsDownload"] => {
                self.is_download = parse_to_int(characters, 0)
            }
            ["AutonomousTransferComplete", "TargetFileName"] => {
                self.target_filename = characters.to_string()
            }
            ["AutonomousTransferComplete", "FileType"] => self.file_type = characters.to_string(),
            ["AutonomousTransferComplete", "FileSize"] => {
                self.file_size = parse_to_int(characters, 0)
            }
            ["AutonomousTransferComplete", "StartTime"] => {
                match characters.parse::<DateTime<Utc>>() {
                    Ok(dt) => self.start_time = Some(dt),
                    _ => {}
                }
            }
            ["AutonomousTransferComplete", "CompleteTime"] => {
                match characters.parse::<DateTime<Utc>>() {
                    Ok(dt) => self.complete_time = Some(dt),
                    _ => {}
                }
            }
            ["AutonomousTransferComplete", "FaultStruct", "FaultCode"] => {
                self.fault.set_code(parse_to_int(characters, 0))
            }
            ["AutonomousTransferComplete", "FaultStruct", "FaultString"] => {
                self.fault.set_string(characters)
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for AutonomousTransferComplete {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        // times are not arbitrary due to qc
        // tuple (used in shrink) limitations
        AutonomousTransferComplete::new(
            String::arbitrary(g),
            String::arbitrary(g),
            u8::arbitrary(g),
            String::arbitrary(g),
            u32::arbitrary(g),
            String::arbitrary(g),
            FaultStruct::arbitrary(g),
            Utc.ymd(2014, 11, 28).and_hms(12, 0, 9),
            Utc.ymd(2014, 11, 29).and_hms(12, 0, 9),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.announce_url.clone(),
                self.transfer_url.clone(),
                self.is_download.clone(),
                self.file_type.clone(),
                self.file_size.clone(),
                self.target_filename.clone(),
                self.fault.clone(),
            )
                .shrink()
                .map(|(a, t, i, ft, fs, tf, f)| AutonomousTransferComplete {
                    announce_url: a,
                    transfer_url: t,
                    is_download: i,
                    file_type: ft,
                    file_size: fs,
                    target_filename: tf,
                    fault: f,
                    start_time: Some(Utc.ymd(2014, 11, 28).and_hms(12, 0, 9)),
                    complete_time: Some(Utc.ymd(2014, 11, 29).and_hms(12, 0, 9)),
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct CancelTransferResponse;

impl CancelTransferResponse {
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        write_empty_tag(writer, &cwmp_prefix(has_cwmp, "CancelTransferResponse")[..])?;
        Ok(())
    }
}
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct CancelTransfer {
    command_key: String,
}

impl CancelTransfer {
    pub fn new(command_key: String) -> Self {
        CancelTransfer {
            command_key: command_key,
        }
    }

    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "CancelTransfer")[..],
        ))?;
        write_simple(writer, "CommandKey", &self.command_key)?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }

    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["CancelTransfer", "CommandKey"] => self.command_key = characters.to_string(),

            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for CancelTransfer {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        CancelTransfer::new(String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.command_key
                .clone()
                .shrink()
                .map(|c| CancelTransfer { command_key: c }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ChangeDUStateResponse;

impl ChangeDUStateResponse {
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        write_empty_tag(writer, &cwmp_prefix(has_cwmp, "ChangeDUStateResponse")[..])?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct InstallOp {
    url: String,
    uuid: String,
    username: String,
    password: String,
    execution_env_ref: String,
}

impl InstallOp {
    pub fn new(
        url: String,
        uuid: String,
        username: String,
        password: String,
        execution_env_ref: String,
    ) -> Self {
        InstallOp {
            url: url,
            uuid: uuid,
            username: username,
            password: password,
            execution_env_ref: execution_env_ref,
        }
    }
}

#[cfg(test)]
impl Arbitrary for InstallOp {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        InstallOp::new(
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.url.clone(),
                self.uuid.clone(),
                self.username.clone(),
                self.password.clone(),
                self.execution_env_ref.clone(),
            )
                .shrink()
                .map(|(u, uu, un, pw, eer)| InstallOp {
                    url: u,
                    uuid: uu,
                    username: un,
                    password: pw,
                    execution_env_ref: eer,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct UninstallOp {
    url: String,
    uuid: String,
    execution_env_ref: String,
}

impl UninstallOp {
    pub fn new(url: String, uuid: String, execution_env_ref: String) -> Self {
        UninstallOp {
            url: url,
            uuid: uuid,
            execution_env_ref: execution_env_ref,
        }
    }
}

#[cfg(test)]
impl Arbitrary for UninstallOp {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        UninstallOp::new(
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.url.clone(),
                self.uuid.clone(),
                self.execution_env_ref.clone(),
            )
                .shrink()
                .map(|(u, uu, eer)| UninstallOp {
                    url: u,
                    uuid: uu,
                    execution_env_ref: eer,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct UpdateOp {
    url: String,
    uuid: String,
    username: String,
    password: String,
    version: String,
}

impl UpdateOp {
    pub fn new(
        url: String,
        uuid: String,
        username: String,
        password: String,
        version: String,
    ) -> Self {
        UpdateOp {
            url: url,
            uuid: uuid,
            username: username,
            password: password,
            version: version,
        }
    }
}

#[cfg(test)]
impl Arbitrary for UpdateOp {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        UpdateOp::new(
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.url.clone(),
                self.uuid.clone(),
                self.username.clone(),
                self.password.clone(),
                self.version.clone(),
            )
                .shrink()
                .map(|(u, uu, un, pw, v)| UpdateOp {
                    url: u,
                    uuid: uu,
                    username: un,
                    password: pw,
                    version: v,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ChangeDUState {
    command_key: String,
    install_operations: Vec<InstallOp>,
    uninstall_operations: Vec<UninstallOp>,
    update_operations: Vec<UpdateOp>,
}

impl ChangeDUState {
    pub fn new(
        command_key: String,
        install_operations: Vec<InstallOp>,
        uninstall_operations: Vec<UninstallOp>,
        update_operations: Vec<UpdateOp>,
    ) -> Self {
        ChangeDUState {
            command_key: command_key,
            install_operations: install_operations,
            uninstall_operations: uninstall_operations,
            update_operations: update_operations,
        }
    }

    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "ChangeDUState")[..],
        ))?;
        write_simple(writer, "CommandKey", &self.command_key)?;
        writer.write(XmlEvent::start_element("Operations"))?;

        for io in self.install_operations.iter() {
            writer.write(XmlEvent::start_element("InstallOpStruct"))?;
            write_simple(writer, "URL", &io.url)?;
            write_simple(writer, "UUID", &io.uuid)?;
            write_simple(writer, "Username", &io.username)?;
            write_simple(writer, "Password", &io.password)?;
            write_simple(writer, "ExecutionEnvRef", &io.execution_env_ref)?;
            writer.write(XmlEvent::end_element())?;
        }
        for uio in self.uninstall_operations.iter() {
            writer.write(XmlEvent::start_element("UninstallOpStruct"))?;
            write_simple(writer, "URL", &uio.url)?;
            write_simple(writer, "UUID", &uio.uuid)?;
            write_simple(writer, "ExecutionEnvRef", &uio.execution_env_ref)?;
            writer.write(XmlEvent::end_element())?;
        }
        for uo in self.update_operations.iter() {
            writer.write(XmlEvent::start_element("UpdateOpStruct"))?;
            write_simple(writer, "URL", &uo.url)?;
            write_simple(writer, "UUID", &uo.uuid)?;
            write_simple(writer, "Username", &uo.username)?;
            write_simple(writer, "Password", &uo.password)?;
            write_simple(writer, "Version", &uo.version)?;
            writer.write(XmlEvent::end_element())?;
        }

        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["ChangeDUState", "Operations", "InstallOpStruct"] => {
                self.install_operations.push(InstallOp::new(
                    String::from(""),
                    String::from(""),
                    String::from(""),
                    String::from(""),
                    String::from(""),
                ))
            }
            ["ChangeDUState", "Operations", "UninstallOpStruct"] => self.uninstall_operations.push(
                UninstallOp::new(String::from(""), String::from(""), String::from("")),
            ),
            ["ChangeDUState", "Operations", "UpdateOpStruct"] => {
                self.update_operations.push(UpdateOp::new(
                    String::from(""),
                    String::from(""),
                    String::from(""),
                    String::from(""),
                    String::from(""),
                ))
            }
            _ => {}
        }
    }

    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["ChangeDUState", "CommandKey"] => self.command_key = characters.to_string(),
            ["ChangeDUState", "Operations", "InstallOpStruct", key] => {
                let last = self.install_operations.last_mut();
                match last {
                    Some(e) => match key {
                        "URL" => e.url = characters.to_string(),
                        "UUID" => e.uuid = characters.to_string(),
                        "Username" => e.username = characters.to_string(),
                        "Password" => e.password = characters.to_string(),
                        "ExecutionEnvRef" => e.execution_env_ref = characters.to_string(),
                        _ => {}
                    },
                    None => {}
                }
            }
            ["ChangeDUState", "Operations", "UninstallOpStruct", key] => {
                let last = self.uninstall_operations.last_mut();
                match last {
                    Some(e) => match key {
                        "URL" => e.url = characters.to_string(),
                        "UUID" => e.uuid = characters.to_string(),
                        "ExecutionEnvRef" => e.execution_env_ref = characters.to_string(),
                        _ => {}
                    },
                    None => {}
                }
            }
            ["ChangeDUState", "Operations", "UpdateOpStruct", key] => {
                let last = self.update_operations.last_mut();
                match last {
                    Some(e) => match key {
                        "URL" => e.url = characters.to_string(),
                        "UUID" => e.uuid = characters.to_string(),
                        "Username" => e.username = characters.to_string(),
                        "Password" => e.password = characters.to_string(),
                        "Version" => e.version = characters.to_string(),
                        _ => {}
                    },
                    None => {}
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for ChangeDUState {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        ChangeDUState::new(
            String::arbitrary(g),
            Vec::<InstallOp>::arbitrary(g),
            Vec::<UninstallOp>::arbitrary(g),
            Vec::<UpdateOp>::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.command_key.clone(),
                self.install_operations.clone(),
                self.uninstall_operations.clone(),
                self.update_operations.clone(),
            )
                .shrink()
                .map(|(c, i, un, up)| ChangeDUState {
                    command_key: c,
                    install_operations: i,
                    uninstall_operations: un,
                    update_operations: up,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct DeleteObjectResponse {
    status: String,
}

impl DeleteObjectResponse {
    pub fn new(status: String) -> Self {
        DeleteObjectResponse { status: status }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "DeleteObjectResponse")[..],
        ))?;
        write_simple(writer, "Status", &self.status)?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["DeleteObjectResponse", "Status"] => self.status = characters.to_string(),
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for DeleteObjectResponse {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        DeleteObjectResponse::new(String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.status
                .clone()
                .shrink()
                .map(|s| DeleteObjectResponse { status: s }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct DeleteObject {
    object_name: String,
    parameter_key: String,
}

impl DeleteObject {
    pub fn new(object_name: String, parameter_key: String) -> Self {
        DeleteObject {
            object_name: object_name,
            parameter_key: parameter_key,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "DeleteObject")[..],
        ))?;
        write_simple(writer, "ObjectName", &self.object_name)?;
        write_simple(writer, "ParameterKey", &self.parameter_key)?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["DeleteObject", "ObjectName"] => {
                self.object_name = characters.to_string();
            }
            ["DeleteObject", "ParameterKey"] => {
                self.parameter_key = characters.to_string();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for DeleteObject {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        DeleteObject::new(String::arbitrary(g), String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.object_name.clone(), self.parameter_key.clone())
                .shrink()
                .map(|(o, p)| DeleteObject {
                    object_name: o,
                    parameter_key: p,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct DownloadResponse {
    status: String,
    start_time: Option<DateTime<Utc>>,
    complete_time: Option<DateTime<Utc>>,
}

impl DownloadResponse {
    pub fn new(status: String, start_time: DateTime<Utc>, complete_time: DateTime<Utc>) -> Self {
        DownloadResponse {
            status: status,
            start_time: Some(start_time),
            complete_time: Some(complete_time),
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "DownloadResponse")[..],
        ))?;
        write_simple(writer, "Status", &self.status)?;
        match self.start_time {
            None => {}
            Some(dt) => write_simple(writer, "StartTime", &dt.to_rfc3339())?,
        }
        match self.complete_time {
            None => {}
            Some(dt) => write_simple(writer, "CompleteTime", &dt.to_rfc3339())?,
        }
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["DownloadResponse", "Status"] => {
                self.status = characters.to_string();
            }
            ["DownloadResponse", "StartTime"] => match characters.parse::<DateTime<Utc>>() {
                Ok(dt) => self.start_time = Some(dt),
                _ => {}
            },
            ["DownloadResponse", "CompleteTime"] => match characters.parse::<DateTime<Utc>>() {
                Ok(dt) => self.complete_time = Some(dt),
                _ => {}
            },
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for DownloadResponse {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        DownloadResponse::new(
            String::arbitrary(g),
            Utc.ymd(2014, 11, 28).and_hms(12, 0, 9),
            Utc.ymd(2014, 11, 29).and_hms(12, 0, 9),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.status.clone().shrink().map(|s| DownloadResponse {
            status: s,
            start_time: Some(Utc.ymd(2014, 11, 28).and_hms(12, 0, 9)),
            complete_time: Some(Utc.ymd(2014, 11, 29).and_hms(12, 0, 9)),
        }))
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Download {
    command_key: String,
    file_type: String,
    url: String,
    username: String,
    password: String,
    file_size: u32,
    target_filename: String,
    delay_seconds: u32,
    success_url: String,
    failure_url: String,
}

impl Download {
    pub fn new(
        command_key: String,
        file_type: String,
        url: String,
        username: String,
        password: String,
        file_size: u32,
        target_filename: String,
        delay_seconds: u32,
        success_url: String,
        failure_url: String,
    ) -> Self {
        Download {
            command_key: command_key,
            file_type: file_type,
            url: url,
            username: username,
            password: password,
            file_size: file_size,
            target_filename: target_filename,
            delay_seconds: delay_seconds,
            success_url: success_url,
            failure_url: failure_url,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "Download")[..],
        ))?;
        write_simple(writer, "CommandKey", &self.command_key)?;
        write_simple(writer, "FileType", &self.file_type)?;
        write_simple(writer, "URL", &self.url)?;
        write_simple(writer, "Username", &self.username)?;
        write_simple(writer, "Password", &self.password)?;
        write_simple(writer, "FileSize", &self.file_size.to_string())?;
        write_simple(writer, "TargetFileName", &self.target_filename)?;
        write_simple(writer, "DelaySeconds", &self.delay_seconds.to_string())?;
        write_simple(writer, "SuccessURL", &self.success_url)?;
        write_simple(writer, "FailureURL", &self.failure_url)?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["Download", "CommandKey"] => self.command_key = characters.to_string(),
            ["Download", "FileType"] => self.file_type = characters.to_string(),
            ["Download", "URL"] => self.url = characters.to_string(),
            ["Download", "Username"] => self.username = characters.to_string(),
            ["Download", "Password"] => self.password = characters.to_string(),
            ["Download", "FileSize"] => self.file_size = parse_to_int(characters, 0),
            ["Download", "TargetFileName"] => self.target_filename = characters.to_string(),
            ["Download", "DelaySeconds"] => self.delay_seconds = parse_to_int(characters, 0),
            ["Download", "SuccessURL"] => self.success_url = characters.to_string(),
            ["Download", "FailureURL"] => self.failure_url = characters.to_string(),
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for Download {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Download::new(
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            u32::arbitrary(g),
            String::arbitrary(g),
            u32::arbitrary(g),
            String::from(""),
            String::from(""),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.command_key.clone(),
                self.file_type.clone(),
                self.url.clone(),
                self.username.clone(),
                self.password.clone(),
                self.file_size.clone(),
                self.target_filename.clone(),
                self.delay_seconds.clone(),
            )
                .shrink()
                .map(|(c, ft, u, un, pw, fs, tf, ds)| Download {
                    command_key: c,
                    file_type: ft,
                    url: u,
                    username: un,
                    password: pw,
                    file_size: fs,
                    target_filename: tf,
                    delay_seconds: ds,
                    success_url: String::from(""),
                    failure_url: String::from(""),
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct OpResult {
    uuid: String,
    deployment_unit_ref: String,
    version: String,
    current_state: String,
    resolved: u32,
    execution_unit_ref_list: String,
    start_time: Option<DateTime<Utc>>,
    complete_time: Option<DateTime<Utc>>,
    fault: FaultStruct,
}

impl OpResult {
    pub fn new(
        uuid: String,
        deployment_unit_ref: String,
        version: String,
        current_state: String,
        resolved: u32,
        execution_unit_ref_list: String,
        start_time: DateTime<Utc>,
        complete_time: DateTime<Utc>,
        fault: FaultStruct,
    ) -> Self {
        OpResult {
            uuid: uuid,
            deployment_unit_ref: deployment_unit_ref,
            version: version,
            current_state: current_state,
            resolved: resolved,
            execution_unit_ref_list: execution_unit_ref_list,
            start_time: Some(start_time),
            complete_time: Some(complete_time),
            fault: fault,
        }
    }
}

#[cfg(test)]
impl Arbitrary for OpResult {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        OpResult::new(
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            u32::arbitrary(g),
            String::arbitrary(g),
            Utc.ymd(2014, 11, 28).and_hms(12, 0, 9),
            Utc.ymd(2014, 11, 29).and_hms(12, 0, 9),
            FaultStruct::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.uuid.clone(),
                self.deployment_unit_ref.clone(),
                self.version.clone(),
                self.current_state.clone(),
                self.resolved.clone(),
                self.execution_unit_ref_list.clone(),
                self.fault.clone(),
            )
                .shrink()
                .map(|(u, dur, v, cs, r, eurl, f)| OpResult {
                    uuid: u,
                    deployment_unit_ref: dur,
                    version: v,
                    current_state: cs,
                    resolved: r,
                    execution_unit_ref_list: eurl,
                    start_time: Some(Utc.ymd(2014, 11, 28).and_hms(12, 0, 9)),
                    complete_time: Some(Utc.ymd(2014, 11, 29).and_hms(12, 0, 9)),
                    fault: f,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct DUStateChangeCompleteResponse;

impl DUStateChangeCompleteResponse {
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        write_empty_tag(
            writer,
            &cwmp_prefix(has_cwmp, "DUStateChangeCompleteResponse")[..],
        )?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct DUStateChangeComplete {
    command_key: String,
    results: Vec<OpResult>,
}

impl DUStateChangeComplete {
    pub fn new(command_key: String, results: Vec<OpResult>) -> Self {
        DUStateChangeComplete {
            command_key: command_key,
            results: results,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "DUStateChangeComplete")[..],
        ))?;
        write_simple(writer, "CommandKey", &self.command_key)?;
        let ss = if has_cwmp {
            format!("cwmp:OpResultStruct[{}]", self.results.len())
        } else {
            format!("OpResultStruct[{}]", self.results.len())
        };

        writer.write(XmlEvent::start_element("Results").attr("SOAP-ENC:arrayType", &ss[..]))?;

        for r in self.results.iter() {
            writer.write(XmlEvent::start_element("OpResultStruct"))?;
            write_simple(writer, "UUID", &r.uuid)?;
            write_simple(writer, "DeploymentUnitRef", &r.deployment_unit_ref)?;
            write_simple(writer, "Version", &r.version)?;
            write_simple(writer, "CurrentState", &r.current_state)?;
            write_simple(writer, "Resolved", &r.resolved.to_string())?;
            write_simple(writer, "ExecutionUnitRefList", &r.execution_unit_ref_list)?;
            match r.start_time {
                None => {}
                Some(dt) => write_simple(writer, "StartTime", &dt.to_rfc3339())?,
            }
            match r.complete_time {
                None => {}
                Some(dt) => write_simple(writer, "CompleteTime", &dt.to_rfc3339())?,
            }
            write_fault(writer, &r.fault)?;
            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["DUStateChangeComplete", "Results", "OpResultStruct"] => {
                self.results.push(OpResult::default())
            }
            _ => {}
        }
    }

    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["DUStateChangeComplete", "CommandKey"] => self.command_key = characters.to_string(),
            ["DUStateChangeComplete", "Results", "OpResultStruct", key] => {
                let last = self.results.last_mut();
                match last {
                    Some(e) => match key {
                        "UUID" => e.uuid = characters.to_string(),
                        "DeploymentUnitRef" => e.deployment_unit_ref = characters.to_string(),
                        "Version" => e.version = characters.to_string(),
                        "CurrentState" => e.current_state = characters.to_string(),
                        "Resolved" => e.resolved = parse_to_int(characters, 0),
                        "ExecutionUnitRefList" => {
                            e.execution_unit_ref_list = characters.to_string()
                        }
                        "StartTime" => match characters.parse::<DateTime<Utc>>() {
                            Ok(dt) => e.start_time = Some(dt),
                            _ => {}
                        },
                        "CompleteTime" => match characters.parse::<DateTime<Utc>>() {
                            Ok(dt) => e.complete_time = Some(dt),
                            _ => {}
                        },
                        _ => {}
                    },
                    None => {}
                }
            }
            ["DUStateChangeComplete", "Results", "OpResultStruct", "Fault", "FaultStruct", key] => {
                let last = self.results.last_mut();
                match last {
                    Some(e) => match key {
                        "FaultCode" => e.fault.set_code(parse_to_int(characters, 0)),
                        "FaultString" => e.fault.set_string(characters),
                        _ => {}
                    },
                    _ => {}
                }
            }

            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for DUStateChangeComplete {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        DUStateChangeComplete::new(String::arbitrary(g), Vec::<OpResult>::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.command_key.clone(), self.results.clone())
                .shrink()
                .map(|(c, r)| DUStateChangeComplete {
                    command_key: c,
                    results: r,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct FactoryResetResponse;

impl FactoryResetResponse {
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        write_empty_tag(writer, &cwmp_prefix(has_cwmp, "FactoryResetResponse")[..])?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct FactoryReset;

impl FactoryReset {
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        write_empty_tag(writer, &cwmp_prefix(has_cwmp, "FactoryReset")[..])?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct FaultDetail {
    code: u32,
    string: String,
}

impl FaultDetail {
    pub fn new(detail_code: u32, detail_string: String) -> Self {
        FaultDetail {
            code: detail_code,
            string: detail_string,
        }
    }
}

#[cfg(test)]
impl Arbitrary for FaultDetail {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        FaultDetail::new(u32::arbitrary(g), String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.code.clone(), self.string.clone())
                .shrink()
                .map(|(c, s)| FaultDetail { code: c, string: s }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Fault {
    faultcode: String,
    faultstring: String,
    detail: FaultDetail,
}

impl Fault {
    pub fn new(faultcode: String, faultstring: String, code: u32, string: String) -> Self {
        Fault {
            faultcode: faultcode,
            faultstring: faultstring,
            detail: FaultDetail::new(code, string),
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element("SOAP-ENV:Fault"))?;
        write_simple(writer, "faultcode", &self.faultcode)?;
        write_simple(writer, "faultstring", &self.faultstring)?;
        writer.write(XmlEvent::start_element("detail"))?;
        writer.write(XmlEvent::start_element(&cwmp_prefix(has_cwmp, "Fault")[..]))?;
        write_simple(writer, "FaultCode", &self.detail.code.to_string())?;
        write_simple(writer, "FaultString", &self.detail.string.to_string())?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["Fault", "faultcode"] => {
                self.faultcode = characters.to_string();
            }
            ["Fault", "faultstring"] => {
                self.faultstring = characters.to_string();
            }
            ["Fault", "detail", "Fault", "FaultCode"] => {
                self.detail.code = parse_to_int(characters, 0);
            }
            ["Fault", "detail", "Fault", "FaultString"] => {
                self.detail.string = characters.to_string();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for Fault {
    fn arbitrary<G: Gen>(g: &mut G) -> Fault {
        Fault::new(
            String::arbitrary(g),
            String::arbitrary(g),
            u32::arbitrary(g),
            String::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.faultcode.clone(),
                self.faultstring.clone(),
                self.detail.clone(),
            )
                .shrink()
                .map(|(c, s, d)| Fault {
                    faultcode: c,
                    faultstring: s,
                    detail: d,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct AllQueuedTransfers {
    command_key: String,
    state: String,
    is_download: u8,
    file_type: String,
    file_size: u32,
    target_filename: String,
}

impl AllQueuedTransfers {
    pub fn new(
        command_key: String,
        state: String,
        is_download: u8,
        file_type: String,
        file_size: u32,
        target_filename: String,
    ) -> Self {
        AllQueuedTransfers {
            command_key: command_key,
            state: state,
            is_download: is_download,
            file_type: file_type,
            file_size: file_size,
            target_filename: target_filename,
        }
    }
}

#[cfg(test)]
impl Arbitrary for AllQueuedTransfers {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        AllQueuedTransfers::new(
            String::arbitrary(g),
            String::arbitrary(g),
            u8::arbitrary(g),
            String::arbitrary(g),
            u32::arbitrary(g),
            String::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.command_key.clone(),
                self.state.clone(),
                self.is_download.clone(),
                self.file_type.clone(),
                self.file_size.clone(),
                self.target_filename.clone(),
            )
                .shrink()
                .map(|(c, s, id, ft, fs, tf)| AllQueuedTransfers {
                    command_key: c,
                    state: s,
                    is_download: id,
                    file_type: ft,
                    file_size: fs,
                    target_filename: tf,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetAllQueuedTransfersResponse {
    transfer_list: Vec<AllQueuedTransfers>,
}

impl GetAllQueuedTransfersResponse {
    pub fn new(transfer_list: Vec<AllQueuedTransfers>) -> Self {
        GetAllQueuedTransfersResponse {
            transfer_list: transfer_list,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "GetAllQueuedTransfersResponse")[..],
        ))?;

        let ss = format!(
            "cwmp::AllQueuedTransferStruct[{}]",
            self.transfer_list.len()
        );

        writer
            .write(XmlEvent::start_element("TransferList").attr("SOAP-ENC:arrayType", &ss[..]))?;

        for t in self.transfer_list.iter() {
            writer.write(XmlEvent::start_element("AllQueuedTransferStruct"))?;
            write_simple(writer, "CommandKey", &t.command_key)?;
            write_simple(writer, "State", &t.state)?;
            write_simple(writer, "IsDownload", &t.is_download.to_string())?;
            write_simple(writer, "FileType", &t.file_type)?;
            write_simple(writer, "FileSize", &t.file_size.to_string())?;
            write_simple(writer, "TargetFileName", &t.target_filename)?;
            writer.write(XmlEvent::end_element())?;
        }

        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["GetAllQueuedTransfersResponse", "TransferList", "AllQueuedTransferStruct"] => {
                self.transfer_list.push(AllQueuedTransfers::new(
                    String::from(""),
                    String::from(""),
                    0,
                    String::from(""),
                    0,
                    String::from(""),
                ))
            }
            _ => {}
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetAllQueuedTransfersResponse", "TransferList", "AllQueuedTransferStruct", key] => {
                match self.transfer_list.last_mut() {
                    Some(last) => match key {
                        "CommandKey" => last.command_key = characters.to_string(),
                        "State" => last.state = characters.to_string(),
                        "IsDownload" => last.is_download = parse_to_int(characters, 0),
                        "FileType" => last.file_type = characters.to_string(),
                        "FileSize" => last.file_size = parse_to_int(characters, 0),
                        "TargetFileName" => last.target_filename = characters.to_string(),
                        _ => {}
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetAllQueuedTransfersResponse {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        GetAllQueuedTransfersResponse::new(Vec::<AllQueuedTransfers>::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.transfer_list
                .clone()
                .shrink()
                .map(|tl| GetAllQueuedTransfersResponse { transfer_list: tl }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetAllQueuedTransfers;

impl GetAllQueuedTransfers {
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        write_empty_tag(writer, &cwmp_prefix(has_cwmp, "GetAllQueuedTransfers")[..])?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct OptionStruct {
    option_name: String,
    voucher_sn: String,
    state: u8,
    mode: String,
    start_date: Option<DateTime<Utc>>,
    expiration_date: Option<DateTime<Utc>>,
    is_transferable: u8,
}

impl OptionStruct {
    pub fn new(
        option_name: String,
        voucher_sn: String,
        state: u8,
        mode: String,
        start_date: DateTime<Utc>,
        expiration_date: DateTime<Utc>,
        is_transferable: u8,
    ) -> Self {
        OptionStruct {
            option_name: option_name,
            voucher_sn: voucher_sn,
            state: state,
            mode: mode,
            start_date: Some(start_date),
            expiration_date: Some(expiration_date),
            is_transferable: is_transferable,
        }
    }
}

#[cfg(test)]
impl Arbitrary for OptionStruct {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        OptionStruct::new(
            String::arbitrary(g),
            String::arbitrary(g),
            u8::arbitrary(g),
            String::arbitrary(g),
            Utc.ymd(2014, 11, 28).and_hms(12, 0, 9),
            Utc.ymd(2014, 11, 29).and_hms(12, 0, 9),
            u8::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.option_name.clone(),
                self.voucher_sn.clone(),
                self.state.clone(),
                self.mode.clone(),
                self.is_transferable.clone(),
            )
                .shrink()
                .map(|(on, vsn, s, m, i)| OptionStruct {
                    option_name: on,
                    voucher_sn: vsn,
                    state: s,
                    mode: m,
                    is_transferable: i,
                    start_date: Some(Utc.ymd(2014, 11, 28).and_hms(12, 0, 9)),
                    expiration_date: Some(Utc.ymd(2014, 11, 29).and_hms(12, 0, 9)),
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetOptionsResponse {
    option_list: Vec<OptionStruct>,
}

impl GetOptionsResponse {
    pub fn new(option_list: Vec<OptionStruct>) -> Self {
        GetOptionsResponse {
            option_list: option_list,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        let ss = if has_cwmp {
            writer.write(XmlEvent::start_element("cwmp:GetOptionsResponse"))?;
            format!("cwmp:OptionStruct[{}]", self.option_list.len())
        } else {
            writer.write(XmlEvent::start_element("GetOptionsResponse"))?;
            format!("OptionStruct[{}]", self.option_list.len())
        };

        writer.write(XmlEvent::start_element("OptionList").attr("SOAP-ENC:arrayType", &ss[..]))?;

        for o in self.option_list.iter() {
            writer.write(XmlEvent::start_element("OptionStruct"))?;
            write_simple(writer, "OptionName", &o.option_name)?;
            write_simple(writer, "VoucherSN", &o.voucher_sn)?;
            write_simple(writer, "State", &o.state.to_string())?;
            write_simple(writer, "Mode", &o.mode)?;
            match o.start_date {
                None => {}
                Some(dt) => write_simple(writer, "StartDate", &dt.to_rfc3339())?,
            }
            match o.expiration_date {
                None => {}
                Some(dt) => write_simple(writer, "ExpirationDate", &dt.to_rfc3339())?,
            }
            write_simple(writer, "IsTransferable", &o.is_transferable.to_string())?;

            writer.write(XmlEvent::end_element())?;
        }

        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["GetOptionsResponse", "OptionList", "OptionStruct"] => {
                self.option_list.push(OptionStruct::new(
                    String::from(""),
                    String::from(""),
                    0,
                    String::from(""),
                    Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
                    Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
                    0,
                ))
            }
            _ => {}
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetOptionsResponse", "OptionList", "OptionStruct", key] => {
                match self.option_list.last_mut() {
                    Some(last) => match key {
                        "OptionName" => last.option_name = characters.to_string(),
                        "VoucherSN" => last.voucher_sn = characters.to_string(),
                        "State" => last.state = parse_to_int(characters, 0),
                        "Mode" => last.mode = characters.to_string(),
                        "StartDate" => match characters.parse::<DateTime<Utc>>() {
                            Ok(dt) => last.start_date = Some(dt),
                            _ => {}
                        },
                        "ExpirationDate" => match characters.parse::<DateTime<Utc>>() {
                            Ok(dt) => last.expiration_date = Some(dt),
                            _ => {}
                        },
                        "IsTransferable" => last.is_transferable = parse_to_int(characters, 0),
                        _ => {}
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetOptionsResponse {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        GetOptionsResponse::new(Vec::<OptionStruct>::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.option_list
                .clone()
                .shrink()
                .map(|ol| GetOptionsResponse { option_list: ol }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetOptions {
    option_name: String,
}

impl GetOptions {
    pub fn new(option_name: String) -> Self {
        GetOptions {
            option_name: option_name,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "GetOptions")[..],
        ))?;
        write_simple(writer, "OptionName", &self.option_name)?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetOptions", "OptionName"] => self.option_name = characters.to_string(),
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetOptions {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        GetOptions::new(String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.option_name
                .clone()
                .shrink()
                .map(|on| GetOptions { option_name: on }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetParameterAttributes {
    pub parameternames: Vec<String>,
}

impl GetParameterAttributes {
    pub fn new(parameternames: Vec<String>) -> Self {
        GetParameterAttributes {
            parameternames: parameternames,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "GetParameterAttributes")[..],
        ))?;
        writer.write(XmlEvent::start_element("ParameterNames"))?;
        for p in self.parameternames.iter() {
            write_simple(writer, "string", &p)?;
        }
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["GetParameterAttributes", "ParameterNames", "string"] => {
                self.parameternames.push(String::from(""));
            }
            _ => {}
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetParameterAttributes", "ParameterNames", "string"] => {
                let last = self.parameternames.last_mut();
                match last {
                    Some(l) => *l = characters.to_string(),
                    None => {}
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetParameterAttributes {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        GetParameterAttributes::new(Vec::<String>::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.parameternames
                .clone()
                .shrink()
                .map(|pn| GetParameterAttributes { parameternames: pn }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ParameterAttribute {
    name: String,
    notification: String,
    accesslist: Vec<String>,
}
impl ParameterAttribute {
    pub fn new(name: String, notification: String, accesslist: Vec<String>) -> Self {
        ParameterAttribute {
            name: name,
            notification: notification,
            accesslist: accesslist,
        }
    }
}

#[cfg(test)]
impl Arbitrary for ParameterAttribute {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        ParameterAttribute::new(
            String::arbitrary(g),
            String::arbitrary(g),
            Vec::<String>::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.name.clone(),
                self.notification.clone(),
                self.accesslist.clone(),
            )
                .shrink()
                .map(|(n, no, a)| ParameterAttribute {
                    name: n,
                    notification: no,
                    accesslist: a,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetParameterAttributesResponse {
    parameters: Vec<ParameterAttribute>,
}

impl GetParameterAttributesResponse {
    pub fn new(parameters: Vec<ParameterAttribute>) -> Self {
        GetParameterAttributesResponse {
            parameters: parameters,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        let ss = if has_cwmp {
            writer.write(XmlEvent::start_element(
                "cwmp:GetParameterAttributesResponse",
            ))?;
            format!("cwmp:ParameterAttributeStruct[{}]", self.parameters.len())
        } else {
            writer.write(XmlEvent::start_element("GetParameterAttributesResponse"))?;
            format!("ParameterAttributeStruct[{}]", self.parameters.len())
        };

        writer
            .write(XmlEvent::start_element("ParameterList").attr("SOAP-ENC:arrayType", &ss[..]))?;

        for p in self.parameters.iter() {
            writer.write(XmlEvent::start_element("ParameterAttributeStruct"))?;
            write_simple(writer, "Name", &p.name)?;
            write_simple(writer, "Notification", &p.notification)?;
            let als = format!("xsd:string[{}]", p.accesslist.len());
            writer.write(
                XmlEvent::start_element("AccessList").attr("SOAP-ENC:arrayType", &als[..]),
            )?;

            for a in p.accesslist.iter() {
                write_simple(writer, "string", &a)?;
            }

            writer.write(XmlEvent::end_element())?;
            writer.write(XmlEvent::end_element())?;
        }

        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["GetParameterAttributesResponse", "ParameterList", "ParameterAttributeStruct"] => {
                self.parameters.push(ParameterAttribute::new(
                    String::from(""),
                    String::from(""),
                    vec![],
                ))
            }
            ["GetParameterAttributesResponse", "ParameterList", "ParameterAttributeStruct", "AccessList", "string"] => {
                if let Some(e) = self.parameters.last_mut() {
                    e.accesslist.push(String::from(""));
                }
            }
            _ => {}
        }
    }

    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetParameterAttributesResponse", "ParameterList", "ParameterAttributeStruct", "Name"] => {
                if let Some(e) = self.parameters.last_mut() {
                    e.name = characters.to_string();
                }
            }
            ["GetParameterAttributesResponse", "ParameterList", "ParameterAttributeStruct", "Notification"] => {
                if let Some(e) = self.parameters.last_mut() {
                    e.notification = characters.to_string();
                }
            }
            ["GetParameterAttributesResponse", "ParameterList", "ParameterAttributeStruct", "AccessList", "string"] => {
                if let Some(e) = self.parameters.last_mut() {
                    if let Some(last) = e.accesslist.last_mut() {
                        *last = characters.to_string();
                    }
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetParameterAttributesResponse {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        GetParameterAttributesResponse::new(Vec::<ParameterAttribute>::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.parameters
                .clone()
                .shrink()
                .map(|p| GetParameterAttributesResponse { parameters: p }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ParameterInfoStruct {
    name: String,
    writable: u8,
}

impl ParameterInfoStruct {
    pub fn new(name: String, writable: u8) -> Self {
        ParameterInfoStruct {
            name: name,
            writable: writable,
        }
    }
}

#[cfg(test)]
impl Arbitrary for ParameterInfoStruct {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        ParameterInfoStruct::new(String::arbitrary(g), u8::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.name.clone(), self.writable.clone())
                .shrink()
                .map(|(n, w)| ParameterInfoStruct {
                    name: n,
                    writable: w,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetParameterNamesResponse {
    parameter_list: Vec<ParameterInfoStruct>,
}

impl GetParameterNamesResponse {
    pub fn new(parameter_list: Vec<ParameterInfoStruct>) -> Self {
        GetParameterNamesResponse {
            parameter_list: parameter_list,
        }
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["GetParameterNamesResponse", "ParameterList", "ParameterInfoStruct"] => {
                self.parameter_list.push(ParameterInfoStruct::default())
            }
            _ => {}
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        let ss = if has_cwmp {
            writer.write(XmlEvent::start_element("cwmp:GetParameterNamesResponse"))?;
            format!("cwmp:ParameterInfoStruct[{}]", self.parameter_list.len())
        } else {
            writer.write(XmlEvent::start_element("GetParameterNamesResponse"))?;
            format!("ParameterInfoStruct[{}]", self.parameter_list.len())
        };

        writer
            .write(XmlEvent::start_element("ParameterList").attr("SOAP-ENC:arrayType", &ss[..]))?;

        for p in self.parameter_list.iter() {
            writer.write(XmlEvent::start_element("ParameterInfoStruct"))?;
            write_simple(writer, "Name", &p.name)?;
            write_simple(writer, "Writable", &p.writable.to_string())?;
            writer.write(XmlEvent::end_element())?;
        }

        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetParameterNamesResponse", "ParameterList", "ParameterInfoStruct", "Name"] => {
                let last = self.parameter_list.last_mut();
                match last {
                    Some(e) => e.name = characters.to_string(),
                    None => {}
                }
            }
            ["GetParameterNamesResponse", "ParameterList", "ParameterInfoStruct", "Writable"] => {
                let last = self.parameter_list.last_mut();
                match last {
                    Some(e) => e.writable = parse_to_int(characters, 0),
                    None => {}
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetParameterNamesResponse {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        GetParameterNamesResponse::new(Vec::<ParameterInfoStruct>::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.parameter_list
                .clone()
                .shrink()
                .map(|pl| GetParameterNamesResponse { parameter_list: pl }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetParameterNames {
    parameter_path: String,
    next_level: u32,
}
impl GetParameterNames {
    pub fn new(parameter_path: String, next_level: u32) -> Self {
        GetParameterNames {
            parameter_path: parameter_path,
            next_level: next_level,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "GetParameterNames")[..],
        ))?;
        write_simple(writer, "ParameterPath", &self.parameter_path)?;
        write_simple(writer, "NextLevel", &self.next_level.to_string())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetParameterNames", "ParameterPath"] => self.parameter_path = characters.to_string(),
            ["GetParameterNames", "NextLevel"] => self.next_level = parse_to_int(characters, 0),
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetParameterNames {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        GetParameterNames::new(String::arbitrary(g), u32::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.parameter_path.clone(), self.next_level.clone())
                .shrink()
                .map(|(pp, nl)| GetParameterNames {
                    parameter_path: pp,
                    next_level: nl,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ParameterValue {
    name: String,
    r#type: String,
    value: String,
}

impl ParameterValue {
    pub fn new(name: String, param_type: String, value: String) -> Self {
        ParameterValue {
            name: name,
            r#type: param_type,
            value: value,
        }
    }
}

#[cfg(test)]
impl Arbitrary for ParameterValue {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        ParameterValue::new(
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.name.clone(), self.r#type.clone(), self.value.clone())
                .shrink()
                .map(|(n, t, v)| ParameterValue {
                    name: n,
                    r#type: t,
                    value: v,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetParameterValues {
    parameternames: Vec<String>,
}

impl GetParameterValues {
    pub fn new(parameternames: Vec<String>) -> Self {
        GetParameterValues {
            parameternames: parameternames,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "GetParameterValues")[..],
        ))?;
        writer.write(XmlEvent::start_element("ParameterNames"))?;
        for p in self.parameternames.iter() {
            write_simple(writer, "string", &p)?;
        }
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["GetParameterValues", "ParameterNames", "string"] => {
                self.parameternames.push(String::from(""));
            }
            _ => {}
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            // no hit on this if we have <string></string>
            ["GetParameterValues", "ParameterNames", "string"] => {
                let last = self.parameternames.last_mut();
                match last {
                    Some(l) => *l = characters.to_string(),
                    None => {}
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetParameterValues {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        GetParameterValues::new(Vec::<String>::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.parameternames
                .clone()
                .shrink()
                .map(|pn| GetParameterValues { parameternames: pn }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetParameterValuesResponse {
    parameters: Vec<ParameterValue>,
}

impl GetParameterValuesResponse {
    pub fn new(parameters: Vec<ParameterValue>) -> Self {
        GetParameterValuesResponse {
            parameters: parameters,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        let ss = if has_cwmp {
            writer.write(XmlEvent::start_element("cwmp:GetParameterValuesResponse"))?;
            format!("cwmp:ParameterValueStruct[{}]", self.parameters.len())
        } else {
            writer.write(XmlEvent::start_element("GetParameterValuesResponse"))?;
            format!("ParameterValueStruct[{}]", self.parameters.len())
        };
        writer.write(
            XmlEvent::start_element("ParameterList")
                .attr("xsi:type", "SOAP-ENC:Array")
                .attr("SOAP-ENC:arrayType", &ss[..]),
        )?;

        for p in self.parameters.iter() {
            writer.write(XmlEvent::start_element("ParameterValueStruct"))?;
            write_simple(writer, "Name", &p.name)?;
            writer.write(XmlEvent::start_element("Value").attr("xsi:type", &p.r#type[..]))?;
            writer.write(&p.value[..])?;
            writer.write(XmlEvent::end_element())?;

            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["GetParameterValuesResponse", "ParameterList", "ParameterValueStruct"] => {
                self.parameters.push(ParameterValue::new(
                    String::from(""),
                    String::from(""),
                    String::from(""),
                ))
            }
            ["GetParameterValuesResponse", "ParameterList", "ParameterValueStruct", "Value"] => {
                // use the type attribute
                let last = self.parameters.last_mut();
                match last {
                    Some(e) => e.r#type = extract_attribute(attributes, "type"),
                    None => {}
                }
            }
            _ => {}
        }
    }

    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetParameterValuesResponse", "ParameterList", "ParameterValueStruct", "Name"] => {
                let last = self.parameters.last_mut();
                match last {
                    Some(e) => e.name = characters.to_string(),
                    None => {}
                }
            }
            ["GetParameterValuesResponse", "ParameterList", "ParameterValueStruct", "Value"] => {
                let last = self.parameters.last_mut();
                match last {
                    Some(e) => e.value = characters.to_string(),
                    None => {}
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetParameterValuesResponse {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        GetParameterValuesResponse::new(Vec::<ParameterValue>::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.parameters
                .clone()
                .shrink()
                .map(|p| GetParameterValuesResponse { parameters: p }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct QueuedTransferStruct {
    command_key: Option<String>,
    state: Option<String>,
}

impl QueuedTransferStruct {
    pub fn new(command_key: Option<String>, state: Option<String>) -> Self {
        QueuedTransferStruct {
            command_key: command_key,
            state: state,
        }
    }
}

#[cfg(test)]
impl Arbitrary for QueuedTransferStruct {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        QueuedTransferStruct::new(
            Option::<String>::arbitrary(g),
            Option::<String>::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.command_key.clone(), self.state.clone())
                .shrink()
                .map(|(c, s)| QueuedTransferStruct {
                    command_key: c,
                    state: s,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetQueuedTransfersResponse {
    transfer_list: Vec<QueuedTransferStruct>,
}

impl GetQueuedTransfersResponse {
    pub fn new(transfer_list: Vec<QueuedTransferStruct>) -> Self {
        GetQueuedTransfersResponse {
            transfer_list: transfer_list,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        let ss = if has_cwmp {
            writer.write(XmlEvent::start_element("cwmp:GetQueuedTransfersResponse"))?;
            format!("cwmp:QueuedTransferStruct[{}]", self.transfer_list.len())
        } else {
            writer.write(XmlEvent::start_element("GetQueuedTransfersResponse"))?;
            format!("QueuedTransferStruct[{}]", self.transfer_list.len())
        };

        writer
            .write(XmlEvent::start_element("TransferList").attr("SOAP-ENC:arrayType", &ss[..]))?;

        for p in self.transfer_list.iter() {
            writer.write(XmlEvent::start_element("QueuedTransferStruct"))?;
            match &p.command_key {
                Some(ck) => write_simple(writer, "CommandKey", &ck)?,
                None => {}
            }
            match &p.state {
                Some(s) => write_simple(writer, "State", &s)?,
                None => {}
            }
            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["GetQueuedTransfersResponse", "TransferList", "QueuedTransferStruct"] => {
                self.transfer_list.push(QueuedTransferStruct::default())
            }
            ["GetQueuedTransfersResponse", "TransferList", "QueuedTransferStruct", "CommandKey"] => {
                let last = self.transfer_list.last_mut();
                match last {
                    Some(l) => l.command_key = Some("".to_string()),
                    None => {}
                }
            }
            ["GetQueuedTransfersResponse", "TransferList", "QueuedTransferStruct", "State"] => {
                let last = self.transfer_list.last_mut();
                match last {
                    Some(l) => l.state = Some("".to_string()),
                    None => {}
                }
            }
            _ => {}
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetQueuedTransfersResponse", "TransferList", "QueuedTransferStruct", key] => {
                let last = self.transfer_list.last_mut();
                match last {
                    Some(e) => match key {
                        "CommandKey" => e.command_key = Some(characters.to_string()),
                        "State" => e.state = Some(characters.to_string()),
                        _ => {}
                    },
                    None => {}
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetQueuedTransfersResponse {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        GetQueuedTransfersResponse::new(Vec::<QueuedTransferStruct>::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.transfer_list
                .clone()
                .shrink()
                .map(|t| GetQueuedTransfersResponse { transfer_list: t }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetQueuedTransfers {}

impl GetQueuedTransfers {
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        write_empty_tag(writer, &cwmp_prefix(has_cwmp, "GetQueuedTransfers"))?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetRPCMethodsResponse {
    method_list: Vec<String>,
}

impl GetRPCMethodsResponse {
    pub fn new(method_list: Vec<String>) -> Self {
        GetRPCMethodsResponse {
            method_list: method_list,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "GetRPCMethodsResponse")[..],
        ))?;
        let ss = format!("xsd:string[{}]", self.method_list.len());

        writer.write(XmlEvent::start_element("MethodList").attr("SOAP-ENC:arrayType", &ss[..]))?;

        for p in self.method_list.iter() {
            write_simple(writer, "string", &p)?;
        }
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["GetRPCMethodsResponse", "MethodList", "string"] => {
                self.method_list.push(String::from(""));
            }
            _ => {}
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["GetRPCMethodsResponse", "MethodList", "string"] => {
                let last = self.method_list.last_mut();
                match last {
                    Some(l) => *l = characters.to_string(),
                    None => {}
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetRPCMethodsResponse {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        GetRPCMethodsResponse::new(Vec::<String>::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.method_list
                .clone()
                .shrink()
                .map(|ml| GetRPCMethodsResponse { method_list: ml }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetRPCMethods {}

impl GetRPCMethods {
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        write_empty_tag(writer, &cwmp_prefix(has_cwmp, "GetRPCMethods")[..])?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct InformResponse {
    max_envelopes: u16,
}

impl InformResponse {
    pub fn new(max_envelopes: u16) -> Self {
        InformResponse {
            max_envelopes: max_envelopes,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "InformResponse")[..],
        ))?;
        write_simple(writer, "MaxEnvelopes", &self.max_envelopes.to_string())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["InformResponse", "MaxEnvelopes"] => {
                self.max_envelopes = parse_to_int(characters, 1);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for InformResponse {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        InformResponse::new(u16::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.max_envelopes
                .clone()
                .shrink()
                .map(|me| InformResponse { max_envelopes: me }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct DeviceId {
    manufacturer: String,
    oui: String,
    product_class: String,
    serial_number: String,
}
impl DeviceId {
    pub fn new(
        manufacturer: String,
        oui: String,
        product_class: String,
        serial_number: String,
    ) -> Self {
        DeviceId {
            manufacturer: manufacturer,
            oui: oui,
            product_class: product_class,
            serial_number: serial_number,
        }
    }
}

#[cfg(test)]
impl Arbitrary for DeviceId {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        DeviceId::new(
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.manufacturer.clone(),
                self.oui.clone(),
                self.product_class.clone(),
                self.serial_number.clone(),
            )
                .shrink()
                .map(|(m, o, p, s)| DeviceId {
                    manufacturer: m,
                    oui: o,
                    product_class: p,
                    serial_number: s,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct EventStruct {
    event_code: String,
    command_key: String,
}

impl EventStruct {
    pub fn new(event_code: String, command_key: String) -> Self {
        EventStruct {
            event_code: event_code,
            command_key: command_key,
        }
    }
}

#[cfg(test)]
impl Arbitrary for EventStruct {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        EventStruct::new(String::arbitrary(g), String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.event_code.clone(), self.command_key.clone())
                .shrink()
                .map(|(e, c)| EventStruct {
                    event_code: e,
                    command_key: c,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Inform {
    device_id: DeviceId,
    event: Vec<EventStruct>,
    max_envelopes: u32,
    current_time: Option<DateTime<Utc>>,
    retry_count: u32,
    parameter_list: Vec<ParameterValue>,
}

impl Inform {
    pub fn new(
        device_id: DeviceId,
        event: Vec<EventStruct>,
        max_envelopes: u32,
        current_time: DateTime<Utc>,
        retry_count: u32,
        parameter_list: Vec<ParameterValue>,
    ) -> Self {
        Inform {
            device_id: device_id,
            event: event,
            max_envelopes: max_envelopes,
            current_time: Some(current_time),
            retry_count: retry_count,
            parameter_list: parameter_list,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "Inform")[..],
        ))?;
        writer.write(XmlEvent::start_element("DeviceId"))?;
        write_simple(writer, "Manufacturer", &self.device_id.manufacturer)?;
        write_simple(writer, "OUI", &self.device_id.oui)?;
        write_simple(writer, "ProductClass", &self.device_id.product_class)?;
        write_simple(writer, "SerialNumber", &self.device_id.serial_number)?;
        writer.write(XmlEvent::end_element())?;

        let ss = format!("cwmp:EventStruct[{}]", self.event.len());

        writer.write(XmlEvent::start_element("Event").attr("SOAP-ENC:arrayType", &ss[..]))?;

        for e in self.event.iter() {
            writer.write(XmlEvent::start_element("EventStruct"))?;
            write_simple(writer, "EventCode", &e.event_code)?;
            write_simple(writer, "CommandKey", &e.command_key)?;
            writer.write(XmlEvent::end_element())?;
        }
        // Event
        writer.write(XmlEvent::end_element())?;

        write_simple(writer, "MaxEnvelopes", &self.max_envelopes.to_string())?;
        match self.current_time {
            None => {}
            Some(dt) => write_simple(writer, "CurrentTime", &dt.to_rfc3339())?,
        }
        write_simple(writer, "RetryCount", &self.retry_count.to_string())?;

        let pls = format!("cwmp:ParameterValueStruct[{}]", self.parameter_list.len());
        writer
            .write(XmlEvent::start_element("ParameterList").attr("SOAP-ENC:arrayType", &pls[..]))?;

        for p in self.parameter_list.iter() {
            writer.write(XmlEvent::start_element("ParameterValueStruct"))?;
            write_simple(writer, "Name", &p.name)?;
            writer.write(XmlEvent::start_element("Value").attr("xsi:type", &p.r#type[..]))?;
            writer.write(&p.value[..])?;
            writer.write(XmlEvent::end_element())?; // Value
            writer.write(XmlEvent::end_element())?; // ParameterValueStruct
        }

        // ParameterList
        writer.write(XmlEvent::end_element())?;

        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["Inform", "Event", "EventStruct"] => self.event.push(EventStruct::default()),
            ["Inform", "ParameterList", "ParameterValueStruct"] => {
                self.parameter_list.push(ParameterValue::default())
            }
            ["Inform", "ParameterList", "ParameterValueStruct", "Value"] => {
                // use the type attribute
                let last = self.parameter_list.last_mut();
                match last {
                    Some(e) => e.r#type = extract_attribute(attributes, "type"),
                    None => {}
                }
            }
            _ => {}
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["Inform", "DeviceId", "Manufacturer"] => {
                self.device_id.manufacturer = characters.to_string()
            }
            ["Inform", "DeviceId", "OUI"] => self.device_id.oui = characters.to_string(),
            ["Inform", "DeviceId", "ProductClass"] => {
                self.device_id.product_class = characters.to_string()
            }
            ["Inform", "DeviceId", "SerialNumber"] => {
                self.device_id.serial_number = characters.to_string()
            }
            ["Inform", "Event", "EventStruct", key] => {
                let event = self.event.last_mut();
                match event {
                    Some(e) => match key {
                        "EventCode" => e.event_code = characters.to_string(),
                        "CommandKey" => e.command_key = characters.to_string(),
                        _ => {}
                    },
                    None => {}
                }
            }
            ["Inform", "MaxEnvelopes"] => self.max_envelopes = parse_to_int(characters, 0),
            ["Inform", "RetryCount"] => self.retry_count = parse_to_int(characters, 0),
            ["Inform", "CurrentTime"] => match characters.parse::<DateTime<Utc>>() {
                Ok(dt) => self.current_time = Some(dt),
                _ => {}
            },
            ["Inform", "ParameterList", "ParameterValueStruct", "Name"] => {
                let param = self.parameter_list.last_mut();
                match param {
                    Some(p) => p.name = characters.to_string(),
                    None => {}
                }
            }
            ["Inform", "ParameterList", "ParameterValueStruct", "Value"] => {
                let param = self.parameter_list.last_mut();
                match param {
                    Some(p) => p.value = characters.to_string(),
                    None => {}
                }
            }

            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for Inform {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Inform::new(
            DeviceId::arbitrary(g),
            Vec::<EventStruct>::arbitrary(g),
            u32::arbitrary(g),
            Utc.ymd(2014, 11, 28).and_hms(12, 0, 9),
            u32::arbitrary(g),
            Vec::<ParameterValue>::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.device_id.clone(),
                self.event.clone(),
                self.max_envelopes.clone(),
                self.retry_count.clone(),
                self.parameter_list.clone(),
            )
                .shrink()
                .map(|(d, e, m, r, p)| Inform {
                    device_id: d,
                    event: e,
                    max_envelopes: m,
                    current_time: Some(Utc.ymd(2014, 11, 28).and_hms(12, 0, 9)),
                    retry_count: r,
                    parameter_list: p,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct KickedResponse {
    next_url: String,
}

impl KickedResponse {
    pub fn new(next_url: String) -> Self {
        KickedResponse { next_url: next_url }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "KickedResponse")[..],
        ))?;
        write_simple(writer, "NextURL", &self.next_url)?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["KickedResponse", "NextURL"] => {
                self.next_url = characters.to_string();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for KickedResponse {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        KickedResponse::new(String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.next_url
                .clone()
                .shrink()
                .map(|n| KickedResponse { next_url: n }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Kicked {
    command: String,
    referer: String,
    arg: String,
    next: String,
}

impl Kicked {
    pub fn new(command: String, referer: String, arg: String, next: String) -> Self {
        Kicked {
            command: command,
            referer: referer,
            arg: arg,
            next: next,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "Kicked")[..],
        ))?;
        write_simple(writer, "Command", &self.command)?;
        write_simple(writer, "Referer", &self.referer)?;
        write_simple(writer, "Arg", &self.arg)?;
        write_simple(writer, "Next", &self.next)?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["Kicked", "Command"] => {
                self.command = characters.to_string();
            }
            ["Kicked", "Referer"] => {
                self.referer = characters.to_string();
            }
            ["Kicked", "Arg"] => {
                self.arg = characters.to_string();
            }
            ["Kicked", "Next"] => {
                self.next = characters.to_string();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for Kicked {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Kicked::new(
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.command.clone(),
                self.referer.clone(),
                self.arg.clone(),
                self.next.clone(),
            )
                .shrink()
                .map(|(c, r, a, n)| Kicked {
                    command: c,
                    referer: r,
                    arg: a,
                    next: n,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct RebootResponse {}

impl RebootResponse {
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        write_empty_tag(writer, &cwmp_prefix(has_cwmp, "RebootResponse")[..])?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Reboot {
    command_key: String,
}

impl Reboot {
    pub fn new(command_key: String) -> Self {
        Reboot {
            command_key: command_key,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "Reboot")[..],
        ))?;
        write_simple(writer, "CommandKey", &self.command_key)?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["Reboot", "CommandKey"] => {
                self.command_key = characters.to_string();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for Reboot {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Reboot::new(String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.command_key
                .clone()
                .shrink()
                .map(|c| Reboot { command_key: c }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ArgStruct {
    name: String,
    value: String,
}

impl ArgStruct {
    pub fn new(name: String, value: String) -> Self {
        ArgStruct {
            name: name,
            value: value,
        }
    }
}

#[cfg(test)]
impl Arbitrary for ArgStruct {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        ArgStruct::new(String::arbitrary(g), String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.name.clone(), self.value.clone())
                .shrink()
                .map(|(n, v)| ArgStruct { name: n, value: v }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct RequestDownload {
    file_type: String,
    file_type_arg: Vec<ArgStruct>,
}

impl RequestDownload {
    pub fn new(file_type: String, file_type_arg: Vec<ArgStruct>) -> Self {
        RequestDownload {
            file_type: file_type,
            file_type_arg: file_type_arg,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "RequestDownload")[..],
        ))?;
        write_simple(writer, "FileType", &self.file_type)?;
        let argss = format!("cwmp:ArgStruct[{}]", self.file_type_arg.len());
        writer
            .write(XmlEvent::start_element("FileTypeArg").attr("SOAP-ENC:arrayType", &argss[..]))?;

        for a in self.file_type_arg.iter() {
            writer.write(XmlEvent::start_element("ArgStruct"))?;
            write_simple(writer, "Name", &a.name)?;
            write_simple(writer, "Value", &a.value)?;
            writer.write(XmlEvent::end_element())?;
        }

        // FileTypeArg
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["RequestDownload", "FileTypeArg", "ArgStruct"] => {
                self.file_type_arg.push(ArgStruct::default())
            }
            // // in case of blanks, where characters wont fire
            // ["RequestDownload", "FileTypeArg", "ArgStruct", "Name"] => {
            //     let last = self.file_type_arg.last_mut();
            //     match last {
            //         Some(l) => l.name = "".to_string(),
            //         None => {}
            //     }
            // }
            // ["RequestDownload", "FileTypeArg", "ArgStruct", "Value"] => {
            //     let last = self.file_type_arg.last_mut();
            //     match last {
            //         Some(l) => l.value = "".to_string(),
            //         None => {}
            //     }
            // }
            _ => {}
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["RequestDownload", "FileType"] => {
                self.file_type = characters.to_string();
            }
            ["RequestDownload", "FileTypeArg", "ArgStruct", key] => {
                let last = self.file_type_arg.last_mut();
                match last {
                    Some(e) => match key {
                        "Name" => e.name = characters.to_string(),
                        "Value" => e.value = characters.to_string(),
                        _ => {}
                    },
                    _ => {}
                }
            }

            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for RequestDownload {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        RequestDownload::new(String::arbitrary(g), Vec::<ArgStruct>::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.file_type.clone(), self.file_type_arg.clone())
                .shrink()
                .map(|(ft, fta)| RequestDownload {
                    file_type: ft,
                    file_type_arg: fta,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct RequestDownloadResponse {}

impl RequestDownloadResponse {
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        write_empty_tag(
            writer,
            &cwmp_prefix(has_cwmp, "RequestDownloadResponse")[..],
        )?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ScheduleDownloadResponse {}

impl ScheduleDownloadResponse {
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        write_empty_tag(
            writer,
            &cwmp_prefix(has_cwmp, "ScheduleDownloadResponse")[..],
        )?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct TimeWindow {
    window_start: u32,
    window_end: u32,
    window_mode: String,
    user_message: String,
    max_retries: i32,
}
impl TimeWindow {
    pub fn new(
        window_start: u32,
        window_end: u32,
        window_mode: String,
        user_message: String,
        max_retries: i32,
    ) -> Self {
        TimeWindow {
            window_start: window_start,
            window_end: window_end,
            window_mode: window_mode,
            user_message: user_message,
            max_retries: max_retries,
        }
    }
}

#[cfg(test)]
impl Arbitrary for TimeWindow {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        TimeWindow::new(
            u32::arbitrary(g),
            u32::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            i32::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.window_start.clone(),
                self.window_end.clone(),
                self.window_mode.clone(),
                self.user_message.clone(),
                self.max_retries.clone(),
            )
                .shrink()
                .map(|(ws, we, wm, um, mr)| TimeWindow {
                    window_start: ws,
                    window_end: we,
                    window_mode: wm,
                    user_message: um,
                    max_retries: mr,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ScheduleDownload {
    command_key: String,
    file_type: String,
    url: String,
    username: String,
    password: String,
    file_size: u32,
    target_filename: String,
    timewindow_list: Vec<TimeWindow>,
}

impl ScheduleDownload {
    pub fn new(
        command_key: String,
        file_type: String,
        url: String,
        username: String,
        password: String,
        file_size: u32,
        target_filename: String,
        timewindow_list: Vec<TimeWindow>,
    ) -> Self {
        ScheduleDownload {
            command_key: command_key,
            file_type: file_type,
            url: url,
            username: username,
            password: password,
            file_size: file_size,
            target_filename: target_filename,
            timewindow_list: timewindow_list,
        }
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["ScheduleDownload", "TimeWindowList", "TimeWindowStruct"] => {
                self.timewindow_list.push(TimeWindow::default())
            }
            _ => {}
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "ScheduleDownload")[..],
        ))?;
        write_simple(writer, "CommandKey", &self.command_key)?;
        write_simple(writer, "FileType", &self.file_type)?;
        write_simple(writer, "URL", &self.url)?;
        write_simple(writer, "Username", &self.username)?;
        write_simple(writer, "Password", &self.password)?;
        write_simple(writer, "FileSize", &self.file_size.to_string())?;
        write_simple(writer, "TargetFileName", &self.target_filename)?;
        let ts = format!("cwmp:TimeWindowStruct[{}]", self.timewindow_list.len());
        writer
            .write(XmlEvent::start_element("TimeWindowList").attr("SOAP-ENC:arrayType", &ts[..]))?;

        for t in self.timewindow_list.iter() {
            writer.write(XmlEvent::start_element("TimeWindowStruct"))?;
            write_simple(writer, "WindowStart", &t.window_start.to_string())?;
            write_simple(writer, "WindowEnd", &t.window_end.to_string())?;
            write_simple(writer, "WindowMode", &t.window_mode)?;
            write_simple(writer, "UserMessage", &t.user_message)?;
            write_simple(writer, "MaxRetries", &t.max_retries.to_string())?;
            writer.write(XmlEvent::end_element())?;
        }

        // TimeWindownList
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["ScheduleDownload", "CommandKey"] => {
                self.command_key = characters.to_string();
            }
            ["ScheduleDownload", "FileType"] => {
                self.file_type = characters.to_string();
            }
            ["ScheduleDownload", "URL"] => {
                self.url = characters.to_string();
            }
            ["ScheduleDownload", "Username"] => {
                self.username = characters.to_string();
            }
            ["ScheduleDownload", "Password"] => {
                self.password = characters.to_string();
            }
            ["ScheduleDownload", "FileSize"] => {
                self.file_size = parse_to_int(characters, 0);
            }
            ["ScheduleDownload", "TargetFileName"] => {
                self.target_filename = characters.to_string();
            }
            ["ScheduleDownload", "TimeWindowList", "TimeWindowStruct", key] => {
                let last = self.timewindow_list.last_mut();
                match last {
                    Some(e) => match key {
                        "WindowStart" => e.window_start = parse_to_int(characters, 0),
                        "WindowEnd" => e.window_end = parse_to_int(characters, 0),
                        "WindowMode" => e.window_mode = characters.to_string(),
                        "UserMessage" => e.user_message = characters.to_string(),
                        "MaxRetries" => e.max_retries = parse_to_int(characters, 0),
                        _ => {}
                    },
                    _ => {}
                }
            }

            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for ScheduleDownload {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        ScheduleDownload::new(
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            u32::arbitrary(g),
            String::arbitrary(g),
            Vec::<TimeWindow>::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.command_key.clone(),
                self.file_type.clone(),
                self.url.clone(),
                self.username.clone(),
                self.password.clone(),
                self.file_size.clone(),
                self.target_filename.clone(),
                self.timewindow_list.clone(),
            )
                .shrink()
                .map(|(c, f, u, un, pw, fs, tf, tl)| ScheduleDownload {
                    command_key: c,
                    file_type: f,
                    url: u,
                    username: un,
                    password: pw,
                    file_size: fs,
                    target_filename: tf,
                    timewindow_list: tl,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ScheduleInformResponse {}

impl ScheduleInformResponse {
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        write_empty_tag(writer, &cwmp_prefix(has_cwmp, "ScheduleInformResponse")[..])?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ScheduleInform {
    delay_seconds: u32,
    command_key: String,
}

impl ScheduleInform {
    pub fn new(delay_seconds: u32, command_key: String) -> Self {
        ScheduleInform {
            delay_seconds: delay_seconds,
            command_key: command_key,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "ScheduleInform")[..],
        ))?;
        write_simple(writer, "DelaySeconds", &self.delay_seconds.to_string())?;
        write_simple(writer, "CommandKey", &self.command_key)?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["ScheduleInform", "DelaySeconds"] => {
                self.delay_seconds = parse_to_int(characters, 0);
            }
            ["ScheduleInform", "CommandKey"] => {
                self.command_key = characters.to_string();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for ScheduleInform {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        ScheduleInform::new(u32::arbitrary(g), String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.delay_seconds.clone(), self.command_key.clone())
                .shrink()
                .map(|(d, c)| ScheduleInform {
                    delay_seconds: d,
                    command_key: c,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SetParameterAttributesResponse {}

impl SetParameterAttributesResponse {
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        write_empty_tag(
            writer,
            &cwmp_prefix(has_cwmp, "SetParameterAttributesResponse")[..],
        )?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SetParameterAttributesStruct {
    name: String,
    notification_change: u8,
    notification: u8,
    access_list_change: u8,
    access_list: Vec<String>,
}

impl SetParameterAttributesStruct {
    pub fn new(
        name: String,
        notification_change: u8,
        notification: u8,
        access_list_change: u8,
        access_list: Vec<String>,
    ) -> Self {
        SetParameterAttributesStruct {
            name: name,
            notification_change: notification_change,
            notification: notification,
            access_list_change: access_list_change,
            access_list: access_list,
        }
    }
}

#[cfg(test)]
impl Arbitrary for SetParameterAttributesStruct {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        SetParameterAttributesStruct::new(
            String::arbitrary(g),
            u8::arbitrary(g),
            u8::arbitrary(g),
            u8::arbitrary(g),
            Vec::<String>::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.name.clone(),
                self.notification_change.clone(),
                self.notification.clone(),
                self.access_list_change.clone(),
                self.access_list.clone(),
            )
                .shrink()
                .map(|(name, nc, n, alc, al)| SetParameterAttributesStruct {
                    name: name,
                    notification_change: nc,
                    notification: n,
                    access_list_change: alc,
                    access_list: al,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SetParameterAttributes {
    parameter_list: Vec<SetParameterAttributesStruct>,
}
impl SetParameterAttributes {
    pub fn new(parameter_list: Vec<SetParameterAttributesStruct>) -> Self {
        SetParameterAttributes {
            parameter_list: parameter_list,
        }
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["SetParameterAttributes", "ParameterList", "SetParameterAttributesStruct"] => self
                .parameter_list
                .push(SetParameterAttributesStruct::default()),
            ["SetParameterAttributes", "ParameterList", "SetParameterAttributesStruct", "AccessList", "string"] => {
                if let Some(p) = self.parameter_list.last_mut() {
                    p.access_list.push(String::from(""));
                }
            }
            _ => {}
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "SetParameterAttributes")[..],
        ))?;

        let pas = format!(
            "cwmp:SetParameterAttributesStruct[{}]",
            self.parameter_list.len()
        );
        writer
            .write(XmlEvent::start_element("ParameterList").attr("SOAP-ENC:arrayType", &pas[..]))?;

        for p in self.parameter_list.iter() {
            writer.write(XmlEvent::start_element("SetParameterAttributesStruct"))?;
            write_simple(writer, "Name", &p.name)?;
            write_simple(
                writer,
                "NotificationChange",
                &p.notification_change.to_string(),
            )?;
            write_simple(writer, "Notification", &p.notification.to_string())?;
            write_simple(
                writer,
                "AccessListChange",
                &p.access_list_change.to_string(),
            )?;
            writer.write(XmlEvent::start_element("AccessList"))?;
            for al in p.access_list.iter() {
                write_simple(writer, "string", &al)?;
            }
            writer.write(XmlEvent::end_element())?; // AccessList
            writer.write(XmlEvent::end_element())?; // SetParameterAttributesStruct
        }

        // ParameterList
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["SetParameterAttributes", "ParameterList", "SetParameterAttributesStruct", "AccessList", "string"] => {
                if let Some(p) = self.parameter_list.last_mut() {
                    if let Some(a) = p.access_list.last_mut() {
                        *a = characters.to_string();
                    }
                }
            }
            ["SetParameterAttributes", "ParameterList", "SetParameterAttributesStruct", key] => {
                if let Some(e) = self.parameter_list.last_mut() {
                    match key {
                        "Name" => e.name = characters.to_string(),
                        "NotificationChange" => e.notification_change = parse_to_int(characters, 0),
                        "Notification" => e.notification = parse_to_int(characters, 0),
                        "AccessListChange" => e.access_list_change = parse_to_int(characters, 0),
                        _ => {}
                    }
                }
            }

            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for SetParameterAttributes {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        SetParameterAttributes::new(Vec::<SetParameterAttributesStruct>::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.parameter_list
                .clone()
                .shrink()
                .map(|pl| SetParameterAttributes { parameter_list: pl }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SetParameterValuesResponse {
    status: u32,
}

impl SetParameterValuesResponse {
    pub fn new(status: u32) -> Self {
        SetParameterValuesResponse { status: status }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "SetParameterValuesResponse")[..],
        ))?;
        write_simple(writer, "Status", &self.status.to_string())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["SetParameterValuesResponse", "Status"] => self.status = parse_to_int(characters, 0),
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for SetParameterValuesResponse {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        SetParameterValuesResponse::new(u32::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.status
                .clone()
                .shrink()
                .map(|s| SetParameterValuesResponse { status: s }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SetParameterValues {
    parameter_list: Vec<ParameterValue>,
    parameter_key: Option<String>,
}

impl SetParameterValues {
    pub fn new(parameter_key: Option<String>, parameter_list: Vec<ParameterValue>) -> Self {
        SetParameterValues {
            parameter_list: parameter_list,
            parameter_key: parameter_key,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        let pvs = if has_cwmp {
            writer.write(XmlEvent::start_element("cwmp:SetParameterValues"))?;
            format!("cwmp:ParameterValueStruct[{}]", self.parameter_list.len())
        } else {
            writer.write(XmlEvent::start_element("SetParameterValues"))?;
            format!("ParameterValueStruct[{}]", self.parameter_list.len())
        };

        if let Some(pk) = &self.parameter_key {
            write_simple(writer, "ParameterKey", &pk)?;
        }
        if self.parameter_list.len() > 0 {
            writer.write(
                XmlEvent::start_element("ParameterList").attr("SOAP-ENC:arrayType", &pvs[..]),
            )?;

            for p in self.parameter_list.iter() {
                writer.write(XmlEvent::start_element("ParameterValueStruct"))?;
                write_simple(writer, "Name", &p.name)?;
                writer.write(XmlEvent::start_element("Value").attr("xsi:type", &p.r#type[..]))?;
                writer.write(&p.value[..])?;
                writer.write(XmlEvent::end_element())?; // Value
                writer.write(XmlEvent::end_element())?;
            }
            writer.write(XmlEvent::end_element())?;
        } else {
            write_empty_tag(writer, "ParameterList")?;
        }

        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["SetParameterValues", "ParameterKey"] => {
                self.parameter_key = Some(String::from(""));
            }
            ["SetParameterValues", "ParameterList", "ParameterValueStruct"] => {
                self.parameter_list.push(ParameterValue::default())
            }
            ["SetParameterValues", "ParameterList", "ParameterValueStruct", "Value"] => {
                // use the type attribute
                let last = self.parameter_list.last_mut();
                match last {
                    Some(e) => e.r#type = extract_attribute(attributes, "type"),
                    None => {}
                }
            }
            _ => {}
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["SetParameterValues", "ParameterKey"] => {
                self.parameter_key = Some(characters.to_string())
            }
            ["SetParameterValues", "ParameterList", "ParameterValueStruct", key] => {
                let last = self.parameter_list.last_mut();
                match last {
                    Some(e) => match key {
                        "Name" => e.name = characters.to_string(),
                        "Value" => e.value = characters.to_string(),
                        _ => {}
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for SetParameterValues {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        SetParameterValues::new(
            Option::<String>::arbitrary(g),
            Vec::<ParameterValue>::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.parameter_list.clone(), self.parameter_key.clone())
                .shrink()
                .map(|(pl, pk)| SetParameterValues {
                    parameter_list: pl,
                    parameter_key: pk,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SetVouchersResponse {}

impl SetVouchersResponse {
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        write_empty_tag(writer, &cwmp_prefix(has_cwmp, "SetVouchersResponse")[..])?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SetVouchers {
    voucher_list: Vec<String>,
}

impl SetVouchers {
    pub fn new(voucher_list: Vec<String>) -> Self {
        SetVouchers {
            voucher_list: voucher_list,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "SetVouchers")[..],
        ))?;

        let vls = format!("base64[{}]", self.voucher_list.len());
        writer
            .write(XmlEvent::start_element("VoucherList").attr("SOAP-ENC:arrayType", &vls[..]))?;

        for v in self.voucher_list.iter() {
            write_simple(writer, "base64", &v)?;
        }
        writer.write(XmlEvent::end_element())?; // VoucherList
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["SetVouchers", "VoucherList", "base64"] => self.voucher_list.push(String::from("")),
            _ => {}
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["SetVouchers", "VoucherList", "base64"] => {
                let last = self.voucher_list.last_mut();
                match last {
                    Some(e) => *e = characters.to_string(),
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for SetVouchers {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        SetVouchers::new(Vec::<String>::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.voucher_list
                .clone()
                .shrink()
                .map(|vl| SetVouchers { voucher_list: vl }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct TransferCompleteResponse {}

impl TransferCompleteResponse {
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        write_empty_tag(
            writer,
            &cwmp_prefix(has_cwmp, "TransferCompleteResponse")[..],
        )?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct TransferComplete {
    command_key: String,
    fault: FaultStruct,
    start_time: Option<DateTime<Utc>>,
    complete_time: Option<DateTime<Utc>>,
}

impl TransferComplete {
    pub fn new(
        command_key: String,
        fault: FaultStruct,
        start_time: Option<DateTime<Utc>>,
        complete_time: Option<DateTime<Utc>>,
    ) -> Self {
        TransferComplete {
            command_key: command_key.to_string(),
            fault: fault,
            start_time: start_time,
            complete_time: complete_time,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "TransferComplete")[..],
        ))?;
        write_simple(writer, "CommandKey", &self.command_key)?;
        write_fault_struct(writer, &self.fault)?;
        match self.start_time {
            None => {}
            Some(dt) => write_simple(writer, "StartTime", &dt.to_rfc3339())?,
        }
        match self.complete_time {
            None => {}
            Some(dt) => write_simple(writer, "CompleteTime", &dt.to_rfc3339())?,
        }

        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["TransferComplete", "CommandKey"] => self.command_key = characters.to_string(),
            ["TransferComplete", "StartTime"] => match characters.parse::<DateTime<Utc>>() {
                Ok(dt) => self.start_time = Some(dt),
                _ => {}
            },
            ["TransferComplete", "CompleteTime"] => match characters.parse::<DateTime<Utc>>() {
                Ok(dt) => self.complete_time = Some(dt),
                _ => {}
            },
            ["TransferComplete", "FaultStruct", "FaultCode"] => {
                self.fault.set_code(parse_to_int(characters, 0))
            }
            ["TransferComplete", "FaultStruct", "FaultString"] => self.fault.set_string(characters),

            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for TransferComplete {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        TransferComplete::new(
            String::arbitrary(g),
            FaultStruct::arbitrary(g),
            Some(Utc.ymd(2014, 11, 28).and_hms(12, 0, 9)),
            Some(Utc.ymd(2014, 11, 29).and_hms(12, 0, 9)),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.command_key.clone(), self.fault.clone())
                .shrink()
                .map(|(c, f)| TransferComplete {
                    command_key: c,
                    fault: f,
                    start_time: Some(Utc.ymd(2014, 11, 28).and_hms(12, 0, 9)),
                    complete_time: Some(Utc.ymd(2014, 11, 29).and_hms(12, 0, 9)),
                }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct UploadResponse {
    status: u8,
    start_time: Option<DateTime<Utc>>,
    complete_time: Option<DateTime<Utc>>,
}

impl UploadResponse {
    pub fn new(
        status: u8,
        start_time: Option<DateTime<Utc>>,
        complete_time: Option<DateTime<Utc>>,
    ) -> Self {
        UploadResponse {
            status: status,
            start_time: start_time,
            complete_time: complete_time,
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "UploadResponse")[..],
        ))?;
        write_simple(writer, "Status", &self.status.to_string())?;
        match self.start_time {
            None => {}
            Some(dt) => write_simple(writer, "StartTime", &dt.to_rfc3339())?,
        }
        match self.complete_time {
            None => {}
            Some(dt) => write_simple(writer, "CompleteTime", &dt.to_rfc3339())?,
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["UploadResponse", "Status"] => self.status = parse_to_int(characters, 0),
            ["UploadResponse", "StartTime"] => match characters.parse::<DateTime<Utc>>() {
                Ok(dt) => self.start_time = Some(dt),
                _ => {}
            },
            ["UploadResponse", "CompleteTime"] => match characters.parse::<DateTime<Utc>>() {
                Ok(dt) => self.complete_time = Some(dt),
                _ => {}
            },
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for UploadResponse {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        UploadResponse::new(
            u8::arbitrary(g),
            Some(Utc.ymd(2014, 11, 28).and_hms(12, 0, 9)),
            Some(Utc.ymd(2014, 11, 29).and_hms(12, 0, 9)),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.status.clone().shrink().map(|s| UploadResponse {
            status: s,
            start_time: Some(Utc.ymd(2014, 11, 28).and_hms(12, 0, 9)),
            complete_time: Some(Utc.ymd(2014, 11, 29).and_hms(12, 0, 9)),
        }))
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Upload {
    command_key: String,
    file_type: String,
    url: String,
    username: String,
    password: String,
    delay_seconds: u32,
}

impl Upload {
    pub fn new(
        command_key: String,
        file_type: String,
        url: String,
        username: String,
        password: String,
        delay_seconds: u32,
    ) -> Self {
        Upload {
            command_key: command_key,
            file_type: file_type,
            url: url,
            username: username,
            password: password,
            delay_seconds: delay_seconds,
        }
    }

    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "Upload")[..],
        ))?;
        writer.write(XmlEvent::start_element("CommandKey"))?;
        writer.write(&self.command_key[..])?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::start_element("FileType"))?;
        writer.write(&self.file_type[..])?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::start_element("URL"))?;
        writer.write(&self.url[..])?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::start_element("Username"))?;
        writer.write(&self.username[..])?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::start_element("Password"))?;
        writer.write(&self.password[..])?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::start_element("DelaySeconds"))?;
        let s: String = self.delay_seconds.to_string();
        writer.write(&s[..])?;
        writer.write(XmlEvent::end_element())?;

        let e: XmlEvent = XmlEvent::end_element().into();
        writer.write(e)?;
        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["Upload", "CommandKey"] => self.command_key = characters.to_string(),
            ["Upload", "FileType"] => self.file_type = characters.to_string(),
            ["Upload", "URL"] => self.url = characters.to_string(),
            ["Upload", "Username"] => self.username = characters.to_string(),
            ["Upload", "Password"] => self.password = characters.to_string(),
            ["Upload", "DelaySeconds"] => self.delay_seconds = parse_to_int(characters, 0),
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for Upload {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Upload::new(
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            u32::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.command_key.clone(),
                self.file_type.clone(),
                self.url.clone(),
                self.username.clone(),
                self.password.clone(),
                self.delay_seconds.clone(),
            )
                .shrink()
                .map(|(c, f, u, un, pw, ds)| Upload {
                    command_key: c,
                    file_type: f,
                    url: u,
                    username: un,
                    password: pw,
                    delay_seconds: ds,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BodyElement {
    AddObjectResponse(AddObjectResponse),
    AddObject(AddObject),
    AutonomousDUStateChangeCompleteResponse(AutonomousDUStateChangeCompleteResponse),
    AutonomousDUStateChangeComplete(AutonomousDUStateChangeComplete),
    AutonomousTransferCompleteResponse(AutonomousTransferCompleteResponse),
    AutonomousTransferComplete(AutonomousTransferComplete),
    CancelTransferResponse(CancelTransferResponse),
    CancelTransfer(CancelTransfer),
    ChangeDUStateResponse(ChangeDUStateResponse),
    ChangeDUState(ChangeDUState),
    DeleteObjectResponse(DeleteObjectResponse),
    DeleteObject(DeleteObject),
    DownloadResponse(DownloadResponse),
    Download(Download),
    DUStateChangeCompleteResponse(DUStateChangeCompleteResponse),
    DUStateChangeComplete(DUStateChangeComplete),
    FactoryResetResponse(FactoryResetResponse),
    FactoryReset(FactoryReset),
    Fault(Fault),
    GetAllQueuedTransfersResponse(GetAllQueuedTransfersResponse),
    GetAllQueuedTransfers(GetAllQueuedTransfers),
    GetOptionsResponse(GetOptionsResponse),
    GetOptions(GetOptions),
    GetParameterAttributes(GetParameterAttributes),
    GetParameterAttributesResponse(GetParameterAttributesResponse),
    GetParameterNamesResponse(GetParameterNamesResponse),
    GetParameterNames(GetParameterNames),
    GetParameterValues(GetParameterValues),
    GetParameterValuesResponse(GetParameterValuesResponse),
    GetQueuedTransfersResponse(GetQueuedTransfersResponse),
    GetQueuedTransfers(GetQueuedTransfers),
    GetRPCMethodsResponse(GetRPCMethodsResponse),
    GetRPCMethods(GetRPCMethods),
    InformResponse(InformResponse),
    Inform(Inform),
    KickedResponse(KickedResponse),
    Kicked(Kicked),
    RebootResponse(RebootResponse),
    Reboot(Reboot),
    RequestDownloadResponse(RequestDownloadResponse),
    RequestDownload(RequestDownload),
    ScheduleDownloadResponse(ScheduleDownloadResponse),
    ScheduleDownload(ScheduleDownload),
    ScheduleInformResponse(ScheduleInformResponse),
    ScheduleInform(ScheduleInform),
    SetParameterAttributesResponse(SetParameterAttributesResponse),
    SetParameterAttributes(SetParameterAttributes),
    SetParameterValuesResponse(SetParameterValuesResponse),
    SetParameterValues(SetParameterValues),
    SetVouchersResponse(SetVouchersResponse),
    SetVouchers(SetVouchers),
    TransferCompleteResponse(TransferCompleteResponse),
    TransferComplete(TransferComplete),
    UploadResponse(UploadResponse),
    Upload(Upload),
}

#[cfg(test)]
impl Arbitrary for BodyElement {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let vals = vec![
            BodyElement::AddObjectResponse(AddObjectResponse::arbitrary(g)),
            BodyElement::AddObject(AddObject::arbitrary(g)),
            BodyElement::AutonomousDUStateChangeCompleteResponse(
                AutonomousDUStateChangeCompleteResponse {},
            ),
            BodyElement::AutonomousDUStateChangeComplete(
                AutonomousDUStateChangeComplete::arbitrary(g),
            ),
            BodyElement::AutonomousTransferCompleteResponse(AutonomousTransferCompleteResponse {}),
            BodyElement::AutonomousTransferComplete(AutonomousTransferComplete::arbitrary(g)),
            BodyElement::CancelTransferResponse(CancelTransferResponse {}),
            BodyElement::CancelTransfer(CancelTransfer::arbitrary(g)),
            BodyElement::ChangeDUStateResponse(ChangeDUStateResponse {}),
            BodyElement::ChangeDUState(ChangeDUState::arbitrary(g)),
            BodyElement::DeleteObjectResponse(DeleteObjectResponse::arbitrary(g)),
            BodyElement::DeleteObject(DeleteObject::arbitrary(g)),
            BodyElement::DownloadResponse(DownloadResponse::arbitrary(g)),
            BodyElement::Download(Download::arbitrary(g)),
            BodyElement::DUStateChangeCompleteResponse(DUStateChangeCompleteResponse {}),
            BodyElement::DUStateChangeComplete(DUStateChangeComplete::arbitrary(g)),
            BodyElement::FactoryResetResponse(FactoryResetResponse {}),
            BodyElement::FactoryReset(FactoryReset {}),
            BodyElement::Fault(Fault::arbitrary(g)),
            BodyElement::GetAllQueuedTransfersResponse(GetAllQueuedTransfersResponse::arbitrary(g)),
            BodyElement::GetAllQueuedTransfers(GetAllQueuedTransfers {}),
            BodyElement::GetOptionsResponse(GetOptionsResponse::arbitrary(g)),
            BodyElement::GetOptions(GetOptions::arbitrary(g)),
            BodyElement::GetParameterAttributes(GetParameterAttributes::arbitrary(g)),
            BodyElement::GetParameterAttributesResponse(GetParameterAttributesResponse::arbitrary(
                g,
            )),
            BodyElement::GetParameterNamesResponse(GetParameterNamesResponse::arbitrary(g)),
            BodyElement::GetParameterNames(GetParameterNames::arbitrary(g)),
            BodyElement::GetParameterValues(GetParameterValues::arbitrary(g)),
            BodyElement::GetParameterValuesResponse(GetParameterValuesResponse::arbitrary(g)),
            BodyElement::GetQueuedTransfersResponse(GetQueuedTransfersResponse::arbitrary(g)),
            BodyElement::GetQueuedTransfers(GetQueuedTransfers {}),
            BodyElement::GetRPCMethodsResponse(GetRPCMethodsResponse::arbitrary(g)),
            BodyElement::GetRPCMethods(GetRPCMethods {}),
            BodyElement::InformResponse(InformResponse::arbitrary(g)),
            BodyElement::Inform(Inform::arbitrary(g)),
            BodyElement::KickedResponse(KickedResponse::arbitrary(g)),
            BodyElement::Kicked(Kicked::arbitrary(g)),
            BodyElement::RebootResponse(RebootResponse {}),
            BodyElement::Reboot(Reboot::arbitrary(g)),
            BodyElement::RequestDownloadResponse(RequestDownloadResponse {}),
            BodyElement::RequestDownload(RequestDownload::arbitrary(g)),
            BodyElement::ScheduleDownloadResponse(ScheduleDownloadResponse {}),
            BodyElement::ScheduleDownload(ScheduleDownload::arbitrary(g)),
            BodyElement::ScheduleInformResponse(ScheduleInformResponse {}),
            BodyElement::ScheduleInform(ScheduleInform::arbitrary(g)),
            BodyElement::SetParameterAttributesResponse(SetParameterAttributesResponse {}),
            BodyElement::SetParameterAttributes(SetParameterAttributes::arbitrary(g)),
            BodyElement::SetParameterValuesResponse(SetParameterValuesResponse::arbitrary(g)),
            BodyElement::SetParameterValues(SetParameterValues::arbitrary(g)),
            BodyElement::SetVouchersResponse(SetVouchersResponse {}),
            BodyElement::SetVouchers(SetVouchers::arbitrary(g)),
            BodyElement::TransferCompleteResponse(TransferCompleteResponse {}),
            BodyElement::TransferComplete(TransferComplete::arbitrary(g)),
            BodyElement::UploadResponse(UploadResponse::arbitrary(g)),
            BodyElement::Upload(Upload::arbitrary(g)),
        ];
        vals.choose(g).unwrap().clone()
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        match self {
            &BodyElement::AddObjectResponse(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::AddObjectResponse(s)))
            }
            &BodyElement::AddObject(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::AddObject(s)))
            }
            &BodyElement::AutonomousDUStateChangeCompleteResponse(_) => {
                quickcheck::empty_shrinker()
            }
            &BodyElement::AutonomousDUStateChangeComplete(ref x) => Box::new(
                x.shrink()
                    .map(|s| BodyElement::AutonomousDUStateChangeComplete(s)),
            ),
            &BodyElement::AutonomousTransferCompleteResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::AutonomousTransferComplete(ref x) => Box::new(
                x.shrink()
                    .map(|s| BodyElement::AutonomousTransferComplete(s)),
            ),
            &BodyElement::CancelTransferResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::CancelTransfer(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::CancelTransfer(s)))
            }
            &BodyElement::ChangeDUStateResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::ChangeDUState(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::ChangeDUState(s)))
            }
            &BodyElement::DeleteObjectResponse(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::DeleteObjectResponse(s)))
            }
            &BodyElement::DeleteObject(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::DeleteObject(s)))
            }
            &BodyElement::DownloadResponse(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::DownloadResponse(s)))
            }
            &BodyElement::Download(ref x) => Box::new(x.shrink().map(|s| BodyElement::Download(s))),
            &BodyElement::DUStateChangeCompleteResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::DUStateChangeComplete(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::DUStateChangeComplete(s)))
            }
            &BodyElement::FactoryResetResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::FactoryReset(_) => quickcheck::empty_shrinker(),
            &BodyElement::Fault(ref x) => Box::new(x.shrink().map(|s| BodyElement::Fault(s))),
            &BodyElement::GetAllQueuedTransfersResponse(ref x) => Box::new(
                x.shrink()
                    .map(|s| BodyElement::GetAllQueuedTransfersResponse(s)),
            ),
            &BodyElement::GetAllQueuedTransfers(_) => quickcheck::empty_shrinker(),
            &BodyElement::GetOptionsResponse(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::GetOptionsResponse(s)))
            }
            &BodyElement::GetOptions(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::GetOptions(s)))
            }
            &BodyElement::GetParameterAttributesResponse(ref x) => Box::new(
                x.shrink()
                    .map(|s| BodyElement::GetParameterAttributesResponse(s)),
            ),
            &BodyElement::GetParameterAttributes(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::GetParameterAttributes(s)))
            }
            &BodyElement::GetParameterNamesResponse(ref x) => Box::new(
                x.shrink()
                    .map(|s| BodyElement::GetParameterNamesResponse(s)),
            ),
            &BodyElement::GetParameterNames(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::GetParameterNames(s)))
            }
            &BodyElement::GetParameterValuesResponse(ref x) => Box::new(
                x.shrink()
                    .map(|s| BodyElement::GetParameterValuesResponse(s)),
            ),
            &BodyElement::GetParameterValues(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::GetParameterValues(s)))
            }
            &BodyElement::GetQueuedTransfersResponse(ref x) => Box::new(
                x.shrink()
                    .map(|s| BodyElement::GetQueuedTransfersResponse(s)),
            ),
            &BodyElement::GetQueuedTransfers(_) => quickcheck::empty_shrinker(),
            &BodyElement::GetRPCMethodsResponse(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::GetRPCMethodsResponse(s)))
            }
            &BodyElement::GetRPCMethods(_) => quickcheck::empty_shrinker(),
            &BodyElement::InformResponse(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::InformResponse(s)))
            }
            &BodyElement::Inform(ref x) => Box::new(x.shrink().map(|s| BodyElement::Inform(s))),
            &BodyElement::KickedResponse(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::KickedResponse(s)))
            }
            &BodyElement::Kicked(ref x) => Box::new(x.shrink().map(|s| BodyElement::Kicked(s))),
            &BodyElement::RebootResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::Reboot(ref x) => Box::new(x.shrink().map(|s| BodyElement::Reboot(s))),
            &BodyElement::RequestDownloadResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::RequestDownload(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::RequestDownload(s)))
            }
            &BodyElement::ScheduleDownloadResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::ScheduleDownload(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::ScheduleDownload(s)))
            }
            &BodyElement::ScheduleInformResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::ScheduleInform(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::ScheduleInform(s)))
            }
            &BodyElement::SetParameterAttributesResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::SetParameterAttributes(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::SetParameterAttributes(s)))
            }
            &BodyElement::SetParameterValuesResponse(ref x) => Box::new(
                x.shrink()
                    .map(|s| BodyElement::SetParameterValuesResponse(s)),
            ),
            &BodyElement::SetParameterValues(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::SetParameterValues(s)))
            }
            &BodyElement::SetVouchersResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::SetVouchers(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::SetVouchers(s)))
            }
            &BodyElement::TransferCompleteResponse(_) => quickcheck::empty_shrinker(),
            &BodyElement::TransferComplete(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::TransferComplete(s)))
            }
            &BodyElement::UploadResponse(ref x) => {
                Box::new(x.shrink().map(|s| BodyElement::UploadResponse(s)))
            }
            &BodyElement::Upload(ref x) => Box::new(x.shrink().map(|s| BodyElement::Upload(s))),
        }
    }
}

#[derive(Debug, PartialEq, Default, Clone)]
pub struct CwmpVersion {
    major: u8,
    minor: u8,
}

impl CwmpVersion {
    pub fn new(major: u8, minor: u8) -> Self {
        CwmpVersion {
            major: major,
            minor: minor,
        }
    }
}

#[cfg(test)]
impl Arbitrary for CwmpVersion {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        CwmpVersion {
            major: u8::arbitrary(g),
            minor: u8::arbitrary(g),
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.major.clone(), self.minor.clone())
                .shrink()
                .map(|(ma, mi)| CwmpVersion {
                    major: ma,
                    minor: mi,
                }),
        )
    }
}

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Envelope {
    cwmp_version: Option<CwmpVersion>,
    header: Vec<HeaderElement>,
    body: Vec<BodyElement>,
}

#[cfg(test)]
impl Arbitrary for Envelope {
    fn arbitrary<G: Gen>(g: &mut G) -> Envelope {
        // cwmp version is handled a bit special, because
        // a value of Some("") becomes xmlns:cwmp="" which
        // is unparsable
        let header = Vec::<HeaderElement>::arbitrary(g);
        let body = Vec::<BodyElement>::arbitrary(g);
        let cwmp = Option::<CwmpVersion>::arbitrary(g);

        Envelope::new(cwmp.clone(), header, body)
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.cwmp_version.clone(),
                self.header.clone(),
                self.body.clone(),
            )
                .shrink()
                .map(|(c, h, b)| Envelope {
                    cwmp_version: c,
                    header: h,
                    body: b,
                }),
        )
    }
}

pub enum GenerateError {
    FromUtf8Error(std::string::FromUtf8Error),
    XmlError(xml::writer::Error),
}

impl From<xml::writer::Error> for GenerateError {
    fn from(e: xml::writer::Error) -> GenerateError {
        GenerateError::XmlError(e)
    }
}
impl From<std::string::FromUtf8Error> for GenerateError {
    fn from(e: std::string::FromUtf8Error) -> GenerateError {
        GenerateError::FromUtf8Error(e)
    }
}

impl fmt::Display for GenerateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenerateError::FromUtf8Error(ref err) => std::fmt::Display::fmt(&err, f),
            GenerateError::XmlError(ref err) => std::fmt::Display::fmt(&err, f),
        }
    }
}
impl fmt::Debug for GenerateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenerateError::FromUtf8Error(ref err) => std::fmt::Debug::fmt(&err, f),
            GenerateError::XmlError(ref err) => std::fmt::Debug::fmt(&err, f),
        }
    }
}

impl Envelope {
    pub fn new(
        cwmp_version: Option<CwmpVersion>,
        header: Vec<HeaderElement>,
        body: Vec<BodyElement>,
    ) -> Self {
        Envelope {
            cwmp_version: cwmp_version,
            header: header,
            body: body,
        }
    }
    pub fn cwmp_version(self) -> Option<CwmpVersion> {
        self.cwmp_version
    }
    pub fn header(self) -> Vec<HeaderElement> {
        self.header
    }
    pub fn body(self) -> Vec<BodyElement> {
        self.body
    }
    pub fn generate(&self) -> Result<String, GenerateError> {
        let mut writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(Vec::new());

        let mut start_event = XmlEvent::start_element("Envelope")
            .ns("SOAP-ENV", "http://schemas.xmlsoap.org/soap/envelope/")
            .ns("SOAP-ENC", "http://schemas.xmlsoap.org/soap/encoding/")
            .ns("xsi", "http://www.w3.org/2001/XMLSchema-instance")
            .ns("xsd", "http://www.w3.org/2001/XMLSchema");

        if let Some(cwmp) = &self.cwmp_version {
            start_event = start_event.ns(
                "cwmp",
                format!("urn:dslforum-org:cwmp-{}-{}", cwmp.major, cwmp.minor),
            );
        }
        start_event = start_event.attr(
            "SOAP-ENV:encodingStyle",
            "http://schemas.xmlsoap.org/soap/encoding/",
        );

        writer.write(start_event)?;

        // now generate the header elements
        let start_header = XmlEvent::start_element("SOAP-ENV:Header");
        writer.write(start_header)?;

        for he in self.header.iter() {
            match he {
                HeaderElement::ID(e) => e.generate(&mut writer, self.cwmp_version.is_some())?,
                HeaderElement::HoldRequests(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                HeaderElement::NoMoreRequests(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                HeaderElement::SessionTimeout(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                HeaderElement::SupportedCWMPVersions(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                HeaderElement::UseCWMPVersion(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
            };
        }

        let end_header: XmlEvent = XmlEvent::end_element().into();
        writer.write(end_header)?;

        // now generate the body elemenets
        let body_start = XmlEvent::start_element("SOAP-ENV:Body");
        writer.write(body_start)?;

        for be in self.body.iter() {
            match be {
                BodyElement::AddObject(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::AddObjectResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::AutonomousDUStateChangeComplete(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::AutonomousDUStateChangeCompleteResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::AutonomousTransferComplete(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::AutonomousTransferCompleteResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::CancelTransferResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::CancelTransfer(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::ChangeDUStateResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::ChangeDUState(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::DeleteObjectResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::DeleteObject(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::DownloadResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::Download(e) => e.generate(&mut writer, self.cwmp_version.is_some())?,
                BodyElement::DUStateChangeCompleteResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::DUStateChangeComplete(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::FactoryResetResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::FactoryReset(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::Fault(e) => e.generate(&mut writer, self.cwmp_version.is_some())?,
                BodyElement::GetAllQueuedTransfersResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::GetAllQueuedTransfers(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::GetOptionsResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::GetOptions(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::GetParameterAttributes(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::GetParameterAttributesResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::GetParameterNamesResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::GetParameterNames(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::GetParameterValues(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::GetParameterValuesResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::GetQueuedTransfersResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::GetQueuedTransfers(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::GetRPCMethodsResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::GetRPCMethods(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::InformResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::Inform(e) => e.generate(&mut writer, self.cwmp_version.is_some())?,
                BodyElement::KickedResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::Kicked(e) => e.generate(&mut writer, self.cwmp_version.is_some())?,
                BodyElement::RebootResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::Reboot(e) => e.generate(&mut writer, self.cwmp_version.is_some())?,
                BodyElement::RequestDownloadResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::RequestDownload(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::ScheduleDownloadResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::ScheduleDownload(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::ScheduleInformResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::ScheduleInform(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::SetParameterAttributesResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::SetParameterAttributes(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::SetParameterValuesResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::SetParameterValues(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::SetVouchersResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::SetVouchers(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::TransferCompleteResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::TransferComplete(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
                BodyElement::Upload(e) => e.generate(&mut writer, self.cwmp_version.is_some())?,
                BodyElement::UploadResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?
                }
            }
        }

        let body_end: XmlEvent = XmlEvent::end_element().into();
        writer.write(body_end)?;

        // End the Envelope
        let end_event: XmlEvent = XmlEvent::end_element().into();
        writer.write(end_event)?;

        Ok(String::from_utf8(writer.into_inner())?)
    }
    fn start_handler(
        &mut self,
        path: &Vec<String>,
        name: &xml::name::OwnedName,
        attributes: &Vec<xml::attribute::OwnedAttribute>,
        namespace: &xml::namespace::Namespace,
    ) {
        // match out all the elements in path. If the path goes into body,
        // call the start_handler for each element in the Body vector
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["Envelope"] => {
                // search through the namespaces to find a cwmp value
                match namespace.get("cwmp") {
                    Some(ns) => self.cwmp_version = Some(cwmp_urn_to_version(ns)),
                    None => self.cwmp_version = None,
                }
            }
            ["Envelope", "Header", header_element] => {
                // check if there is a mustUnderstand attribute, and if so, check
                // if we actually understand the header_element given

                // the mustUnderstand attributes is used in more than one header element
                let must_understand_filter = attributes
                    .iter()
                    .filter(|&x| x.name.local_name == "mustUnderstand")
                    .next();

                let must_understand: bool = if let Some(mua) = must_understand_filter {
                    str2bool(&mua.value.to_string())
                } else {
                    true
                };
                match *header_element {
                    "ID" => self.header.push(HeaderElement::ID(ID {
                        must_understand: must_understand,
                        id: String::from(""),
                    })),
                    "NoMoreRequests" => {
                        self.header
                            .push(HeaderElement::NoMoreRequests(NoMoreRequests::new(
                                must_understand,
                                0,
                            )))
                    }
                    "HoldRequests" => {
                        self.header
                            .push(HeaderElement::HoldRequests(HoldRequests::new(
                                must_understand,
                                false,
                            )))
                    }
                    "SessionTimeout" => {
                        self.header
                            .push(HeaderElement::SessionTimeout(SessionTimeout::new(
                                must_understand,
                                0,
                            )))
                    }
                    "SupportedCWMPVersions" => {
                        self.header.push(HeaderElement::SupportedCWMPVersions(
                            SupportedCWMPVersions::new(must_understand, String::from("")),
                        ))
                    }
                    "UseCWMPVersion" => {
                        self.header
                            .push(HeaderElement::UseCWMPVersion(UseCWMPVersion::new(
                                must_understand,
                                String::from(""),
                            )))
                    }
                    _ => {}
                }
            }
            ["Envelope", "Body", body_element, ..] => {
                // Start of a new body element, create an instance of the correct
                // type, push the empty instance on to the stack and call the start
                // handler on the type
                if path_pattern.len() == 3 {
                    // an actual new Body element
                    match *body_element {
                        "AddObject" => self.body.push(BodyElement::AddObject(AddObject {
                            object_name: String::from(""),
                            parameter_key: String::from(""),
                        })),
                        "AddObjectResponse" => {
                            self.body
                                .push(BodyElement::AddObjectResponse(AddObjectResponse {
                                    instance_number: 0,
                                    status: String::from("0"),
                                }))
                        }
                        "AutonomousDUStateChangeCompleteResponse" => {
                            self.body
                                .push(BodyElement::AutonomousDUStateChangeCompleteResponse(
                                    AutonomousDUStateChangeCompleteResponse {},
                                ))
                        }
                        "AutonomousDUStateChangeComplete" => {
                            self.body.push(BodyElement::AutonomousDUStateChangeComplete(
                                AutonomousDUStateChangeComplete::default(),
                            ))
                        }
                        "AutonomousTransferCompleteResponse" => {
                            self.body
                                .push(BodyElement::AutonomousTransferCompleteResponse(
                                    AutonomousTransferCompleteResponse {},
                                ))
                        }
                        "AutonomousTransferComplete" => {
                            self.body.push(BodyElement::AutonomousTransferComplete(
                                AutonomousTransferComplete::new(
                                    String::from(""),
                                    String::from(""),
                                    0,
                                    String::from(""),
                                    0,
                                    String::from(""),
                                    FaultStruct::new(0, String::from("")),
                                    Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
                                    Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
                                ),
                            ))
                        }
                        "CancelTransferResponse" => self.body.push(
                            BodyElement::CancelTransferResponse(CancelTransferResponse {}),
                        ),
                        "CancelTransfer" => {
                            self.body
                                .push(BodyElement::CancelTransfer(CancelTransfer::new(
                                    String::from(""),
                                )))
                        }
                        "ChangeDUStateResponse" => self
                            .body
                            .push(BodyElement::ChangeDUStateResponse(ChangeDUStateResponse {})),
                        "ChangeDUState" => {
                            self.body
                                .push(BodyElement::ChangeDUState(ChangeDUState::new(
                                    String::from(""),
                                    vec![],
                                    vec![],
                                    vec![],
                                )))
                        }
                        "DeleteObjectResponse" => {
                            self.body.push(BodyElement::DeleteObjectResponse(
                                DeleteObjectResponse::new(String::from("")),
                            ))
                        }
                        "DeleteObject" => self.body.push(BodyElement::DeleteObject(
                            DeleteObject::new(String::from(""), String::from("")),
                        )),
                        "DownloadResponse" => {
                            self.body
                                .push(BodyElement::DownloadResponse(DownloadResponse::new(
                                    String::from(""),
                                    Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
                                    Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
                                )))
                        }
                        "Download" => self.body.push(BodyElement::Download(Download::new(
                            String::from(""),
                            String::from(""),
                            String::from(""),
                            String::from(""),
                            String::from(""),
                            0,
                            String::from(""),
                            0,
                            String::from(""),
                            String::from(""),
                        ))),
                        "DUStateChangeCompleteResponse" => {
                            self.body.push(BodyElement::DUStateChangeCompleteResponse(
                                DUStateChangeCompleteResponse {},
                            ))
                        }
                        "DUStateChangeComplete" => {
                            self.body.push(BodyElement::DUStateChangeComplete(
                                DUStateChangeComplete::new(String::from(""), vec![]),
                            ))
                        }
                        "FactoryResetResponse" => self
                            .body
                            .push(BodyElement::FactoryResetResponse(FactoryResetResponse {})),
                        "FactoryReset" => {
                            self.body.push(BodyElement::FactoryReset(FactoryReset {}))
                        }
                        "Fault" => self.body.push(BodyElement::Fault(Fault::new(
                            String::from(""),
                            String::from(""),
                            0,
                            String::from(""),
                        ))),
                        "GetAllQueuedTransfersResponse" => {
                            self.body.push(BodyElement::GetAllQueuedTransfersResponse(
                                GetAllQueuedTransfersResponse::new(vec![]),
                            ))
                        }
                        "GetAllQueuedTransfers" => self
                            .body
                            .push(BodyElement::GetAllQueuedTransfers(GetAllQueuedTransfers {})),
                        "GetOptionsResponse" => self.body.push(BodyElement::GetOptionsResponse(
                            GetOptionsResponse::new(vec![]),
                        )),
                        "GetOptions" => self.body.push(BodyElement::GetOptions(Default::default())),
                        "GetParameterAttributes" => self.body.push(
                            BodyElement::GetParameterAttributes(GetParameterAttributes {
                                parameternames: vec![],
                            }),
                        ),
                        "GetParameterAttributesResponse" => {
                            self.body.push(BodyElement::GetParameterAttributesResponse(
                                GetParameterAttributesResponse { parameters: vec![] },
                            ))
                        }
                        "GetParameterNamesResponse" => {
                            self.body.push(BodyElement::GetParameterNamesResponse(
                                GetParameterNamesResponse::default(),
                            ))
                        }
                        "GetParameterNames" => self
                            .body
                            .push(BodyElement::GetParameterNames(GetParameterNames::default())),
                        "GetParameterValues" => {
                            self.body
                                .push(BodyElement::GetParameterValues(GetParameterValues {
                                    parameternames: vec![],
                                }))
                        }
                        "GetParameterValuesResponse" => {
                            self.body.push(BodyElement::GetParameterValuesResponse(
                                GetParameterValuesResponse { parameters: vec![] },
                            ))
                        }
                        "GetQueuedTransfersResponse" => {
                            self.body.push(BodyElement::GetQueuedTransfersResponse(
                                GetQueuedTransfersResponse::new(vec![]),
                            ))
                        }
                        "GetQueuedTransfers" => self
                            .body
                            .push(BodyElement::GetQueuedTransfers(GetQueuedTransfers {})),
                        "GetRPCMethodsResponse" => self.body.push(
                            BodyElement::GetRPCMethodsResponse(GetRPCMethodsResponse::new(vec![])),
                        ),
                        "GetRPCMethods" => {
                            self.body.push(BodyElement::GetRPCMethods(GetRPCMethods {}))
                        }
                        "InformResponse" => self
                            .body
                            .push(BodyElement::InformResponse(InformResponse::new(1))),
                        "Inform" => self.body.push(BodyElement::Inform(Inform::default())),
                        "KickedResponse" => self
                            .body
                            .push(BodyElement::KickedResponse(KickedResponse::default())),
                        "Kicked" => self.body.push(BodyElement::Kicked(Kicked::default())),
                        "RebootResponse" => self
                            .body
                            .push(BodyElement::RebootResponse(RebootResponse::default())),
                        "Reboot" => self.body.push(BodyElement::Reboot(Reboot::default())),
                        "RequestDownloadResponse" => self.body.push(
                            BodyElement::RequestDownloadResponse(RequestDownloadResponse {}),
                        ),
                        "RequestDownload" => self
                            .body
                            .push(BodyElement::RequestDownload(RequestDownload::default())),
                        "ScheduleDownloadResponse" => self.body.push(
                            BodyElement::ScheduleDownloadResponse(ScheduleDownloadResponse {}),
                        ),
                        "ScheduleDownload" => self
                            .body
                            .push(BodyElement::ScheduleDownload(ScheduleDownload::default())),
                        "ScheduleInformResponse" => self.body.push(
                            BodyElement::ScheduleInformResponse(ScheduleInformResponse {}),
                        ),
                        "ScheduleInform" => self
                            .body
                            .push(BodyElement::ScheduleInform(ScheduleInform::default())),
                        "SetParameterAttributesResponse" => {
                            self.body.push(BodyElement::SetParameterAttributesResponse(
                                SetParameterAttributesResponse {},
                            ))
                        }
                        "SetParameterAttributes" => self.body.push(
                            BodyElement::SetParameterAttributes(SetParameterAttributes::default()),
                        ),
                        "SetParameterValuesResponse" => {
                            self.body.push(BodyElement::SetParameterValuesResponse(
                                SetParameterValuesResponse::default(),
                            ))
                        }
                        "SetParameterValues" => self.body.push(BodyElement::SetParameterValues(
                            SetParameterValues::default(),
                        )),
                        "SetVouchersResponse" => self
                            .body
                            .push(BodyElement::SetVouchersResponse(SetVouchersResponse {})),
                        "SetVouchers" => self
                            .body
                            .push(BodyElement::SetVouchers(SetVouchers::default())),
                        "TransferCompleteResponse" => self.body.push(
                            BodyElement::TransferCompleteResponse(TransferCompleteResponse {}),
                        ),
                        "TransferComplete" => self
                            .body
                            .push(BodyElement::TransferComplete(TransferComplete::default())),
                        "UploadResponse" => self
                            .body
                            .push(BodyElement::UploadResponse(UploadResponse::default())),
                        "Upload" => self.body.push(BodyElement::Upload(Upload::default())),
                        _ => {}
                    }
                }
                let last = self.body.last_mut();
                match last {
                    Some(BodyElement::AutonomousDUStateChangeComplete(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::GetParameterAttributesResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::GetParameterAttributes(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::GetParameterValuesResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::ChangeDUState(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::DUStateChangeComplete(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::GetAllQueuedTransfersResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::GetOptionsResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::GetParameterNamesResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::GetParameterValues(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::GetQueuedTransfersResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::GetRPCMethodsResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::Inform(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::RequestDownload(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::ScheduleDownload(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::SetParameterAttributes(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::SetParameterValues(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(BodyElement::SetVouchers(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes)
                    }
                    Some(_unhandled) => { // the ones who dont need a start_handler, ie GetParameterValues aso
                    }
                    None => {
                        warn!(
                            "Element found under {}, but state list of bodies is empty",
                            body_element
                        );
                    }
                }
            }
            _ => {
                warn!("Unrecoqnized pattern");
            }
        }
    }

    fn end_handler(&mut self, path: &Vec<String>, _name: &xml::name::OwnedName) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            // match the ones who actually need and end_handler, and call their
            // respective end_handler
            _ => {}
        }
    }

    fn characters(&mut self, path: &Vec<String>, characters: &String) {
        // println!("Path: {:?} Chars: {}", path, characters);
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["Envelope", "Header", header_element] => {
                let last = self.header.last_mut();
                match last {
                    Some(HeaderElement::ID(data)) => {
                        if header_element == &"ID" {
                            data.id = characters.to_string()
                        }
                    }
                    Some(HeaderElement::NoMoreRequests(data)) => {
                        if header_element == &"NoMoreRequests" {
                            data.value = parse_to_int(characters, 0);
                        }
                    }
                    Some(HeaderElement::HoldRequests(data)) => {
                        if header_element == &"HoldRequests" {
                            data.hold = str2bool(characters);
                        }
                    }
                    Some(HeaderElement::SessionTimeout(data)) => {
                        if header_element == &"SessionTimeout" {
                            data.timeout = parse_to_int(characters, 0);
                        }
                    }
                    Some(HeaderElement::SupportedCWMPVersions(data)) => {
                        if header_element == &"SupportedCWMPVersions" {
                            data.value = characters.to_string()
                        }
                    }
                    Some(HeaderElement::UseCWMPVersion(data)) => {
                        if header_element == &"UseCWMPVersion" {
                            data.value = characters.to_string()
                        }
                    }
                    _ => {} // should never happen
                }
            }
            ["Envelope", "Body", body_element, ..] => {
                let last = self.body.last_mut();
                match last {
                    Some(BodyElement::AddObjectResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::AddObject(e)) => e.characters(&path_pattern[2..], characters),
                    Some(BodyElement::AutonomousDUStateChangeComplete(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::AutonomousTransferComplete(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::CancelTransfer(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::ChangeDUState(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::DeleteObjectResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::DeleteObject(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::DownloadResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::Download(e)) => e.characters(&path_pattern[2..], characters),
                    Some(BodyElement::DUStateChangeComplete(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::Fault(e)) => e.characters(&path_pattern[2..], characters),
                    Some(BodyElement::GetAllQueuedTransfersResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::GetOptionsResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::GetOptions(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::GetParameterAttributes(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::GetParameterAttributesResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::GetParameterNamesResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::GetParameterNames(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::GetParameterValues(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::GetParameterValuesResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::GetQueuedTransfersResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::GetRPCMethodsResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::InformResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::Inform(e)) => e.characters(&path_pattern[2..], characters),
                    Some(BodyElement::KickedResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::Kicked(e)) => e.characters(&path_pattern[2..], characters),
                    Some(BodyElement::Reboot(e)) => e.characters(&path_pattern[2..], characters),
                    Some(BodyElement::RequestDownload(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::ScheduleDownload(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::ScheduleInform(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::SetParameterAttributes(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::SetParameterValuesResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::SetParameterValues(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::SetVouchers(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::TransferComplete(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::UploadResponse(e)) => {
                        e.characters(&path_pattern[2..], characters)
                    }
                    Some(BodyElement::Upload(e)) => e.characters(&path_pattern[2..], characters),
                    Some(unhandled) => {
                        println!("characters for {:?} is so far unhandled", unhandled);
                    }
                    None => {
                        warn!(
                            "Element found under {}, but state list of bodies is empty",
                            body_element
                        );
                    }
                }
            }
            _ => {
                // cant find anywhere to stuff this text, ok...
            }
        }
    }
}

// private functions
fn extract_attribute(
    attributes: &Vec<xml::attribute::OwnedAttribute>,
    attrib_name: &str,
) -> String {
    let f = attributes
        .iter()
        .filter(|&x| x.name.local_name == attrib_name)
        .next();
    match f {
        Some(e) => e.value.to_string(),
        None => String::from(""),
    }
}

fn cwmp_prefix(envelope_has_cwmp_version: bool, postfix: &str) -> String {
    if envelope_has_cwmp_version {
        format!("cwmp:{}", postfix)
    } else {
        postfix.to_string()
    }
}

// parses urns like "urn:dslforum-org:cwmp-1-0" into
// CwmpVersion, i.e. (1,0) in this example
fn cwmp_urn_to_version(urn: &str) -> CwmpVersion {
    let mut version_string: Vec<&str> = urn.split("-").collect();
    let mi = if let Some(mi_s) = version_string.pop() {
        parse_to_int(&mi_s.to_string(), 0)
    } else {
        0
    };
    let ma = if let Some(ma_s) = version_string.pop() {
        parse_to_int(&ma_s.to_string(), 0)
    } else {
        0
    };

    CwmpVersion::new(ma, mi)
}

pub trait Parseable {}
impl Parseable for u32 {}
impl Parseable for u16 {}
impl Parseable for i32 {}
impl Parseable for u8 {}

fn parse_to_int<T: Parseable + std::str::FromStr>(chars: &String, default: T) -> T {
    match chars.parse::<T>() {
        Ok(parsed) => parsed,
        _ => default,
    }
}

impl State {
    pub fn new() -> Self {
        State {
            last_text: String::from(""),
            envelope: Envelope::default(),
            path: vec![],
            error: None,
        }
    }
    pub fn start_handler(
        &mut self,
        name: &xml::name::OwnedName,
        attributes: &Vec<xml::attribute::OwnedAttribute>,
        namespace: &xml::namespace::Namespace,
    ) {
        // push a copy of the name into the current path
        self.path.push(name.local_name.to_string());

        self.envelope
            .start_handler(&self.path, name, attributes, namespace);
    }

    pub fn end_handler(&mut self, name: &xml::name::OwnedName) {
        // pop the name from the current path
        self.path.pop();
        self.envelope.end_handler(&self.path, name);
    }
    pub fn characters(&mut self, characters: &String) {
        self.last_text = String::from(characters);
        self.envelope.characters(&self.path, characters);
    }
}

pub struct State {
    pub path: Vec<String>,
    pub last_text: String,
    pub envelope: Envelope,
    pub error: Option<Box<dyn Error>>,
}
