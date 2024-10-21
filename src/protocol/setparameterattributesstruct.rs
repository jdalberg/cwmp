#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

use super::{convert_to_xml_safe_strings, XmlSafeString};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SetParameterAttributesStruct {
    pub name: XmlSafeString,
    pub notification_change: u8,
    pub notification: u8,
    pub access_list_change: u8,
    pub access_list: Vec<XmlSafeString>,
}

impl SetParameterAttributesStruct {
    #[must_use]
    pub fn new(
        name: &str,
        notification_change: u8,
        notification: u8,
        access_list_change: u8,
        access_list: &[&str],
    ) -> Self {
        SetParameterAttributesStruct {
            name: name.into(),
            notification_change,
            notification,
            access_list_change,
            access_list: convert_to_xml_safe_strings(access_list),
        }
    }
}

#[cfg(test)]
impl Arbitrary for SetParameterAttributesStruct {
    fn arbitrary(g: &mut Gen) -> Self {
        SetParameterAttributesStruct {
            name: XmlSafeString::arbitrary(g),
            notification_change: u8::arbitrary(g),
            notification: u8::arbitrary(g),
            access_list_change: u8::arbitrary(g),
            access_list: Vec::<XmlSafeString>::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.name.clone(),
                self.notification_change.clone(),
                self.notification.clone(),
                self.access_list_change.clone(),
                self.access_list.clone(),
            )
                .shrink()
                .map(|(name, nc, n, alc, al)| SetParameterAttributesStruct {
                    name: name,
                    notification_change: nc,
                    notification: n,
                    access_list_change: alc,
                    access_list: al,
                }),
        )
    }
}
