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
    #[must_use]
    pub fn new(results: Vec<AutonOpResult>) -> Self {
        AutonomousDUStateChangeComplete { results }
    }
    /// Generate XML for `AutonomousDUStateChangeComplete`
    ///     
    /// # Errors
    ///
    /// Any errors encountered while writing to "writer" will be returned.
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

        for r in &self.results {
            writer.write(XmlEvent::start_element("AutonOpResultStruct"))?;

            write_simple(writer, "UUID", r.uuid.0.as_ref())?;
            write_simple(
                writer,
                "DeploymentUnitRef",
                r.deployment_unit_ref.0.as_ref(),
            )?;
            write_simple(writer, "Version", r.version.0.as_ref())?;
            write_simple(writer, "CurrentState", r.current_state.0.as_ref())?;
            write_simple(writer, "Resolved", r.resolved.0.as_ref())?;
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
            write_simple(
                writer,
                "OperationPerformed",
                r.operation_performed.0.as_ref(),
            )?;

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
        _attributes: &[xml::attribute::OwnedAttribute],
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        if let ["AutonomousDUStateChangeComplete", "Results", "AutonOpResultStruct"] =
            &path_pattern[..]
        {
            self.results.push(AutonOpResult::default());
        }
    }
    pub fn characters(&mut self, path: &[&str], characters: &str) {
        match *path {
            ["AutonomousDUStateChangeComplete", "Results", "AutonOpResultStruct", key] => {
                let last = self.results.last_mut();
                if let Some(e) = last {
                    match key {
                        "UUID" => e.uuid = characters.into(),
                        "DeploymentUnitRef" => e.deployment_unit_ref = characters.into(),
                        "Version" => e.version = characters.into(),
                        "CurrentState" => e.current_state = characters.into(),
                        "Resolved" => e.resolved = characters.into(),
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
                        "OperationPerformed" => e.operation_performed = characters.into(),
                        _ => {}
                    }
                }
            }
            ["AutonomousDUStateChangeComplete", "Results", "AutonOpResultStruct", "Fault", "FaultStruct", key] =>
            {
                let last = self.results.last_mut();
                if let Some(e) = last {
                    match key {
                        "FaultCode" => {
                            if let Ok(parsed) = characters.parse::<u32>() {
                                e.fault.set_code(parsed);
                            }
                        }
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
