#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
#[cfg(test)]
use rand::Rng;

use super::{
    HoldRequests, NoMoreRequests, SessionTimeout, SupportedCWMPVersions, UseCWMPVersion, ID,
};

#[derive(Debug, PartialEq, Clone)]
pub enum HeaderElement {
    ID(ID),
    HoldRequests(HoldRequests),
    SessionTimeout(SessionTimeout),
    NoMoreRequests(NoMoreRequests),
    SupportedCWMPVersions(SupportedCWMPVersions),
    UseCWMPVersion(UseCWMPVersion),
}

#[cfg(test)]
impl Arbitrary for HeaderElement {
    fn arbitrary(g: &mut Gen) -> Self {
        let vals = vec![
            HeaderElement::ID(ID::arbitrary(g)),
            HeaderElement::HoldRequests(HoldRequests::arbitrary(g)),
            HeaderElement::SessionTimeout(SessionTimeout::arbitrary(g)),
            HeaderElement::NoMoreRequests(NoMoreRequests::arbitrary(g)),
            HeaderElement::SupportedCWMPVersions(SupportedCWMPVersions::arbitrary(g)),
            HeaderElement::UseCWMPVersion(UseCWMPVersion::arbitrary(g)),
        ];
        let mut rng = rand::thread_rng();
        let idxs = std::ops::Range {
            start: 0,
            end: vals.len() - 1,
        };
        let random_index: usize = rng.gen_range(idxs);
        match vals.get(random_index) {
            Some(v) => v.clone(),
            None => HeaderElement::ID(ID::arbitrary(g)),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        match self {
            &HeaderElement::ID(ref x) => Box::new(x.shrink().map(|s| HeaderElement::ID(s))),
            &HeaderElement::HoldRequests(ref x) => {
                Box::new(x.shrink().map(|s| HeaderElement::HoldRequests(s)))
            }
            &HeaderElement::SessionTimeout(ref x) => {
                Box::new(x.shrink().map(|s| HeaderElement::SessionTimeout(s)))
            }
            &HeaderElement::NoMoreRequests(ref x) => {
                Box::new(x.shrink().map(|s| HeaderElement::NoMoreRequests(s)))
            }
            &HeaderElement::SupportedCWMPVersions(ref x) => {
                Box::new(x.shrink().map(|s| HeaderElement::SupportedCWMPVersions(s)))
            }
            &HeaderElement::UseCWMPVersion(ref x) => {
                Box::new(x.shrink().map(|s| HeaderElement::UseCWMPVersion(s)))
            }
        }
    }
}
