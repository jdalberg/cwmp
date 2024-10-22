use std::io::Write;

use xml::writer::XmlEvent;

use super::{convert_to_xml_safe_strings, cwmp_prefix, write_simple, GenerateError, XmlSafeString};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SetVouchers {
    pub voucher_list: Vec<XmlSafeString>,
}

impl SetVouchers {
    #[must_use]
    pub fn new(voucher_list: &[&str]) -> Self {
        SetVouchers {
            voucher_list: convert_to_xml_safe_strings(voucher_list),
        }
    }

    /// Generate XML for `SetVouchers`
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
            &cwmp_prefix(has_cwmp, "SetVouchers")[..],
        ))?;

        let vls = format!("base64[{}]", self.voucher_list.len());
        writer
            .write(XmlEvent::start_element("VoucherList").attr("SOAP-ENC:arrayType", &vls[..]))?;

        for v in &self.voucher_list {
            write_simple(writer, "base64", v.0.as_ref())?;
        }
        writer.write(XmlEvent::end_element())?; // VoucherList
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
        if let ["SetVouchers", "VoucherList", "base64"] = &path_pattern[..] {
            self.voucher_list.push(XmlSafeString::new());
        }
    }
    pub fn characters(&mut self, path: &[&str], characters: &str) {
        if let ["SetVouchers", "VoucherList", "base64"] = *path {
            if let Some(v) = self.voucher_list.last_mut() {
                *v = characters.into();
            }
        }
    }
}

#[cfg(test)]
impl Arbitrary for SetVouchers {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            voucher_list: Vec::<XmlSafeString>::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.voucher_list
                .clone()
                .shrink()
                .map(|vl| SetVouchers { voucher_list: vl }),
        )
    }
}
