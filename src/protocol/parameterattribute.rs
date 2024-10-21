#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

use super::{convert_to_xml_safe_strings, XmlSafeString};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ParameterAttribute {
    pub name: XmlSafeString,
    pub notification: XmlSafeString,
    pub accesslist: Vec<XmlSafeString>,
}
impl ParameterAttribute {
    #[must_use]
    pub fn new(name: &str, notification: &str, accesslist: &[&str]) -> Self {
        ParameterAttribute {
            name: name.into(),
            notification: notification.into(),
            accesslist: convert_to_xml_safe_strings(accesslist),
        }
    }
}

#[cfg(test)]
impl Arbitrary for ParameterAttribute {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            name: XmlSafeString::arbitrary(g),
            notification: XmlSafeString::arbitrary(g),
            accesslist: Vec::<XmlSafeString>::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.name.clone(),
                self.notification.clone(),
                self.accesslist.clone(),
            )
                .shrink()
                .map(|(n, no, a)| ParameterAttribute {
                    name: n,
                    notification: no,
                    accesslist: a,
                }),
        )
    }
}
