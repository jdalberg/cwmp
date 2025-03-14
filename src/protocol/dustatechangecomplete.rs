use std::io::Write;

use chrono::{DateTime, Utc};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{
    cwmp_prefix, parse_to_int, write_fault, write_simple, GenerateError, OpResult, XmlSafeString,
};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct DUStateChangeComplete {
    pub command_key: XmlSafeString,
    pub results: Vec<OpResult>,
}

impl DUStateChangeComplete {
    #[must_use]
    pub fn new(command_key: &str, results: Vec<OpResult>) -> Self {
        DUStateChangeComplete {
            command_key: command_key.into(),
            results,
        }
    }

    /// Generate XML for `DUStateChangeComplete`
    ///     
    /// # Errors
    ///
    /// Any errors encountered while writing to "writer" will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "DUStateChangeComplete")[..],
        ))?;
        write_simple(writer, "CommandKey", self.command_key.0.as_ref())?;
        let ss = if has_cwmp {
            format!("cwmp:OpResultStruct[{}]", self.results.len())
        } else {
            format!("OpResultStruct[{}]", self.results.len())
        };

        writer.write(XmlEvent::start_element("Results").attr("SOAP-ENC:arrayType", &ss[..]))?;

        for r in &self.results {
            writer.write(XmlEvent::start_element("OpResultStruct"))?;
            write_simple(writer, "UUID", r.uuid.0.as_ref())?;
            write_simple(
                writer,
                "DeploymentUnitRef",
                r.deployment_unit_ref.0.as_ref(),
            )?;
            write_simple(writer, "Version", r.version.0.as_ref())?;
            write_simple(writer, "CurrentState", r.current_state.0.as_ref())?;
            write_simple(writer, "Resolved", &r.resolved.to_string())?;
            write_simple(
                writer,
                "ExecutionUnitRefList",
                r.execution_unit_ref_list.0.as_ref(),
            )?;
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

    pub fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &[xml::attribute::OwnedAttribute],
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        if let ["DUStateChangeComplete", "Results", "OpResultStruct"] = &path_pattern[..] {
            self.results.push(OpResult::default());
        }
    }

    pub fn characters(&mut self, path: &[&str], characters: &str) {
        match *path {
            ["DUStateChangeComplete", "CommandKey"] => self.command_key = characters.into(),
            ["DUStateChangeComplete", "Results", "OpResultStruct", key] => {
                if let Some(e) = self.results.last_mut() {
                    match key {
                        "UUID" => e.uuid = characters.into(),
                        "DeploymentUnitRef" => e.deployment_unit_ref = characters.into(),
                        "Version" => e.version = characters.into(),
                        "CurrentState" => e.current_state = characters.into(),
                        "Resolved" => e.resolved = parse_to_int(characters, 0),
                        "ExecutionUnitRefList" => {
                            e.execution_unit_ref_list = characters.into();
                        }
                        "StartTime" => {
                            if let Ok(dt) = characters.parse::<DateTime<Utc>>() {
                                e.start_time = Some(dt);
                            }
                        }
                        "CompleteTime" => {
                            if let Ok(dt) = characters.parse::<DateTime<Utc>>() {
                                e.complete_time = Some(dt);
                            }
                        }
                        _ => {}
                    }
                }
            }
            ["DUStateChangeComplete", "Results", "OpResultStruct", "Fault", "FaultStruct", key] => {
                if let Some(e) = self.results.last_mut() {
                    match key {
                        "FaultCode" => e.fault.set_code(parse_to_int(characters, 0)),
                        "FaultString" => e.fault.set_string(characters),
                        _ => {}
                    }
                }
            }

            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for DUStateChangeComplete {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            command_key: XmlSafeString::arbitrary(g),
            results: Vec::<OpResult>::arbitrary(g),
        }
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
