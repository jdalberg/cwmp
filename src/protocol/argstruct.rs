#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

use super::XmlSafeString;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ArgStruct {
    pub name: XmlSafeString,
    pub value: XmlSafeString,
}

impl ArgStruct {
    #[must_use]
    pub fn new(name: &str, value: &str) -> Self {
        ArgStruct {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[cfg(test)]
impl Arbitrary for ArgStruct {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            name: XmlSafeString::arbitrary(g),
            value: XmlSafeString::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.name.clone(), self.value.clone())
                .shrink()
                .map(|(n, v)| ArgStruct { name: n, value: v }),
        )
    }
}
