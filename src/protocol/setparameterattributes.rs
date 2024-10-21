use std::io::Write;

use xml::writer::XmlEvent;

use super::{
    cwmp_prefix, parse_to_int, setparameterattributesstruct::SetParameterAttributesStruct,
    write_simple, GenerateError, XmlSafeString,
};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SetParameterAttributes {
    pub parameter_list: Vec<SetParameterAttributesStruct>,
}
impl SetParameterAttributes {
    #[must_use]
    pub fn new(parameter_list: Vec<SetParameterAttributesStruct>) -> Self {
        SetParameterAttributes { parameter_list }
    }
    pub fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        _attributes: &[xml::attribute::OwnedAttribute],
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["SetParameterAttributes", "ParameterList", "SetParameterAttributesStruct"] => self
                .parameter_list
                .push(SetParameterAttributesStruct::default()),
            ["SetParameterAttributes", "ParameterList", "SetParameterAttributesStruct", "AccessList", "string"] => {
                if let Some(p) = self.parameter_list.last_mut() {
                    p.access_list.push(XmlSafeString::new());
                }
            }
            _ => {}
        }
    }

    /// Generate XML for `SetParameterAttributes`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
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
            write_simple(writer, "Name", p.name.0.as_ref())?;
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
                write_simple(writer, "string", al.0.as_ref())?;
            }
            writer.write(XmlEvent::end_element())?; // AccessList
            writer.write(XmlEvent::end_element())?; // SetParameterAttributesStruct
        }

        // ParameterList
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &str) {
        match *path {
            ["SetParameterAttributes", "ParameterList", "SetParameterAttributesStruct", "AccessList", "string"] => {
                if let Some(p) = self.parameter_list.last_mut() {
                    if let Some(a) = p.access_list.last_mut() {
                        *a = characters.into();
                    }
                }
            }
            ["SetParameterAttributes", "ParameterList", "SetParameterAttributesStruct", key] => {
                if let Some(e) = self.parameter_list.last_mut() {
                    match key {
                        "Name" => e.name = characters.into(),
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
