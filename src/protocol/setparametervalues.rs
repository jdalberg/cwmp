use std::io::Write;

use xml::writer::XmlEvent;

use super::{extract_attribute, write_empty_tag, write_simple, GenerateError, ParameterValue};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SetParameterValues {
    pub parameter_list: Vec<ParameterValue>,
    pub parameter_key: Option<String>,
}

impl SetParameterValues {
    #[must_use]
    pub fn new(parameter_key: Option<String>, parameter_list: Vec<ParameterValue>) -> Self {
        SetParameterValues {
            parameter_list,
            parameter_key,
        }
    }

    /// Generate XML for `SetParameterValues`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
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
        if self.parameter_list.is_empty() {
            write_empty_tag(writer, "ParameterList")?;
        } else {
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
    pub fn characters(&mut self, path: &[&str], characters: &String) {
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
