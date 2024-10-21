use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, parse_to_int, write_simple, GenerateError, XmlSafeString};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct FaultStruct {
    pub code: u32,
    pub string: XmlSafeString,
}

impl FaultStruct {
    #[must_use]
    pub fn new(code: u32, string: &str) -> Self {
        FaultStruct {
            code,
            string: string.into(),
        }
    }
    pub fn set_code(&mut self, code: u32) {
        self.code = code;
    }
    pub fn set_string(&mut self, string: &str) {
        self.string = string.into();
    }
}

#[cfg(test)]
impl Arbitrary for FaultStruct {
    fn arbitrary(g: &mut Gen) -> Self {
        FaultStruct {
            code: u32::arbitrary(g),
            string: XmlSafeString::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.code, self.string.clone())
                .shrink()
                .map(|(c, s)| FaultStruct { code: c, string: s }),
        )
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct FaultDetail {
    pub code: u32,
    pub string: XmlSafeString,
}

impl FaultDetail {
    #[must_use]
    pub fn new(code: u32, string: &str) -> Self {
        FaultDetail {
            code,
            string: string.into(),
        }
    }
}

#[cfg(test)]
impl Arbitrary for FaultDetail {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            code: u32::arbitrary(g),
            string: XmlSafeString::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.code, self.string.clone())
                .shrink()
                .map(|(c, s)| FaultDetail { code: c, string: s }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Fault {
    pub faultcode: XmlSafeString,
    pub faultstring: XmlSafeString,
    pub detail: FaultDetail,
}

impl Fault {
    #[must_use]
    pub fn new(faultcode: &str, faultstring: &str, code: u32, string: &str) -> Self {
        Fault {
            faultcode: faultcode.into(),
            faultstring: faultstring.into(),
            detail: FaultDetail::new(code, string),
        }
    }
    /// Generate XML for `Fault`
    ///     
    /// # Errors
    ///     Any errors encountered while writing to `writer` will be returned.
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element("SOAP-ENV:Fault"))?;
        write_simple(writer, "faultcode", self.faultcode.0.as_ref())?;
        write_simple(writer, "faultstring", self.faultstring.0.as_ref())?;
        writer.write(XmlEvent::start_element("detail"))?;
        writer.write(XmlEvent::start_element(&cwmp_prefix(has_cwmp, "Fault")[..]))?;
        write_simple(writer, "FaultCode", &self.detail.code.to_string())?;
        write_simple(writer, "FaultString", self.detail.string.0.as_ref())?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &str) {
        match *path {
            ["Fault", "faultcode"] => {
                self.faultcode = characters.into();
            }
            ["Fault", "faultstring"] => {
                self.faultstring = characters.into();
            }
            ["Fault", "detail", "Fault", "FaultCode"] => {
                self.detail.code = parse_to_int(characters, 0);
            }
            ["Fault", "detail", "Fault", "FaultString"] => {
                self.detail.string = characters.into();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for Fault {
    fn arbitrary(g: &mut Gen) -> Fault {
        Self {
            faultcode: XmlSafeString::arbitrary(g),
            faultstring: XmlSafeString::arbitrary(g),
            detail: FaultDetail {
                code: u32::arbitrary(g),
                string: XmlSafeString::arbitrary(g),
            },
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.faultcode.clone(),
                self.faultstring.clone(),
                self.detail.clone(),
            )
                .shrink()
                .map(|(c, s, d)| Fault {
                    faultcode: c,
                    faultstring: s,
                    detail: d,
                }),
        )
    }
}
