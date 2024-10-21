#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct CwmpVersion {
    pub major: u8,
    pub minor: u8,
}

impl CwmpVersion {
    #[must_use]
    pub fn new(major: u8, minor: u8) -> Self {
        CwmpVersion { major, minor }
    }
}

#[cfg(test)]
impl Arbitrary for CwmpVersion {
    fn arbitrary(g: &mut Gen) -> Self {
        CwmpVersion {
            major: u8::arbitrary(g),
            minor: u8::arbitrary(g),
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.major, self.minor)
                .shrink()
                .map(|(ma, mi)| CwmpVersion {
                    major: ma,
                    minor: mi,
                }),
        )
    }
}
