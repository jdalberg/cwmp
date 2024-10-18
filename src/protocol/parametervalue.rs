#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ParameterValue {
    pub name: String,
    pub r#type: String,
    pub value: String,
}

impl ParameterValue {
    #[must_use]
    pub fn new(name: String, param_type: String, value: String) -> Self {
        ParameterValue {
            name,
            r#type: param_type,
            value,
        }
    }
}

#[cfg(test)]
impl Arbitrary for ParameterValue {
    fn arbitrary(g: &mut Gen) -> Self {
        ParameterValue::new(
            String::arbitrary(g),
            String::arbitrary(g),
            String::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.name.clone(), self.r#type.clone(), self.value.clone())
                .shrink()
                .map(|(n, t, v)| ParameterValue {
                    name: n,
                    r#type: t,
                    value: v,
                }),
        )
    }
}
