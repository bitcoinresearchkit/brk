use std::{cmp::Ordering, fmt, str::FromStr};

use allocative::{Allocative, Visitor};
use bincode::{
    de::{BorrowDecoder, Decoder},
    enc::Encoder,
    error::{DecodeError, EncodeError},
    BorrowDecode, Decode, Encode,
};
use chrono::{Datelike, Days, NaiveDate, NaiveDateTime};
use derive_deref::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

use crate::utils::ONE_DAY_IN_S;

use super::{DateMapChunkId, MapKey, Timestamp};

const NUMBER_OF_UNSAFE_DATES: usize = 2;
const MIN_YEAR: i32 = 2009;
const APPROX_MAX_YEAR: i32 = 2100;

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Deref,
    DerefMut,
    Default,
    Serialize,
    Deserialize,
)]
pub struct Date(NaiveDate);

impl Date {
    pub fn wrap(date: NaiveDate) -> Self {
        Self(date)
    }

    pub fn today() -> Self {
        Self(chrono::offset::Utc::now().date_naive())
    }

    pub fn yesterday() -> Self {
        Self(Self::today().checked_sub_days(Days::new(1)).unwrap())
    }

    pub fn difference_in_days_between(&self, older: Self) -> u32 {
        (**self - *older).num_days() as u32
    }

    pub fn to_timestamp(self) -> Timestamp {
        Timestamp::wrap(NaiveDateTime::from(*self).and_utc().timestamp() as u32)
    }

    /// Returns value between 0.0 and 1.0 depending on its completion
    ///
    /// Any date before today (utc) will return 1.0
    ///
    /// Any date after today (utc) will panic even though it should return 0.0, as it shouldn't happen in the code
    ///
    /// Any date equal to today will have a completion between 0.0 and 1.0
    pub fn get_day_completion(self) -> f64 {
        let now = Timestamp::now();
        let today = Date::today();

        match self.cmp(&today) {
            Ordering::Less => 1.0,
            Ordering::Equal => *(now - self.to_timestamp()) as f64 / ONE_DAY_IN_S as f64,
            Ordering::Greater => unreachable!("0.0 but shouldn't be called"),
        }
    }
}

impl MapKey<DateMapChunkId> for Date {
    fn to_chunk_id(&self) -> DateMapChunkId {
        DateMapChunkId::new(self)
    }

    fn to_first_unsafe(&self) -> Option<Self> {
        let offset = NUMBER_OF_UNSAFE_DATES - 1;

        self.checked_sub_days(Days::new(offset as u64))
            .map(Date::wrap)
    }

    fn to_serialized_key(&self) -> Self {
        *self
    }

    fn is_out_of_bounds(&self) -> bool {
        !(MIN_YEAR..=APPROX_MAX_YEAR).contains(&self.year())
    }

    fn is_first(&self) -> bool {
        let day = self.day();

        if self.year() == 2009 && self.month() == 1 {
            day == 3
        } else {
            day == 1
        }
    }

    fn checked_sub(&self, days: usize) -> Option<Self> {
        self.checked_sub_days(Days::new(days as u64))
            .map(Self::wrap)
    }

    fn min_percentile_key() -> Self {
        Self::wrap(NaiveDate::from_ymd_opt(2012, 1, 1).unwrap())
    }

    fn iter_up_to(&self, other: &Self) -> impl Iterator<Item = Self> {
        self.iter_days().take_while(|d| d <= other).map(Date::wrap)
    }

    fn map_name<'a>() -> &'a str {
        "date"
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl Encode for Date {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Encode::encode(&self.to_string(), encoder)
    }
}

impl Decode for Date {
    fn decode<D: Decoder>(decoder: &mut D) -> core::result::Result<Self, DecodeError> {
        let str: String = Decode::decode(decoder)?;

        Ok(Self(NaiveDate::from_str(&str).unwrap()))
    }
}

impl<'de> BorrowDecode<'de> for Date {
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let str: String = BorrowDecode::borrow_decode(decoder)?;

        Ok(Self(NaiveDate::from_str(&str).unwrap()))
    }
}

impl Allocative for Date {
    fn visit<'a, 'b: 'a>(&self, visitor: &'a mut Visitor<'b>) {
        visitor.visit_simple_sized::<Self>();
    }
}
