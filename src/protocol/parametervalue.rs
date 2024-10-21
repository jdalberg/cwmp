#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

use super::XmlSafeString;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ParameterValue {
    pub name: XmlSafeString,
    pub r#type: XmlSafeString,
    pub value: XmlSafeString,
}

impl ParameterValue {
    #[must_use]
    pub fn new(name: &str, param_type: &str, value: &str) -> Self {
        ParameterValue {
            name: name.into(),
            r#type: param_type.into(),
            value: value.into(),
        }
    }
}

#[cfg(test)]
impl Arbitrary for ParameterValue {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            name: XmlSafeString::arbitrary(g),
            r#type: XmlSafeString::arbitrary(g),
            value: XmlSafeString::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.name.clone(), self.r#type.clone(), self.value.clone())
                .shrink()
                .map(|(n, t, v)| ParameterValue {
                    name: n,
                    r#type: t,
                    value: v,
                }),
        )
    }
}
