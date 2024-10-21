#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

use super::XmlSafeString;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct UninstallOp {
    pub url: XmlSafeString,
    pub uuid: XmlSafeString,
    pub execution_env_ref: XmlSafeString,
}

impl UninstallOp {
    #[must_use]
    pub fn new(url: &str, uuid: &str, execution_env_ref: &str) -> Self {
        Self {
            url: url.into(),
            uuid: uuid.into(),
            execution_env_ref: execution_env_ref.into(),
        }
    }
}

#[cfg(test)]
impl Arbitrary for UninstallOp {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            url: XmlSafeString::arbitrary(g),
            uuid: XmlSafeString::arbitrary(g),
            execution_env_ref: XmlSafeString::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.url.clone(),
                self.uuid.clone(),
                self.execution_env_ref.clone(),
            )
                .shrink()
                .map(|(u, uu, eer)| UninstallOp {
                    url: u,
                    uuid: uu,
                    execution_env_ref: eer,
                }),
        )
    }
}
