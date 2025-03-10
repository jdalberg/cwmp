use std::io::Write;

use super::{cwmp_prefix, write_simple, GenerateError, XmlSafeString};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GetRPCMethodsResponse {
    pub method_list: Vec<XmlSafeString>,
}

impl GetRPCMethodsResponse {
    #[must_use]
    pub fn new(method_list: &[&str]) -> Self {
        GetRPCMethodsResponse {
            method_list: super::convert_to_xml_safe_strings(method_list),
        }
    }

    /// Generate XML for `GetRPCMethodsResponse`
    ///     
    /// # Errors
    /// Any errors encountered while writing to `writer` will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element(
            &cwmp_prefix(has_cwmp, "GetRPCMethodsResponse")[..],
        ))?;
        let ss = format!("xsd:string[{}]", self.method_list.len());

        writer.write(XmlEvent::start_element("MethodList").attr("SOAP-ENC:arrayType", &ss[..]))?;

        for p in &self.method_list {
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
        if let ["GetRPCMethodsResponse", "MethodList", "string"] = &path_pattern[..] {
            self.method_list.push(XmlSafeString::new());
        }
    }
    pub fn characters(&mut self, path: &[&str], characters: &str) {
        if let ["GetRPCMethodsResponse", "MethodList", "string"] = *path {
            let last = self.method_list.last_mut();
            if let Some(l) = last {
                *l = characters.into();
            }
        }
    }
}

#[cfg(test)]
impl Arbitrary for GetRPCMethodsResponse {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            method_list: Vec::<XmlSafeString>::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.method_list
                .clone()
                .shrink()
                .map(|ml| GetRPCMethodsResponse { method_list: ml }),
        )
    }
}
