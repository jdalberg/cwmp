#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

use super::XmlSafeString;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct DeviceId {
    pub manufacturer: XmlSafeString,
    pub oui: XmlSafeString,
    pub product_class: XmlSafeString,
    pub serial_number: XmlSafeString,
}
impl DeviceId {
    #[must_use]
    pub fn new(manufacturer: &str, oui: &str, product_class: &str, serial_number: &str) -> Self {
        DeviceId {
            manufacturer: manufacturer.into(),
            oui: oui.into(),
            product_class: product_class.into(),
            serial_number: serial_number.into(),
        }
    }
}

#[cfg(test)]
impl Arbitrary for DeviceId {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            manufacturer: XmlSafeString::arbitrary(g),
            oui: XmlSafeString::arbitrary(g),
            product_class: XmlSafeString::arbitrary(g),
            serial_number: XmlSafeString::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.manufacturer.clone(),
                self.oui.clone(),
                self.product_class.clone(),
                self.serial_number.clone(),
            )
                .shrink()
                .map(|(m, o, p, s)| DeviceId {
                    manufacturer: m,
                    oui: o,
                    product_class: p,
                    serial_number: s,
                }),
        )
    }
}
