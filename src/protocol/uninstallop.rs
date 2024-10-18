#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct UninstallOp {
    pub url: String,
    pub uuid: String,
    pub execution_env_ref: String,
}

impl UninstallOp {
    #[must_use] pub fn new(url: String, uuid: String, execution_env_ref: String) -> Self {
        UninstallOp {
            url,
            uuid,
            execution_env_ref,
        }
    }
}

#[cfg(test)]
impl Arbitrary for UninstallOp {
    fn arbitrary(g: &mut Gen) -> Self {
        UninstallOp::new(
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
        )
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
