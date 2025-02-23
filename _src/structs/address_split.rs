use allocative::Allocative;

use super::{AddressLiquidity, AddressSize, AddressType};

#[derive(Default, Allocative)]
pub enum AddressSplit {
    #[default]
    All,
    Type(AddressType),
    Size(AddressSize),
    Liquidity(AddressLiquidity),
}
