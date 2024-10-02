use std::{
    f64::consts::E,
    ops::{AddAssign, SubAssign},
};

use allocative::Allocative;

use super::Amount;

#[derive(Debug)]
pub struct LiquidityClassification {
    illiquid: f64,
    liquid: f64,
}

impl LiquidityClassification {
    /// Following this:
    /// https://insights.glassnode.com/bitcoin-liquid-supply/
    /// https://www.desmos.com/calculator/dutgni5rtj
    pub fn new(sent: Amount, received: Amount) -> Self {
        if received == Amount::ZERO {
            dbg!(sent, received);
            panic!()
        }

        let liquidity = {
            if sent > received {
                panic!("Shouldn't be possible");
            }

            if sent == Amount::ZERO {
                0.0
            } else {
                let liquidity = sent.to_sat() as f64 / received.to_sat() as f64;

                if liquidity.is_nan() {
                    dbg!(sent, received);
                    unreachable!()
                } else {
                    liquidity
                }
            }
        };

        let illiquid_line = Self::compute_illiquid_line(liquidity);
        let liquid_line = Self::compute_liquid_line(liquidity);

        let illiquid = illiquid_line;
        let liquid = liquid_line - illiquid_line;
        let highly_liquid = 1.0 - liquid_line;

        if illiquid < 0.0 || liquid < 0.0 || highly_liquid < 0.0 {
            unreachable!()
        }

        Self { illiquid, liquid }
    }

    #[inline(always)]
    pub fn split(&self, value: f64) -> LiquiditySplitResult {
        let illiquid = value * self.illiquid;
        let liquid = value * self.liquid;
        let highly_liquid = value - illiquid - liquid;

        LiquiditySplitResult {
            illiquid,
            liquid,
            highly_liquid,
        }
    }

    /// Returns value in range 0.0..1.0
    #[inline(always)]
    fn compute_illiquid_line(x: f64) -> f64 {
        Self::compute_ratio(x, 0.25)
    }

    /// Returns value in range 0.0..1.0
    #[inline(always)]
    fn compute_liquid_line(x: f64) -> f64 {
        Self::compute_ratio(x, 0.75)
    }

    #[inline(always)]
    fn compute_ratio(x: f64, x0: f64) -> f64 {
        let l = 1.0;
        let k = 25.0;

        l / (1.0 + E.powf(k * (x - x0)))
    }
}

#[derive(Debug, Default)]
pub struct LiquiditySplitResult {
    pub illiquid: f64,
    pub liquid: f64,
    pub highly_liquid: f64,
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy, Allocative)]
pub struct SplitByLiquidity<T>
where
    T: Default,
{
    pub all: T,
    pub illiquid: T,
    pub liquid: T,
    pub highly_liquid: T,
}

impl<T> AddAssign for SplitByLiquidity<T>
where
    T: AddAssign + Default,
{
    fn add_assign(&mut self, rhs: Self) {
        self.all += rhs.all;
        self.illiquid += rhs.illiquid;
        self.liquid += rhs.liquid;
        self.highly_liquid += rhs.highly_liquid;
    }
}

impl<T> SubAssign for SplitByLiquidity<T>
where
    T: SubAssign + Default,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.all -= rhs.all;
        self.illiquid -= rhs.illiquid;
        self.liquid -= rhs.liquid;
        self.highly_liquid -= rhs.highly_liquid;
    }
}
