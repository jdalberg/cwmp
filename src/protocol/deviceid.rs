#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct DeviceId {
    pub manufacturer: String,
    pub oui: String,
    pub product_class: String,
    pub serial_number: String,
}
impl DeviceId {
    #[must_use]
    pub fn new(
        manufacturer: String,
        oui: String,
        product_class: String,
        serial_number: String,
    ) -> Self {
        DeviceId {
            manufacturer,
            oui,
            product_class,
            serial_number,
        }
    }
}

#[cfg(test)]
impl Arbitrary for DeviceId {
    fn arbitrary(g: &mut Gen) -> Self {
        DeviceId::new(
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
        )
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
