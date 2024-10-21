use chrono::{DateTime, Utc};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[cfg(test)]
use super::gen_utc_date;
use super::{FaultStruct, XmlSafeString};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct OpResult {
    pub uuid: XmlSafeString,
    pub deployment_unit_ref: XmlSafeString,
    pub version: XmlSafeString,
    pub current_state: XmlSafeString,
    pub resolved: u32,
    pub execution_unit_ref_list: XmlSafeString,
    pub start_time: Option<DateTime<Utc>>,
    pub complete_time: Option<DateTime<Utc>>,
    pub fault: FaultStruct,
}

impl OpResult {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        uuid: &str,
        deployment_unit_ref: &str,
        version: &str,
        current_state: &str,
        resolved: u32,
        execution_unit_ref_list: &str,
        start_time: DateTime<Utc>,
        complete_time: DateTime<Utc>,
        fault: FaultStruct,
    ) -> Self {
        OpResult {
            uuid: uuid.into(),
            deployment_unit_ref: deployment_unit_ref.into(),
            version: version.into(),
            current_state: current_state.into(),
            resolved,
            execution_unit_ref_list: execution_unit_ref_list.into(),
            start_time: Some(start_time),
            complete_time: Some(complete_time),
            fault,
        }
    }
}

#[cfg(test)]
impl Arbitrary for OpResult {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            uuid: XmlSafeString::arbitrary(g),
            deployment_unit_ref: XmlSafeString::arbitrary(g),
            version: XmlSafeString::arbitrary(g),
            current_state: XmlSafeString::arbitrary(g),
            resolved: u32::arbitrary(g),
            execution_unit_ref_list: XmlSafeString::arbitrary(g),
            start_time: Some(gen_utc_date(2014, 11, 28, 12, 0, 9)),
            complete_time: Some(gen_utc_date(2014, 11, 29, 12, 0, 9)),
            fault: FaultStruct::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.uuid.clone(),
                self.deployment_unit_ref.clone(),
                self.version.clone(),
                self.current_state.clone(),
                self.resolved,
                self.execution_unit_ref_list.clone(),
                self.fault.clone(),
            )
                .shrink()
                .map(|(u, dur, v, cs, r, eurl, f)| OpResult {
                    uuid: u,
                    deployment_unit_ref: dur,
                    version: v,
                    current_state: cs,
                    resolved: r,
                    execution_unit_ref_list: eurl,
                    start_time: Some(gen_utc_date(2014, 11, 28, 12, 0, 9)),
                    complete_time: Some(gen_utc_date(2014, 11, 29, 12, 0, 9)),
                    fault: f,
                }),
        )
    }
}
