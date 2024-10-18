use std::io::Write;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use xml::writer::XmlEvent;

use super::{cwmp_prefix, parse_to_int, write_simple, GenerateError};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct FaultStruct {
    pub code: u32,
    pub string: String,
}

impl FaultStruct {
    #[must_use] pub fn new(code: u32, string: String) -> Self {
        FaultStruct { code, string }
    }
    pub fn set_code(&mut self, code: u32) {
        self.code = code;
    }
    pub fn set_string(&mut self, string: &str) {
        self.string = string.to_string();
    }
}

#[cfg(test)]
impl Arbitrary for FaultStruct {
    fn arbitrary(g: &mut Gen) -> Self {
        FaultStruct::new(u32::arbitrary(g), String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.code.clone(), self.string.clone())
                .shrink()
                .map(|(c, s)| FaultStruct { code: c, string: s }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct FaultDetail {
    pub code: u32,
    pub string: String,
}

impl FaultDetail {
    #[must_use] pub fn new(code: u32, string: String) -> Self {
        FaultDetail { code, string }
    }
}

#[cfg(test)]
impl Arbitrary for FaultDetail {
    fn arbitrary(g: &mut Gen) -> Self {
        FaultDetail::new(u32::arbitrary(g), String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.code.clone(), self.string.clone())
                .shrink()
                .map(|(c, s)| FaultDetail { code: c, string: s }),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Fault {
    pub faultcode: String,
    pub faultstring: String,
    pub detail: FaultDetail,
}

impl Fault {
    #[must_use] pub fn new(faultcode: String, faultstring: String, code: u32, string: String) -> Self {
        Fault {
            faultcode,
            faultstring,
            detail: FaultDetail::new(code, string),
        }
    }
    pub fn generate<W: Write>(
        &self,
        writer: &mut xml::EventWriter<W>,
        has_cwmp: bool,
    ) -> Result<(), GenerateError> {
        writer.write(XmlEvent::start_element("SOAP-ENV:Fault"))?;
        write_simple(writer, "faultcode", &self.faultcode)?;
        write_simple(writer, "faultstring", &self.faultstring)?;
        writer.write(XmlEvent::start_element("detail"))?;
        writer.write(XmlEvent::start_element(&cwmp_prefix(has_cwmp, "Fault")[..]))?;
        write_simple(writer, "FaultCode", &self.detail.code.to_string())?;
        write_simple(writer, "FaultString", &self.detail.string.to_string())?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
    pub fn characters(&mut self, path: &[&str], characters: &String) {
        match *path {
            ["Fault", "faultcode"] => {
                self.faultcode = characters.to_string();
            }
            ["Fault", "faultstring"] => {
                self.faultstring = characters.to_string();
            }
            ["Fault", "detail", "Fault", "FaultCode"] => {
                self.detail.code = parse_to_int(characters, 0);
            }
            ["Fault", "detail", "Fault", "FaultString"] => {
                self.detail.string = characters.to_string();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
impl Arbitrary for Fault {
    fn arbitrary(g: &mut Gen) -> Fault {
        Fault::new(
            String::arbitrary(g),
            String::arbitrary(g),
            u32::arbitrary(g),
            String::arbitrary(g),
        )
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
