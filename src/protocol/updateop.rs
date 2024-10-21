#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

use super::XmlSafeString;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct UpdateOp {
    pub url: XmlSafeString,
    pub uuid: XmlSafeString,
    pub username: XmlSafeString,
    pub password: XmlSafeString,
    pub version: XmlSafeString,
}

impl UpdateOp {
    #[must_use]
    pub fn new(url: &str, uuid: &str, username: &str, password: &str, version: &str) -> Self {
        Self {
            url: url.into(),
            uuid: uuid.into(),
            username: username.into(),
            password: password.into(),
            version: version.into(),
        }
    }
}

#[cfg(test)]
impl Arbitrary for UpdateOp {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            url: XmlSafeString::arbitrary(g),
            uuid: XmlSafeString::arbitrary(g),
            username: XmlSafeString::arbitrary(g),
            password: XmlSafeString::arbitrary(g),
            version: XmlSafeString::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.url.clone(),
                self.uuid.clone(),
                self.username.clone(),
                self.password.clone(),
                self.version.clone(),
            )
                .shrink()
                .map(|(u, uu, un, pw, v)| UpdateOp {
                    url: u,
                    uuid: uu,
                    username: un,
                    password: pw,
                    version: v,
                }),
        )
    }
}
