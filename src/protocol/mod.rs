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
use chrono::{NaiveDate, NaiveDateTime};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
#[cfg(test)]
use rand::Rng;

mod addobject;
mod addobjectresponse;
mod allqueuedtransfers;
mod autonomousdustatechangecomplete;
mod autonomousdustatechangecompleteresponse;
mod autonomoustransfercomplete;
mod autonomoustransfercompleteresponse;
mod autonopresult;
mod canceltransfer;
mod canceltransferresponse;
mod changedustate;
mod changedustateresponse;
mod deleteobject;
mod deleteobjectresponse;
mod download;
mod downloadresponse;
mod dustatechangecomplete;
mod dustatechangecompleteresponse;
mod factoryreset;
mod factoryresetresponse;
mod fault;
mod getallqueuedtransfers;
mod getallqueuedtransfersresponse;
mod getoptions;
mod getoptionsresponse;
mod getparameterattributes;
mod getparameterattributesresponse;
mod getparameternames;
mod getparameternamesresponse;
mod getparametervalues;
mod getparametervaluesresponse;
mod getqueuedtransfers;
mod getqueuedtransfersresponse;
mod getrpcmethods;
mod getrpcmethodsresponse;
mod headerelement;
mod holdrequests;
mod id;
mod installop;
mod nomorerequests;
mod opresult;
mod optionstruct;
mod parameterattribute;
mod parameterinfostruct;
mod parametervalue;
mod queuedtransferstruct;
mod sessiontimeout;
mod supportedcwmpversions;
mod uninstallop;
mod updateop;
mod upload;
mod usecwmpversion;

pub use addobject::AddObject;
pub use addobjectresponse::AddObjectResponse;
pub use allqueuedtransfers::AllQueuedTransfers;
pub use autonomousdustatechangecomplete::AutonomousDUStateChangeComplete;
pub use autonomousdustatechangecompleteresponse::AutonomousDUStateChangeCompleteResponse;
pub use autonomoustransfercomplete::AutonomousTransferComplete;
pub use autonomoustransfercompleteresponse::AutonomousTransferCompleteResponse;
pub use autonopresult::AutonOpResult;
pub use canceltransfer::CancelTransfer;
pub use canceltransferresponse::CancelTransferResponse;
pub use changedustate::ChangeDUState;
pub use changedustateresponse::ChangeDUStateResponse;
pub use deleteobject::DeleteObject;
pub use deleteobjectresponse::DeleteObjectResponse;
pub use download::Download;
pub use downloadresponse::DownloadResponse;
pub use dustatechangecomplete::DUStateChangeComplete;
pub use dustatechangecompleteresponse::DUStateChangeCompleteResponse;
pub use factoryreset::FactoryReset;
pub use factoryresetresponse::FactoryResetResponse;
pub use fault::{Fault, FaultDetail, FaultStruct};
pub use getallqueuedtransfers::GetAllQueuedTransfers;
pub use getallqueuedtransfersresponse::GetAllQueuedTransfersResponse;
pub use getoptions::GetOptions;
pub use getoptionsresponse::GetOptionsResponse;
pub use getparameterattributes::GetParameterAttributes;
pub use getparameterattributesresponse::GetParameterAttributesResponse;
pub use getparameternames::GetParameterNames;
pub use getparameternamesresponse::GetParameterNamesResponse;
pub use getparametervalues::GetParameterValues;
pub use getparametervaluesresponse::GetParameterValuesResponse;
pub use getqueuedtransfers::GetQueuedTransfers;
pub use getqueuedtransfersresponse::GetQueuedTransfersResponse;
pub use getrpcmethods::GetRPCMethods;
pub use getrpcmethodsresponse::GetRPCMethodsResponse;
pub use headerelement::HeaderElement;
pub use holdrequests::HoldRequests;
pub use id::ID;
pub use installop::InstallOp;
pub use nomorerequests::NoMoreRequests;
pub use opresult::OpResult;
pub use optionstruct::OptionStruct;
pub use parameterattribute::ParameterAttribute;
pub use parameterinfostruct::ParameterInfoStruct;
pub use parametervalue::ParameterValue;
pub use queuedtransferstruct::QueuedTransferStruct;
pub use sessiontimeout::SessionTimeout;
pub use supportedcwmpversions::SupportedCWMPVersions;
pub use uninstallop::UninstallOp;
pub use updateop::UpdateOp;
pub use upload::Upload;
pub use usecwmpversion::UseCWMPVersion;

fn bool2str(b: bool) -> &'static str {
    if b {
        "1"
    } else {
        "0"
    }
}
fn str2bool(s: &str) -> bool {
    s != "0"
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

#[cfg(test)]
pub fn gen_utc_date(year: i32, mon: u32, day: u32, hour: u32, min: u32, sec: u32) -> DateTime<Utc> {
    NaiveDate::from_ymd_opt(year, mon, day)
        .unwrap_or(NaiveDate::default())
        .and_hms_opt(hour, min, sec)
        .unwrap_or(NaiveDateTime::default())
        .and_utc()
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct InformResponse {
    max_envelopes: u16,
}

impl InformResponse {
    #[must_use]
    pub fn new(max_envelopes: u16) -> Self {
        InformResponse { max_envelopes }
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
        if let ["InformResponse", "MaxEnvelopes"] = *path {
            self.max_envelopes = parse_to_int(characters, 1);
        }
    }
}

#[cfg(test)]
impl Arbitrary for InformResponse {
    fn arbitrary(g: &mut Gen) -> Self {
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
    #[must_use]
    pub fn new(
        manufacturer: String,
        oui: String,
        product_class: String,
        serial_number: String,
    ) -> Self {
        DeviceId {
            manufacturer,
            oui,
            product_class,
            serial_number,
        }
    }
}

#[cfg(test)]
impl Arbitrary for DeviceId {
    fn arbitrary(g: &mut Gen) -> Self {
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
    #[must_use]
    pub fn new(event_code: String, command_key: String) -> Self {
        EventStruct {
            event_code,
            command_key,
        }
    }
}

#[cfg(test)]
impl Arbitrary for EventStruct {
    fn arbitrary(g: &mut Gen) -> Self {
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
    #[must_use]
    pub fn new(
        device_id: DeviceId,
        event: Vec<EventStruct>,
        max_envelopes: u32,
        current_time: DateTime<Utc>,
        retry_count: u32,
        parameter_list: Vec<ParameterValue>,
    ) -> Self {
        Inform {
            device_id,
            event,
            max_envelopes,
            current_time: Some(current_time),
            retry_count,
            parameter_list,
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

        for e in &self.event {
            writer.write(XmlEvent::start_element("EventStruct"))?;
            write_simple(writer, "EventCode", &e.event_code)?;
            write_simple(writer, "CommandKey", &e.command_key)?;
            writer.write(XmlEvent::end_element())?;
        }
        // Event
        writer.write(XmlEvent::end_element())?;

        write_simple(writer, "MaxEnvelopes", &self.max_envelopes.to_string())?;
        if let Some(dt) = self.current_time {
            write_simple(writer, "CurrentTime", &dt.to_rfc3339())?;
        }
        write_simple(writer, "RetryCount", &self.retry_count.to_string())?;

        let pls = format!("cwmp:ParameterValueStruct[{}]", self.parameter_list.len());
        writer
            .write(XmlEvent::start_element("ParameterList").attr("SOAP-ENC:arrayType", &pls[..]))?;

        for p in &self.parameter_list {
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
                self.parameter_list.push(ParameterValue::default());
            }
            ["Inform", "ParameterList", "ParameterValueStruct", "Value"] => {
                // use the type attribute
                if let Some(e) = self.parameter_list.last_mut() {
                    e.r#type = extract_attribute(attributes, "type");
                }
            }
            _ => {}
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["Inform", "DeviceId", "Manufacturer"] => {
                self.device_id.manufacturer = characters.to_string();
            }
            ["Inform", "DeviceId", "OUI"] => self.device_id.oui = characters.to_string(),
            ["Inform", "DeviceId", "ProductClass"] => {
                self.device_id.product_class = characters.to_string();
            }
            ["Inform", "DeviceId", "SerialNumber"] => {
                self.device_id.serial_number = characters.to_string();
            }
            ["Inform", "Event", "EventStruct", key] => {
                if let Some(e) = self.event.last_mut() {
                    match key {
                        "EventCode" => e.event_code = characters.to_string(),
                        "CommandKey" => e.command_key = characters.to_string(),
                        _ => {}
                    }
                }
            }
            ["Inform", "MaxEnvelopes"] => self.max_envelopes = parse_to_int(characters, 0),
            ["Inform", "RetryCount"] => self.retry_count = parse_to_int(characters, 0),
            ["Inform", "CurrentTime"] => {
                if let Ok(dt) = characters.parse::<DateTime<Utc>>() {
                    self.current_time = Some(dt);
                }
            }
            ["Inform", "ParameterList", "ParameterValueStruct", "Name"] => {
                if let Some(p) = self.parameter_list.last_mut() {
                    p.name = characters.to_string();
                }
            }
            ["Inform", "ParameterList", "ParameterValueStruct", "Value"] => {
                if let Some(p) = self.parameter_list.last_mut() {
                    p.value = characters.to_string();
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for Inform {
    fn arbitrary(g: &mut Gen) -> Self {
        Inform::new(
            DeviceId::arbitrary(g),
            Vec::<EventStruct>::arbitrary(g),
            u32::arbitrary(g),
            gen_utc_date(2014, 11, 28, 12, 0, 9),
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
                    current_time: Some(gen_utc_date(2014, 11, 28, 12, 0, 9)),
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
    #[must_use]
    pub fn new(next_url: String) -> Self {
        KickedResponse { next_url }
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
        if let ["KickedResponse", "NextURL"] = *path {
            self.next_url = characters.to_string();
        }
    }
}

#[cfg(test)]
impl Arbitrary for KickedResponse {
    fn arbitrary(g: &mut Gen) -> Self {
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
    #[must_use]
    pub fn new(command: String, referer: String, arg: String, next: String) -> Self {
        Kicked {
            command,
            referer,
            arg,
            next,
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
    fn arbitrary(g: &mut Gen) -> Self {
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
    #[must_use]
    pub fn new(command_key: String) -> Self {
        Reboot { command_key }
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
        if let ["Reboot", "CommandKey"] = *path {
            self.command_key = characters.to_string();
        }
    }
}

#[cfg(test)]
impl Arbitrary for Reboot {
    fn arbitrary(g: &mut Gen) -> Self {
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
    #[must_use]
    pub fn new(name: String, value: String) -> Self {
        ArgStruct { name, value }
    }
}

#[cfg(test)]
impl Arbitrary for ArgStruct {
    fn arbitrary(g: &mut Gen) -> Self {
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
    #[must_use]
    pub fn new(file_type: String, file_type_arg: Vec<ArgStruct>) -> Self {
        RequestDownload {
            file_type,
            file_type_arg,
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

        for a in &self.file_type_arg {
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
        if let ["RequestDownload", "FileTypeArg", "ArgStruct"] = &path_pattern[..] {
            self.file_type_arg.push(ArgStruct::default());
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["RequestDownload", "FileType"] => {
                self.file_type = characters.to_string();
            }
            ["RequestDownload", "FileTypeArg", "ArgStruct", key] => {
                if let Some(e) = self.file_type_arg.last_mut() {
                    match key {
                        "Name" => e.name = characters.to_string(),
                        "Value" => e.value = characters.to_string(),
                        _ => {}
                    }
                }
            }

            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for RequestDownload {
    fn arbitrary(g: &mut Gen) -> Self {
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
    #[must_use]
    pub fn new(
        window_start: u32,
        window_end: u32,
        window_mode: String,
        user_message: String,
        max_retries: i32,
    ) -> Self {
        TimeWindow {
            window_start,
            window_end,
            window_mode,
            user_message,
            max_retries,
        }
    }
}

#[cfg(test)]
impl Arbitrary for TimeWindow {
    fn arbitrary(g: &mut Gen) -> Self {
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
    #[must_use]
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
            command_key,
            file_type,
            url,
            username,
            password,
            file_size,
            target_filename,
            timewindow_list,
        }
    }
    fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        if let ["ScheduleDownload", "TimeWindowList", "TimeWindowStruct"] = &path_pattern[..] {
            self.timewindow_list.push(TimeWindow::default());
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

        for t in &self.timewindow_list {
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
                if let Some(e) = self.timewindow_list.last_mut() {
                    match key {
                        "WindowStart" => e.window_start = parse_to_int(characters, 0),
                        "WindowEnd" => e.window_end = parse_to_int(characters, 0),
                        "WindowMode" => e.window_mode = characters.to_string(),
                        "UserMessage" => e.user_message = characters.to_string(),
                        "MaxRetries" => e.max_retries = parse_to_int(characters, 0),
                        _ => {}
                    }
                }
            }

            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for ScheduleDownload {
    fn arbitrary(g: &mut Gen) -> Self {
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
    #[must_use]
    pub fn new(delay_seconds: u32, command_key: String) -> Self {
        ScheduleInform {
            delay_seconds,
            command_key,
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
    fn arbitrary(g: &mut Gen) -> Self {
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
    #[must_use]
    pub fn new(
        name: String,
        notification_change: u8,
        notification: u8,
        access_list_change: u8,
        access_list: Vec<String>,
    ) -> Self {
        SetParameterAttributesStruct {
            name,
            notification_change,
            notification,
            access_list_change,
            access_list,
        }
    }
}

#[cfg(test)]
impl Arbitrary for SetParameterAttributesStruct {
    fn arbitrary(g: &mut Gen) -> Self {
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
    #[must_use]
    pub fn new(parameter_list: Vec<SetParameterAttributesStruct>) -> Self {
        SetParameterAttributes { parameter_list }
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
                    p.access_list.push(String::new());
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

        for p in &self.parameter_list {
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
            for al in &p.access_list {
                write_simple(writer, "string", al)?;
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
    fn arbitrary(g: &mut Gen) -> Self {
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
    #[must_use]
    pub fn new(status: u32) -> Self {
        SetParameterValuesResponse { status }
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
        if let ["SetParameterValuesResponse", "Status"] = *path {
            self.status = parse_to_int(characters, 0);
        }
    }
}

#[cfg(test)]
impl Arbitrary for SetParameterValuesResponse {
    fn arbitrary(g: &mut Gen) -> Self {
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
    #[must_use]
    pub fn new(parameter_key: Option<String>, parameter_list: Vec<ParameterValue>) -> Self {
        SetParameterValues {
            parameter_list,
            parameter_key,
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
            write_simple(writer, "ParameterKey", pk)?;
        }
        if !self.parameter_list.is_empty() {
            writer.write(
                XmlEvent::start_element("ParameterList").attr("SOAP-ENC:arrayType", &pvs[..]),
            )?;

            for p in &self.parameter_list {
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
                self.parameter_key = Some(String::new());
            }
            ["SetParameterValues", "ParameterList", "ParameterValueStruct"] => {
                self.parameter_list.push(ParameterValue::default());
            }
            ["SetParameterValues", "ParameterList", "ParameterValueStruct", "Value"] => {
                // use the type attribute
                if let Some(p) = self.parameter_list.last_mut() {
                    p.r#type = extract_attribute(attributes, "type");
                }
            }
            _ => {}
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["SetParameterValues", "ParameterKey"] => {
                self.parameter_key = Some(characters.to_string());
            }
            ["SetParameterValues", "ParameterList", "ParameterValueStruct", key] => {
                if let Some(p) = self.parameter_list.last_mut() {
                    match key {
                        "Name" => p.name = characters.to_string(),
                        "Value" => p.value = characters.to_string(),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for SetParameterValues {
    fn arbitrary(g: &mut Gen) -> Self {
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
    #[must_use]
    pub fn new(voucher_list: Vec<String>) -> Self {
        SetVouchers { voucher_list }
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

        for v in &self.voucher_list {
            write_simple(writer, "base64", v)?;
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
        if let ["SetVouchers", "VoucherList", "base64"] = &path_pattern[..] {
            self.voucher_list.push(String::new());
        }
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        if let ["SetVouchers", "VoucherList", "base64"] = *path {
            if let Some(v) = self.voucher_list.last_mut() {
                *v = characters.to_string();
            }
        }
    }
}

#[cfg(test)]
impl Arbitrary for SetVouchers {
    fn arbitrary(g: &mut Gen) -> Self {
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
    #[must_use]
    pub fn new(
        command_key: String,
        fault: FaultStruct,
        start_time: Option<DateTime<Utc>>,
        complete_time: Option<DateTime<Utc>>,
    ) -> Self {
        TransferComplete {
            command_key: command_key.to_string(),
            fault,
            start_time,
            complete_time,
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
        if let Some(dt) = self.start_time {
            write_simple(writer, "StartTime", &dt.to_rfc3339())?;
        }
        if let Some(dt) = self.complete_time {
            write_simple(writer, "CompleteTime", &dt.to_rfc3339())?;
        }

        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["TransferComplete", "CommandKey"] => self.command_key = characters.to_string(),
            ["TransferComplete", "StartTime"] => {
                if let Ok(dt) = characters.parse::<DateTime<Utc>>() {
                    self.start_time = Some(dt);
                }
            }
            ["TransferComplete", "CompleteTime"] => {
                if let Ok(dt) = characters.parse::<DateTime<Utc>>() {
                    self.complete_time = Some(dt);
                }
            }
            ["TransferComplete", "FaultStruct", "FaultCode"] => {
                self.fault.set_code(parse_to_int(characters, 0));
            }
            ["TransferComplete", "FaultStruct", "FaultString"] => self.fault.set_string(characters),

            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for TransferComplete {
    fn arbitrary(g: &mut Gen) -> Self {
        TransferComplete::new(
            String::arbitrary(g),
            FaultStruct::arbitrary(g),
            Some(gen_utc_date(2014, 11, 28, 12, 0, 9)),
            Some(gen_utc_date(2014, 11, 29, 12, 0, 9)),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.command_key.clone(), self.fault.clone())
                .shrink()
                .map(|(c, f)| TransferComplete {
                    command_key: c,
                    fault: f,
                    start_time: Some(gen_utc_date(2014, 11, 28, 12, 0, 9)),
                    complete_time: Some(gen_utc_date(2014, 11, 29, 12, 0, 9)),
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
    #[must_use]
    pub fn new(
        status: u8,
        start_time: Option<DateTime<Utc>>,
        complete_time: Option<DateTime<Utc>>,
    ) -> Self {
        UploadResponse {
            status,
            start_time,
            complete_time,
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
        if let Some(dt) = self.start_time {
            write_simple(writer, "StartTime", &dt.to_rfc3339())?;
        }
        if let Some(dt) = self.complete_time {
            write_simple(writer, "CompleteTime", &dt.to_rfc3339())?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["UploadResponse", "Status"] => self.status = parse_to_int(characters, 0),
            ["UploadResponse", "StartTime"] => {
                if let Ok(dt) = characters.parse::<DateTime<Utc>>() {
                    self.start_time = Some(dt);
                }
            }
            ["UploadResponse", "CompleteTime"] => {
                if let Ok(dt) = characters.parse::<DateTime<Utc>>() {
                    self.complete_time = Some(dt);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for UploadResponse {
    fn arbitrary(g: &mut Gen) -> Self {
        UploadResponse::new(
            u8::arbitrary(g),
            Some(gen_utc_date(2014, 11, 28, 12, 0, 9)),
            Some(gen_utc_date(2014, 11, 29, 12, 0, 9)),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.status.clone().shrink().map(|s| UploadResponse {
            status: s,
            start_time: Some(gen_utc_date(2014, 11, 28, 12, 0, 9)),
            complete_time: Some(gen_utc_date(2014, 11, 29, 12, 0, 9)),
        }))
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
    fn arbitrary(g: &mut Gen) -> Self {
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
        let mut rng = rand::thread_rng();
        let idxs = std::ops::Range {
            start: 0,
            end: vals.len() - 1,
        };
        let random_index: usize = rng.gen_range(idxs);
        match vals.get(random_index) {
            Some(v) => v.clone(),
            None => BodyElement::AddObjectResponse(AddObjectResponse::arbitrary(g)),
        }
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
    #[must_use]
    pub fn new(major: u8, minor: u8) -> Self {
        CwmpVersion { major, minor }
    }
}

#[cfg(test)]
impl Arbitrary for CwmpVersion {
    fn arbitrary(g: &mut Gen) -> Self {
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
    fn arbitrary(g: &mut Gen) -> Envelope {
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
    #[must_use]
    pub fn new(
        cwmp_version: Option<CwmpVersion>,
        header: Vec<HeaderElement>,
        body: Vec<BodyElement>,
    ) -> Self {
        Envelope {
            cwmp_version,
            header,
            body,
        }
    }
    #[must_use]
    pub fn cwmp_version(self) -> Option<CwmpVersion> {
        self.cwmp_version
    }
    #[must_use]
    pub fn header(self) -> Vec<HeaderElement> {
        self.header
    }
    #[must_use]
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

        for he in &self.header {
            match he {
                HeaderElement::ID(e) => e.generate(&mut writer, self.cwmp_version.is_some())?,
                HeaderElement::HoldRequests(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                HeaderElement::NoMoreRequests(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                HeaderElement::SessionTimeout(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                HeaderElement::SupportedCWMPVersions(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                HeaderElement::UseCWMPVersion(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
            };
        }

        let end_header: XmlEvent = XmlEvent::end_element().into();
        writer.write(end_header)?;

        // now generate the body elemenets
        let body_start = XmlEvent::start_element("SOAP-ENV:Body");
        writer.write(body_start)?;

        for be in &self.body {
            match be {
                BodyElement::AddObject(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::AddObjectResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::AutonomousDUStateChangeComplete(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::AutonomousDUStateChangeCompleteResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::AutonomousTransferComplete(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::AutonomousTransferCompleteResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::CancelTransferResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::CancelTransfer(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::ChangeDUStateResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::ChangeDUState(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::DeleteObjectResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::DeleteObject(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::DownloadResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::Download(e) => e.generate(&mut writer, self.cwmp_version.is_some())?,
                BodyElement::DUStateChangeCompleteResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::DUStateChangeComplete(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::FactoryResetResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::FactoryReset(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::Fault(e) => e.generate(&mut writer, self.cwmp_version.is_some())?,
                BodyElement::GetAllQueuedTransfersResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetAllQueuedTransfers(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetOptionsResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetOptions(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetParameterAttributes(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetParameterAttributesResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetParameterNamesResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetParameterNames(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetParameterValues(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetParameterValuesResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetQueuedTransfersResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetQueuedTransfers(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetRPCMethodsResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::GetRPCMethods(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::InformResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::Inform(e) => e.generate(&mut writer, self.cwmp_version.is_some())?,
                BodyElement::KickedResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::Kicked(e) => e.generate(&mut writer, self.cwmp_version.is_some())?,
                BodyElement::RebootResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::Reboot(e) => e.generate(&mut writer, self.cwmp_version.is_some())?,
                BodyElement::RequestDownloadResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::RequestDownload(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::ScheduleDownloadResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::ScheduleDownload(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::ScheduleInformResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::ScheduleInform(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::SetParameterAttributesResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::SetParameterAttributes(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::SetParameterValuesResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::SetParameterValues(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::SetVouchersResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::SetVouchers(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::TransferCompleteResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::TransferComplete(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
                }
                BodyElement::Upload(e) => e.generate(&mut writer, self.cwmp_version.is_some())?,
                BodyElement::UploadResponse(e) => {
                    e.generate(&mut writer, self.cwmp_version.is_some())?;
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
                    .find(|&x| x.name.local_name == "mustUnderstand");

                let must_understand: bool = if let Some(mua) = must_understand_filter {
                    str2bool(&mua.value.to_string())
                } else {
                    true
                };
                match *header_element {
                    "ID" => self.header.push(HeaderElement::ID(ID {
                        must_understand,
                        id: String::new(),
                    })),
                    "NoMoreRequests" => {
                        self.header
                            .push(HeaderElement::NoMoreRequests(NoMoreRequests::new(
                                must_understand,
                                0,
                            )));
                    }
                    "HoldRequests" => {
                        self.header
                            .push(HeaderElement::HoldRequests(HoldRequests::new(
                                must_understand,
                                false,
                            )));
                    }
                    "SessionTimeout" => {
                        self.header
                            .push(HeaderElement::SessionTimeout(SessionTimeout::new(
                                must_understand,
                                0,
                            )));
                    }
                    "SupportedCWMPVersions" => {
                        self.header.push(HeaderElement::SupportedCWMPVersions(
                            SupportedCWMPVersions::new(must_understand, String::new()),
                        ));
                    }
                    "UseCWMPVersion" => {
                        self.header
                            .push(HeaderElement::UseCWMPVersion(UseCWMPVersion::new(
                                must_understand,
                                String::new(),
                            )));
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
                        "AddObject" => self.body.push(BodyElement::AddObject(AddObject::default())),
                        "AddObjectResponse" => self
                            .body
                            .push(BodyElement::AddObjectResponse(AddObjectResponse::default())),
                        "AutonomousDUStateChangeCompleteResponse" => {
                            self.body
                                .push(BodyElement::AutonomousDUStateChangeCompleteResponse(
                                    AutonomousDUStateChangeCompleteResponse {},
                                ));
                        }
                        "AutonomousDUStateChangeComplete" => {
                            self.body.push(BodyElement::AutonomousDUStateChangeComplete(
                                AutonomousDUStateChangeComplete::default(),
                            ));
                        }
                        "AutonomousTransferCompleteResponse" => {
                            self.body
                                .push(BodyElement::AutonomousTransferCompleteResponse(
                                    AutonomousTransferCompleteResponse {},
                                ));
                        }
                        "AutonomousTransferComplete" => {
                            self.body.push(BodyElement::AutonomousTransferComplete(
                                AutonomousTransferComplete::default(),
                            ));
                        }
                        "CancelTransferResponse" => self.body.push(
                            BodyElement::CancelTransferResponse(CancelTransferResponse {}),
                        ),
                        "CancelTransfer" => self
                            .body
                            .push(BodyElement::CancelTransfer(CancelTransfer::default())),
                        "ChangeDUStateResponse" => self
                            .body
                            .push(BodyElement::ChangeDUStateResponse(ChangeDUStateResponse {})),
                        "ChangeDUState" => self
                            .body
                            .push(BodyElement::ChangeDUState(ChangeDUState::default())),
                        "DeleteObjectResponse" => self.body.push(
                            BodyElement::DeleteObjectResponse(DeleteObjectResponse::default()),
                        ),
                        "DeleteObject" => self
                            .body
                            .push(BodyElement::DeleteObject(DeleteObject::default())),
                        "DownloadResponse" => self
                            .body
                            .push(BodyElement::DownloadResponse(DownloadResponse::default())),
                        "Download" => self.body.push(BodyElement::Download(Download::default())),
                        "DUStateChangeCompleteResponse" => {
                            self.body.push(BodyElement::DUStateChangeCompleteResponse(
                                DUStateChangeCompleteResponse {},
                            ));
                        }
                        "DUStateChangeComplete" => self.body.push(
                            BodyElement::DUStateChangeComplete(DUStateChangeComplete::default()),
                        ),
                        "FactoryResetResponse" => self
                            .body
                            .push(BodyElement::FactoryResetResponse(FactoryResetResponse {})),
                        "FactoryReset" => {
                            self.body.push(BodyElement::FactoryReset(FactoryReset {}));
                        }
                        "Fault" => self.body.push(BodyElement::Fault(Fault::default())),
                        "GetAllQueuedTransfersResponse" => {
                            self.body.push(BodyElement::GetAllQueuedTransfersResponse(
                                GetAllQueuedTransfersResponse::default(),
                            ));
                        }
                        "GetAllQueuedTransfers" => self
                            .body
                            .push(BodyElement::GetAllQueuedTransfers(GetAllQueuedTransfers {})),
                        "GetOptionsResponse" => self.body.push(BodyElement::GetOptionsResponse(
                            GetOptionsResponse::default(),
                        )),
                        "GetOptions" => self.body.push(BodyElement::GetOptions(Default::default())),
                        "GetParameterAttributes" => self.body.push(
                            BodyElement::GetParameterAttributes(GetParameterAttributes::default()),
                        ),
                        "GetParameterAttributesResponse" => {
                            self.body.push(BodyElement::GetParameterAttributesResponse(
                                GetParameterAttributesResponse::default(),
                            ));
                        }
                        "GetParameterNamesResponse" => {
                            self.body.push(BodyElement::GetParameterNamesResponse(
                                GetParameterNamesResponse::default(),
                            ));
                        }
                        "GetParameterNames" => self
                            .body
                            .push(BodyElement::GetParameterNames(GetParameterNames::default())),
                        "GetParameterValues" => self.body.push(BodyElement::GetParameterValues(
                            GetParameterValues::default(),
                        )),
                        "GetParameterValuesResponse" => {
                            self.body.push(BodyElement::GetParameterValuesResponse(
                                GetParameterValuesResponse::default(),
                            ));
                        }
                        "GetQueuedTransfersResponse" => {
                            self.body.push(BodyElement::GetQueuedTransfersResponse(
                                GetQueuedTransfersResponse::default(),
                            ));
                        }
                        "GetQueuedTransfers" => self
                            .body
                            .push(BodyElement::GetQueuedTransfers(GetQueuedTransfers {})),
                        "GetRPCMethodsResponse" => self.body.push(
                            BodyElement::GetRPCMethodsResponse(GetRPCMethodsResponse::default()),
                        ),
                        "GetRPCMethods" => {
                            self.body.push(BodyElement::GetRPCMethods(GetRPCMethods {}));
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
                            ));
                        }
                        "SetParameterAttributes" => self.body.push(
                            BodyElement::SetParameterAttributes(SetParameterAttributes::default()),
                        ),
                        "SetParameterValuesResponse" => {
                            self.body.push(BodyElement::SetParameterValuesResponse(
                                SetParameterValuesResponse::default(),
                            ));
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
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::GetParameterAttributesResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::GetParameterAttributes(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::GetParameterValuesResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::ChangeDUState(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::DUStateChangeComplete(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::GetAllQueuedTransfersResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::GetOptionsResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::GetParameterNamesResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::GetParameterValues(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::GetQueuedTransfersResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::GetRPCMethodsResponse(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::Inform(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::RequestDownload(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::ScheduleDownload(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::SetParameterAttributes(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::SetParameterValues(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
                    }
                    Some(BodyElement::SetVouchers(e)) => {
                        e.start_handler(&path_pattern[2..], name, attributes);
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

    // TODO: todo!("// match the ones who actually need and end_handler, and call their respective end_handler");
    fn end_handler(&mut self, path: &Vec<String>, _name: &xml::name::OwnedName) {
        let _path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        {}
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
                            data.id = characters.to_string();
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
                            data.value = characters.to_string();
                        }
                    }
                    Some(HeaderElement::UseCWMPVersion(data)) => {
                        if header_element == &"UseCWMPVersion" {
                            data.value = characters.to_string();
                        }
                    }
                    _ => {} // should never happen
                }
            }
            ["Envelope", "Body", body_element, ..] => {
                let last = self.body.last_mut();
                match last {
                    Some(BodyElement::AddObjectResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::AddObject(e)) => e.characters(&path_pattern[2..], characters),
                    Some(BodyElement::AutonomousDUStateChangeComplete(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::AutonomousTransferComplete(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::CancelTransfer(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::ChangeDUState(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::DeleteObjectResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::DeleteObject(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::DownloadResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::Download(e)) => e.characters(&path_pattern[2..], characters),
                    Some(BodyElement::DUStateChangeComplete(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::Fault(e)) => e.characters(&path_pattern[2..], characters),
                    Some(BodyElement::GetAllQueuedTransfersResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::GetOptionsResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::GetOptions(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::GetParameterAttributes(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::GetParameterAttributesResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::GetParameterNamesResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::GetParameterNames(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::GetParameterValues(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::GetParameterValuesResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::GetQueuedTransfersResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::GetRPCMethodsResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::InformResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::Inform(e)) => e.characters(&path_pattern[2..], characters),
                    Some(BodyElement::KickedResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::Kicked(e)) => e.characters(&path_pattern[2..], characters),
                    Some(BodyElement::Reboot(e)) => e.characters(&path_pattern[2..], characters),
                    Some(BodyElement::RequestDownload(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::ScheduleDownload(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::ScheduleInform(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::SetParameterAttributes(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::SetParameterValuesResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::SetParameterValues(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::SetVouchers(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::TransferComplete(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::UploadResponse(e)) => {
                        e.characters(&path_pattern[2..], characters);
                    }
                    Some(BodyElement::Upload(e)) => e.characters(&path_pattern[2..], characters),
                    Some(unhandled) => {
                        println!("characters for {unhandled:?} is so far unhandled");
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
        .find(|&x| x.name.local_name == attrib_name);
    match f {
        Some(e) => e.value.to_string(),
        None => String::new(),
    }
}

fn cwmp_prefix(envelope_has_cwmp_version: bool, postfix: &str) -> String {
    if envelope_has_cwmp_version {
        format!("cwmp:{postfix}")
    } else {
        postfix.to_string()
    }
}

// parses urns like "urn:dslforum-org:cwmp-1-0" into
// CwmpVersion, i.e. (1,0) in this example
fn cwmp_urn_to_version(urn: &str) -> CwmpVersion {
    let mut version_string: Vec<&str> = urn.split('-').collect();
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

fn parse_to_int<T: Parseable + std::str::FromStr>(chars: &str, default: T) -> T {
    match chars.parse::<T>() {
        Ok(parsed) => parsed,
        _ => default,
    }
}

impl State {
    #[must_use]
    pub fn new() -> Self {
        State {
            last_text: String::new(),
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

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

pub struct State {
    pub path: Vec<String>,
    pub last_text: String,
    pub envelope: Envelope,
    pub error: Option<Box<dyn Error>>,
}
