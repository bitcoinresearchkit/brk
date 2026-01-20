use brk_traversable::Traversable;
use brk_types::{StoredF32, StoredI8, StoredU16, Version};

use super::{
    indexes,
    internal::{ConstantVecs, ReturnF32Tenths, ReturnI8, ReturnU16},
};

pub const DB_NAME: &str = "constants";

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub constant_0: ConstantVecs<StoredU16>,
    pub constant_1: ConstantVecs<StoredU16>,
    pub constant_2: ConstantVecs<StoredU16>,
    pub constant_3: ConstantVecs<StoredU16>,
    pub constant_4: ConstantVecs<StoredU16>,
    pub constant_20: ConstantVecs<StoredU16>,
    pub constant_30: ConstantVecs<StoredU16>,
    pub constant_38_2: ConstantVecs<StoredF32>,
    pub constant_50: ConstantVecs<StoredU16>,
    pub constant_61_8: ConstantVecs<StoredF32>,
    pub constant_70: ConstantVecs<StoredU16>,
    pub constant_80: ConstantVecs<StoredU16>,
    pub constant_100: ConstantVecs<StoredU16>,
    pub constant_600: ConstantVecs<StoredU16>,
    pub constant_minus_1: ConstantVecs<StoredI8>,
    pub constant_minus_2: ConstantVecs<StoredI8>,
    pub constant_minus_3: ConstantVecs<StoredI8>,
    pub constant_minus_4: ConstantVecs<StoredI8>,
}

impl Vecs {
    pub fn new(version: Version, indexes: &indexes::Vecs) -> Self {
        let v = version;

        Self {
            constant_0: ConstantVecs::new::<ReturnU16<0>>("constant_0", v, indexes),
            constant_1: ConstantVecs::new::<ReturnU16<1>>("constant_1", v, indexes),
            constant_2: ConstantVecs::new::<ReturnU16<2>>("constant_2", v, indexes),
            constant_3: ConstantVecs::new::<ReturnU16<3>>("constant_3", v, indexes),
            constant_4: ConstantVecs::new::<ReturnU16<4>>("constant_4", v, indexes),
            constant_20: ConstantVecs::new::<ReturnU16<20>>("constant_20", v, indexes),
            constant_30: ConstantVecs::new::<ReturnU16<30>>("constant_30", v, indexes),
            constant_38_2: ConstantVecs::new::<ReturnF32Tenths<382>>("constant_38_2", v, indexes),
            constant_50: ConstantVecs::new::<ReturnU16<50>>("constant_50", v, indexes),
            constant_61_8: ConstantVecs::new::<ReturnF32Tenths<618>>("constant_61_8", v, indexes),
            constant_70: ConstantVecs::new::<ReturnU16<70>>("constant_70", v, indexes),
            constant_80: ConstantVecs::new::<ReturnU16<80>>("constant_80", v, indexes),
            constant_100: ConstantVecs::new::<ReturnU16<100>>("constant_100", v, indexes),
            constant_600: ConstantVecs::new::<ReturnU16<600>>("constant_600", v, indexes),
            constant_minus_1: ConstantVecs::new::<ReturnI8<-1>>("constant_minus_1", v, indexes),
            constant_minus_2: ConstantVecs::new::<ReturnI8<-2>>("constant_minus_2", v, indexes),
            constant_minus_3: ConstantVecs::new::<ReturnI8<-3>>("constant_minus_3", v, indexes),
            constant_minus_4: ConstantVecs::new::<ReturnI8<-4>>("constant_minus_4", v, indexes),
        }
    }
}
