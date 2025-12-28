use brk_traversable::Traversable;
use brk_types::{StoredF32, StoredI16, StoredU16, Version};

use super::{
    grouped::{ConstantVecs, ReturnF32Tenths, ReturnI16, ReturnU16},
    indexes,
};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub constant_0: ConstantVecs<StoredU16>,
    pub constant_1: ConstantVecs<StoredU16>,
    pub constant_2: ConstantVecs<StoredU16>,
    pub constant_3: ConstantVecs<StoredU16>,
    pub constant_4: ConstantVecs<StoredU16>,
    pub constant_38_2: ConstantVecs<StoredF32>,
    pub constant_50: ConstantVecs<StoredU16>,
    pub constant_61_8: ConstantVecs<StoredF32>,
    pub constant_100: ConstantVecs<StoredU16>,
    pub constant_600: ConstantVecs<StoredU16>,
    pub constant_minus_1: ConstantVecs<StoredI16>,
    pub constant_minus_2: ConstantVecs<StoredI16>,
    pub constant_minus_3: ConstantVecs<StoredI16>,
    pub constant_minus_4: ConstantVecs<StoredI16>,
}

impl Vecs {
    pub fn new(version: Version, indexes: &indexes::Vecs) -> Self {
        let v = version + Version::ZERO;

        Self {
            constant_0: ConstantVecs::new::<ReturnU16<0>>("constant_0", v, indexes),
            constant_1: ConstantVecs::new::<ReturnU16<1>>("constant_1", v, indexes),
            constant_2: ConstantVecs::new::<ReturnU16<2>>("constant_2", v, indexes),
            constant_3: ConstantVecs::new::<ReturnU16<3>>("constant_3", v, indexes),
            constant_4: ConstantVecs::new::<ReturnU16<4>>("constant_4", v, indexes),
            constant_38_2: ConstantVecs::new::<ReturnF32Tenths<382>>("constant_38_2", v, indexes),
            constant_50: ConstantVecs::new::<ReturnU16<50>>("constant_50", v, indexes),
            constant_61_8: ConstantVecs::new::<ReturnF32Tenths<618>>("constant_61_8", v, indexes),
            constant_100: ConstantVecs::new::<ReturnU16<100>>("constant_100", v, indexes),
            constant_600: ConstantVecs::new::<ReturnU16<600>>("constant_600", v, indexes),
            constant_minus_1: ConstantVecs::new::<ReturnI16<-1>>("constant_minus_1", v, indexes),
            constant_minus_2: ConstantVecs::new::<ReturnI16<-2>>("constant_minus_2", v, indexes),
            constant_minus_3: ConstantVecs::new::<ReturnI16<-3>>("constant_minus_3", v, indexes),
            constant_minus_4: ConstantVecs::new::<ReturnI16<-4>>("constant_minus_4", v, indexes),
        }
    }
}
