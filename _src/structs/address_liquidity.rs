use allocative::Allocative;

#[derive(Default, Allocative, Clone, Copy)]
pub enum AddressLiquidity {
    #[default]
    Illiquid,
    Liquid,
    HighlyLiquid,
}
