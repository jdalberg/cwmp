use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, write_simple, GenerateError};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetParameterAttributes {
    pub parameternames: Vec<String>,
}

impl GetParameterAttributes {
    #[must_use]
    pub fn new(parameternames: Vec<String>) -> Self {
        GetParameterAttributes { parameternames }
    }
    /// Generate XML for `GetParameterAttributes`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "GetParameterAttributes")[..],
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
        _attributes: &[xml::attribute::OwnedAttribute],
    ) {
        let path_pattern: Vec<&str> = path.iter().map(AsRef::as_ref).collect();
        if let ["GetParameterAttributes", "ParameterNames", "string"] = &path_pattern[..] {
            self.parameternames.push(String::new());
        }
    }
    pub fn characters(&mut self, path: &[&str], characters: &String) {
        if let ["GetParameterAttributes", "ParameterNames", "string"] = *path {
            if let Some(l) = self.parameternames.last_mut() {
                *l = characters.to_string();
            }
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetParameterAttributes {
    fn arbitrary(g: &mut Gen) -> Self {
        GetParameterAttributes::new(Vec::<String>::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.parameternames
                .clone()
                .shrink()
                .map(|pn| GetParameterAttributes { parameternames: pn }),
        )
    }
}
