#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct InstallOp {
    pub url: String,
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub execution_env_ref: String,
}

impl InstallOp {
    pub fn new(
        url: String,
        uuid: String,
        username: String,
        password: String,
        execution_env_ref: String,
    ) -> Self {
        InstallOp {
            url,
            uuid,
            username,
            password,
            execution_env_ref,
        }
    }
}

#[cfg(test)]
impl Arbitrary for InstallOp {
    fn arbitrary(g: &mut Gen) -> Self {
        InstallOp::new(
            String::arbitrary(g),
            String::arbitrary(g),
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
