#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

use super::XmlSafeString;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct InstallOp {
    pub url: XmlSafeString,
    pub uuid: XmlSafeString,
    pub username: XmlSafeString,
    pub password: XmlSafeString,
    pub execution_env_ref: XmlSafeString,
}

impl InstallOp {
    #[must_use]
    pub fn new(
        url: &str,
        uuid: &str,
        username: &str,
        password: &str,
        execution_env_ref: &str,
    ) -> Self {
        InstallOp {
            url: url.into(),
            uuid: uuid.into(),
            username: username.into(),
            password: password.into(),
            execution_env_ref: execution_env_ref.into(),
        }
    }
}

#[cfg(test)]
impl Arbitrary for InstallOp {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            url: XmlSafeString::arbitrary(g),
            uuid: XmlSafeString::arbitrary(g),
            username: XmlSafeString::arbitrary(g),
            password: XmlSafeString::arbitrary(g),
            execution_env_ref: XmlSafeString::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.url.clone(),
                self.uuid.clone(),
                self.username.clone(),
                self.password.clone(),
                self.execution_env_ref.clone(),
            )
                .shrink()
                .map(|(u, uu, un, pw, eer)| InstallOp {
                    url: u,
                    uuid: uu,
                    username: un,
                    password: pw,
                    execution_env_ref: eer,
                }),
        )
    }
}
