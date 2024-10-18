#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ParameterInfoStruct {
    pub name: String,
    pub writable: u8,
}

impl ParameterInfoStruct {
    #[must_use]
    pub fn new(name: String, writable: u8) -> Self {
        ParameterInfoStruct { name, writable }
    }
}

#[cfg(test)]
impl Arbitrary for ParameterInfoStruct {
    fn arbitrary(g: &mut Gen) -> Self {
        ParameterInfoStruct::new(String::arbitrary(g), u8::arbitrary(g))
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.name.clone(), self.writable.clone())
                .shrink()
                .map(|(n, w)| ParameterInfoStruct {
                    name: n,
                    writable: w,
                }),
        )
    }
}
