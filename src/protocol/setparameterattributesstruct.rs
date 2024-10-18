#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SetParameterAttributesStruct {
    pub name: String,
    pub notification_change: u8,
    pub notification: u8,
    pub access_list_change: u8,
    pub access_list: Vec<String>,
}

impl SetParameterAttributesStruct {
    #[must_use]
    pub fn new(
        name: String,
        notification_change: u8,
        notification: u8,
        access_list_change: u8,
        access_list: Vec<String>,
    ) -> Self {
        SetParameterAttributesStruct {
            name,
            notification_change,
            notification,
            access_list_change,
            access_list,
        }
    }
}

#[cfg(test)]
impl Arbitrary for SetParameterAttributesStruct {
    fn arbitrary(g: &mut Gen) -> Self {
        SetParameterAttributesStruct::new(
            String::arbitrary(g),
            u8::arbitrary(g),
            u8::arbitrary(g),
            u8::arbitrary(g),
            Vec::<String>::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.name.clone(),
                self.notification_change.clone(),
                self.notification.clone(),
                self.access_list_change.clone(),
                self.access_list.clone(),
            )
                .shrink()
                .map(|(name, nc, n, alc, al)| SetParameterAttributesStruct {
                    name: name,
                    notification_change: nc,
                    notification: n,
                    access_list_change: alc,
                    access_list: al,
                }),
        )
    }
}
