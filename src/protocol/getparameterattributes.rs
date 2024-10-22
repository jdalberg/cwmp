use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, write_simple, GenerateError, XmlSafeString};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetParameterAttributes {
    pub parameternames: Vec<XmlSafeString>,
}

impl GetParameterAttributes {
    #[must_use]
    pub fn new(parameternames: &[&str]) -> Self {
        GetParameterAttributes {
            parameternames: super::convert_to_xml_safe_strings(parameternames),
        }
    }
    /// Generate XML for `GetParameterAttributes`
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
            &cwmp_prefix(has_cwmp, "GetParameterAttributes")[..],
        ))?;
        writer.write(XmlEvent::start_element("ParameterNames"))?;
        for p in &self.parameternames {
            write_simple(writer, "string", p.0.as_ref())?;
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
            self.parameternames.push(XmlSafeString::new());
        }
    }
    pub fn characters(&mut self, path: &[&str], characters: &str) {
        if let ["GetParameterAttributes", "ParameterNames", "string"] = *path {
            if let Some(l) = self.parameternames.last_mut() {
                *l = characters.into();
            }
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetParameterAttributes {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            parameternames: Vec::<XmlSafeString>::arbitrary(g),
        }
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
