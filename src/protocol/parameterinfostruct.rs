#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

use super::XmlSafeString;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ParameterInfoStruct {
    pub name: XmlSafeString,
    pub writable: u8,
}

impl ParameterInfoStruct {
    #[must_use]
    pub fn new(name: &str, writable: u8) -> Self {
        ParameterInfoStruct {
            name: name.into(),
            writable,
        }
    }
}

#[cfg(test)]
impl Arbitrary for ParameterInfoStruct {
    fn arbitrary(g: &mut Gen) -> Self {
        ParameterInfoStruct::new(XmlSafeString::arbitrary(g).0.as_ref(), u8::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.name.clone(), self.writable)
                .shrink()
                .map(|(n, w)| ParameterInfoStruct {
                    name: n,
                    writable: w,
                }),
        )
    }
}
