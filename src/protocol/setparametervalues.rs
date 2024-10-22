use std::io::Write;

use xml::writer::XmlEvent;

use super::{
    extract_attribute, write_empty_tag, write_simple, GenerateError, ParameterValue, XmlSafeString,
};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SetParameterValues {
    pub parameter_list: Vec<ParameterValue>,
    pub parameter_key: Option<XmlSafeString>,
}

impl SetParameterValues {
    #[must_use]
    pub fn new(parameter_key: Option<&str>, parameter_list: &[&ParameterValue]) -> Self {
        Self {
            parameter_list: parameter_list.iter().copied().cloned().collect(),
            parameter_key: parameter_key.map(XmlSafeString::from),
        }
    }

    /// Generate XML for `SetParameterValues`
    ///     
    /// # Errors
    ///
    /// Any errors encountered while writing to "writer" will be returned.
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
            write_simple(writer, "ParameterKey", pk.0.as_ref())?;
        }
        if self.parameter_list.is_empty() {
            write_empty_tag(writer, "ParameterList")?;
        } else {
            writer.write(
                XmlEvent::start_element("ParameterList").attr("SOAP-ENC:arrayType", &pvs[..]),
            )?;

            for p in &self.parameter_list {
                writer.write(XmlEvent::start_element("ParameterValueStruct"))?;
                write_simple(writer, "Name", p.name.0.as_ref())?;
                writer.write(
                    XmlEvent::start_element("Value").attr("xsi:type", p.r#type.0.as_ref()),
                )?;
                writer.write(p.value.0.as_ref())?;
                writer.write(XmlEvent::end_element())?; // Value
                writer.write(XmlEvent::end_element())?;
            }
            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    pub fn start_handler(
        &mut self,
        path: &[&str],
        _name: &xml::name::OwnedName,
        attributes: &[xml::attribute::OwnedAttribute],
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["SetParameterValues", "ParameterKey"] => {
                self.parameter_key = Some(XmlSafeString::new());
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
    pub fn characters(&mut self, path: &[&str], characters: &str) {
        match *path {
            ["SetParameterValues", "ParameterKey"] => {
                self.parameter_key = Some(characters.into());
            }
            ["SetParameterValues", "ParameterList", "ParameterValueStruct", key] => {
                if let Some(p) = self.parameter_list.last_mut() {
                    match key {
                        "Name" => p.name = characters.into(),
                        "Value" => p.value = characters.into(),
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
        Self {
            parameter_list: Vec::<ParameterValue>::arbitrary(g),
            parameter_key: Option::<XmlSafeString>::arbitrary(g).map(XmlSafeString::from),
        }
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
