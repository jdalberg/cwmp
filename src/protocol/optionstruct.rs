use chrono::{DateTime, Utc};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[cfg(test)]
use super::gen_utc_date;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct OptionStruct {
    pub option_name: String,
    pub voucher_sn: String,
    pub state: u8,
    pub mode: String,
    pub start_date: Option<DateTime<Utc>>,
    pub expiration_date: Option<DateTime<Utc>>,
    pub is_transferable: u8,
}

impl OptionStruct {
    pub fn new(
        option_name: String,
        voucher_sn: String,
        state: u8,
        mode: String,
        start_date: DateTime<Utc>,
        expiration_date: DateTime<Utc>,
        is_transferable: u8,
    ) -> Self {
        OptionStruct {
            option_name,
            voucher_sn,
            state,
            mode,
            start_date: Some(start_date),
            expiration_date: Some(expiration_date),
            is_transferable,
        }
    }
}

#[cfg(test)]
impl Arbitrary for OptionStruct {
    fn arbitrary(g: &mut Gen) -> Self {
        OptionStruct::new(
            String::arbitrary(g),
            String::arbitrary(g),
            u8::arbitrary(g),
            String::arbitrary(g),
            gen_utc_date(2014, 11, 28, 12, 0, 9),
            gen_utc_date(2014, 11, 29, 12, 0, 9),
            u8::arbitrary(g),
        )
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (
                self.option_name.clone(),
                self.voucher_sn.clone(),
                self.state.clone(),
                self.mode.clone(),
                self.is_transferable.clone(),
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
