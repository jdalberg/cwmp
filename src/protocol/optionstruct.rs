use chrono::{DateTime, Utc};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[cfg(test)]
use super::gen_utc_date;
use super::XmlSafeString;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct OptionStruct {
    pub option_name: XmlSafeString,
    pub voucher_sn: XmlSafeString,
    pub state: u8,
    pub mode: XmlSafeString,
    pub start_date: Option<DateTime<Utc>>,
    pub expiration_date: Option<DateTime<Utc>>,
    pub is_transferable: u8,
}

impl OptionStruct {
    #[must_use]
    pub fn new(
        option_name: &str,
        voucher_sn: &str,
        state: u8,
        mode: &str,
        start_date: DateTime<Utc>,
        expiration_date: DateTime<Utc>,
        is_transferable: u8,
    ) -> Self {
        OptionStruct {
            option_name: option_name.into(),
            voucher_sn: voucher_sn.into(),
            state,
            mode: mode.into(),
            start_date: Some(start_date),
            expiration_date: Some(expiration_date),
            is_transferable,
        }
    }
}

#[cfg(test)]
impl Arbitrary for OptionStruct {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            option_name: XmlSafeString::arbitrary(g),
            voucher_sn: XmlSafeString::arbitrary(g),
            state: u8::arbitrary(g),
            mode: XmlSafeString::arbitrary(g),
            start_date: Some(gen_utc_date(2014, 11, 28, 12, 0, 9)),
            expiration_date: Some(gen_utc_date(2014, 11, 29, 12, 0, 9)),
            is_transferable: u8::arbitrary(g),
        }
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.option_name.clone(),
                self.voucher_sn.clone(),
                self.state,
                self.mode.clone(),
                self.is_transferable,
            )
                .shrink()
                .map(|(on, vsn, s, m, i)| OptionStruct {
                    option_name: on,
                    voucher_sn: vsn,
                    state: s,
                    mode: m,
                    is_transferable: i,
                    start_date: Some(gen_utc_date(2014, 11, 28, 12, 0, 9)),
                    expiration_date: Some(gen_utc_date(2014, 11, 29, 12, 0, 9)),
                }),
        )
    }
}
