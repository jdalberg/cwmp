#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ParameterAttribute {
    pub name: String,
    pub notification: String,
    pub accesslist: Vec<String>,
}
impl ParameterAttribute {
    #[must_use]
    pub fn new(name: String, notification: String, accesslist: Vec<String>) -> Self {
        ParameterAttribute {
            name,
            notification,
            accesslist,
        }
    }
}

#[cfg(test)]
impl Arbitrary for ParameterAttribute {
    fn arbitrary(g: &mut Gen) -> Self {
        ParameterAttribute::new(
            String::arbitrary(g),
            String::arbitrary(g),
            Vec::<String>::arbitrary(g),
        )
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
