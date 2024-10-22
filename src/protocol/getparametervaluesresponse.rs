use super::{extract_attribute, write_simple, GenerateError, ParameterValue};
use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetParameterValuesResponse {
    pub parameters: Vec<ParameterValue>,
}

impl GetParameterValuesResponse {
    #[must_use]
    pub fn new(parameters: Vec<ParameterValue>) -> Self {
        GetParameterValuesResponse { parameters }
    }

    /// Generate XML for `GetParameterValuesResponse`
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

        for p in &self.parameters {
            writer.write(XmlEvent::start_element("ParameterValueStruct"))?;
            write_simple(writer, "Name", p.name.0.as_ref())?;
            writer.write(XmlEvent::start_element("Value").attr("xsi:type", p.r#type.0.as_ref()))?;
            writer.write(p.value.0.as_ref())?;
            writer.write(XmlEvent::end_element())?;

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
        attributes: &[xml::attribute::OwnedAttribute],
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["GetParameterValuesResponse", "ParameterList", "ParameterValueStruct"] => {
                self.parameters.push(ParameterValue::default());
            }
            ["GetParameterValuesResponse", "ParameterList", "ParameterValueStruct", "Value"] => {
                // use the type attribute
                if let Some(e) = self.parameters.last_mut() {
                    e.r#type = extract_attribute(attributes, "type");
                }
            }
            _ => {}
        }
    }
    pub fn characters(&mut self, path: &[&str], characters: &str) {
        match *path {
            ["GetParameterValuesResponse", "ParameterList", "ParameterValueStruct", "Name"] => {
                if let Some(e) = self.parameters.last_mut() {
                    e.name = characters.into();
                }
            }
            ["GetParameterValuesResponse", "ParameterList", "ParameterValueStruct", "Value"] => {
                if let Some(e) = self.parameters.last_mut() {
                    e.value = characters.into();
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetParameterValuesResponse {
    fn arbitrary(g: &mut Gen) -> Self {
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
