use std::io::Write;

use chrono::{DateTime, Utc};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{write_fault, write_simple, AutonOpResult, GenerateError};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct AutonomousDUStateChangeComplete {
    pub results: Vec<AutonOpResult>,
}

impl AutonomousDUStateChangeComplete {
    pub fn new(results: Vec<AutonOpResult>) -> Self {
        AutonomousDUStateChangeComplete { results }
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
    pub fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        if let ["AutonomousDUStateChangeComplete", "Results", "AutonOpResultStruct"] = &path_pattern[..] {
            self.results.push(AutonOpResult::default());
        }
    }
    pub fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["AutonomousDUStateChangeComplete", "Results", "AutonOpResultStruct", key] => {
                let last = self.results.last_mut();
                if let Some(e) = last { match key {
                    "UUID" => e.uuid = characters.to_string(),
                    "DeploymentUnitRef" => e.deployment_unit_ref = characters.to_string(),
                    "Version" => e.version = characters.to_string(),
                    "CurrentState" => e.current_state = characters.to_string(),
                    "Resolved" => e.resolved = characters.to_string(),
                    "ExecutionUnitRefList" => {
                        e.execution_unit_ref_list = characters.to_string()
                    }
                    "StartTime" => if let Ok(dt) = characters.parse::<DateTime<Utc>>() { e.start_time = Some(dt) },
                    "CompleteTime" => if let Ok(dt) = characters.parse::<DateTime<Utc>>() { e.complete_time = Some(dt) },
                    "OperationPerformed" => e.operation_performed = characters.to_string(),
                    _ => {}
                } }
            }
            ["AutonomousDUStateChangeComplete", "Results", "AutonOpResultStruct", "Fault", "FaultStruct", key] =>
            {
                let last = self.results.last_mut();
                if let Some(e) = last { match key {
                    "FaultCode" => if let Ok(parsed) = characters.parse::<u32>() { e.fault.set_code(parsed) },
                    "FaultString" => e.fault.set_string(&characters[..]),
                    _ => {}
                } }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for AutonomousDUStateChangeComplete {
    fn arbitrary(g: &mut Gen) -> Self {
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
