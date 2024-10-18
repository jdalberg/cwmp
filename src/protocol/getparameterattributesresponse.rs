use super::{write_simple, GenerateError, ParameterAttribute};
use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetParameterAttributesResponse {
    pub parameters: Vec<ParameterAttribute>,
}

impl GetParameterAttributesResponse {
    #[must_use]
    pub fn new(parameters: Vec<ParameterAttribute>) -> Self {
        GetParameterAttributesResponse { parameters }
    }
    /// Generate XML for `GetParameterAttributesResponse`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
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

        for p in &self.parameters {
            writer.write(XmlEvent::start_element("ParameterAttributeStruct"))?;
            write_simple(writer, "Name", &p.name)?;
            write_simple(writer, "Notification", &p.notification)?;
            let als = format!("xsd:string[{}]", p.accesslist.len());
            writer.write(
                XmlEvent::start_element("AccessList").attr("SOAP-ENC:arrayType", &als[..]),
            )?;

            for a in &p.accesslist {
                write_simple(writer, "string", a)?;
            }

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
        _attributes: &Vec<xml::attribute::OwnedAttribute>,
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        match &path_pattern[..] {
            ["GetParameterAttributesResponse", "ParameterList", "ParameterAttributeStruct"] => {
                self.parameters.push(ParameterAttribute::default());
            }
            ["GetParameterAttributesResponse", "ParameterList", "ParameterAttributeStruct", "AccessList", "string"] => {
                if let Some(e) = self.parameters.last_mut() {
                    e.accesslist.push(String::new());
                }
            }
            _ => {}
        }
    }
    pub fn characters(&mut self, path: &[&str], characters: &String) {
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
    fn arbitrary(g: &mut Gen) -> Self {
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
