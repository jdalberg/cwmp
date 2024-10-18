use super::{cwmp_prefix, write_simple, GenerateError};
use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetParameterValues {
    parameternames: Vec<String>,
}

impl GetParameterValues {
    #[must_use]
    pub fn new(parameternames: Vec<String>) -> Self {
        GetParameterValues { parameternames }
    }

    /// Generate XML for `GetParameterValues`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "GetParameterValues")[..],
        ))?;
        writer.write(XmlEvent::start_element("ParameterNames"))?;
        for p in &self.parameternames {
            write_simple(writer, "string", p)?;
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
        if let ["GetParameterValues", "ParameterNames", "string"] = &path_pattern[..] {
            self.parameternames.push(String::new());
        }
    }
    pub fn characters(&mut self, path: &[&str], characters: &String) {
        if let ["GetParameterValues", "ParameterNames", "string"] = *path {
            if let Some(l) = self.parameternames.last_mut() {
                *l = characters.to_string();
            }
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetParameterValues {
    fn arbitrary(g: &mut Gen) -> Self {
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
