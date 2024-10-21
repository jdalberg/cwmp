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
            HeaderElement::ID(x) => Box::new(x.shrink().map(HeaderElement::ID)),
            HeaderElement::HoldRequests(x) => {
                Box::new(x.shrink().map(HeaderElement::HoldRequests))
            }
            HeaderElement::SessionTimeout(x) => {
                Box::new(x.shrink().map(HeaderElement::SessionTimeout))
            }
            HeaderElement::NoMoreRequests(x) => {
                Box::new(x.shrink().map(HeaderElement::NoMoreRequests))
            }
            HeaderElement::SupportedCWMPVersions(x) => {
                Box::new(x.shrink().map(HeaderElement::SupportedCWMPVersions))
            }
            HeaderElement::UseCWMPVersion(x) => {
                Box::new(x.shrink().map(HeaderElement::UseCWMPVersion))
            }
        }
    }
}
