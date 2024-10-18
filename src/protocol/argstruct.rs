#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ArgStruct {
    pub name: String,
    pub value: String,
}

impl ArgStruct {
    #[must_use]
    pub fn new(name: String, value: String) -> Self {
        ArgStruct { name, value }
    }
}

#[cfg(test)]
impl Arbitrary for ArgStruct {
    fn arbitrary(g: &mut Gen) -> Self {
        ArgStruct::new(String::arbitrary(g), String::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.name.clone(), self.value.clone())
                .shrink()
                .map(|(n, v)| ArgStruct { name: n, value: v }),
        )
    }
}
