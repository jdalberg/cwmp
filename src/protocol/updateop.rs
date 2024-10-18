#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct UpdateOp {
    pub url: String,
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub version: String,
}

impl UpdateOp {
    pub fn new(
        url: String,
        uuid: String,
        username: String,
        password: String,
        version: String,
    ) -> Self {
        UpdateOp {
            url,
            uuid,
            username,
            password,
            version,
        }
    }
}

#[cfg(test)]
impl Arbitrary for UpdateOp {
    fn arbitrary(g: &mut Gen) -> Self {
        UpdateOp::new(
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
