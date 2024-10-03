use crate::structs::{Amount, Price};

#[derive(Default, Debug)]
pub struct PricePaidState {
    pub pp_05p: Option<Price>,
    pub pp_10p: Option<Price>,
    pub pp_15p: Option<Price>,
    pub pp_20p: Option<Price>,
    pub pp_25p: Option<Price>,
    pub pp_30p: Option<Price>,
    pub pp_35p: Option<Price>,
    pub pp_40p: Option<Price>,
    pub pp_45p: Option<Price>,
    pub pp_median: Option<Price>,
    pub pp_55p: Option<Price>,
    pub pp_60p: Option<Price>,
    pub pp_65p: Option<Price>,
    pub pp_70p: Option<Price>,
    pub pp_75p: Option<Price>,
    pub pp_80p: Option<Price>,
    pub pp_85p: Option<Price>,
    pub pp_90p: Option<Price>,
    pub pp_95p: Option<Price>,

    pub processed_amount: Amount,
}

impl PricePaidState {
    pub fn iterate(&mut self, price: Price, amount: Amount, supply: Amount) {
        let PricePaidState {
            processed_amount: processed_supply,
            pp_05p,
            pp_10p,
            pp_15p,
            pp_20p,
            pp_25p,
            pp_30p,
            pp_35p,
            pp_40p,
            pp_45p,
            pp_median,
            pp_55p,
            pp_60p,
            pp_65p,
            pp_70p,
            pp_75p,
            pp_80p,
            pp_85p,
            pp_90p,
            pp_95p,
        } = self;

        *processed_supply += amount;

        if pp_95p.is_some() {
            return;
        }

        let processed_sat_amount = processed_supply.to_sat();
        let total_sat_supply = supply.to_sat();

        if processed_sat_amount >= total_sat_supply * 95 / 100 {
            pp_95p.replace(price);
        }

        if pp_90p.is_some() {
            return;
        }

        if processed_sat_amount >= total_sat_supply * 90 / 100 {
            pp_90p.replace(price);
        }

        if pp_85p.is_some() {
            return;
        }

        if processed_sat_amount >= total_sat_supply * 85 / 100 {
            pp_85p.replace(price);
        }

        if pp_80p.is_some() {
            return;
        }

        if processed_sat_amount >= total_sat_supply * 80 / 100 {
            pp_80p.replace(price);
        }

        if pp_75p.is_some() {
            return;
        }

        if processed_sat_amount >= total_sat_supply * 75 / 100 {
            pp_75p.replace(price);
        }

        if pp_70p.is_some() {
            return;
        }

        if processed_sat_amount >= total_sat_supply * 70 / 100 {
            pp_70p.replace(price);
        }

        if pp_65p.is_some() {
            return;
        }

        if processed_sat_amount >= total_sat_supply * 65 / 100 {
            pp_65p.replace(price);
        }

        if pp_60p.is_some() {
            return;
        }

        if processed_sat_amount >= total_sat_supply * 60 / 100 {
            pp_60p.replace(price);
        }

        if pp_55p.is_some() {
            return;
        }

        if processed_sat_amount >= total_sat_supply * 55 / 100 {
            pp_55p.replace(price);
        }

        if pp_median.is_some() {
            return;
        }

        if processed_sat_amount >= total_sat_supply / 2 {
            pp_median.replace(price);
        }

        if pp_45p.is_some() {
            return;
        }

        if processed_sat_amount >= total_sat_supply * 45 / 100 {
            pp_45p.replace(price);
        }

        if pp_40p.is_some() {
            return;
        }

        if processed_sat_amount >= total_sat_supply * 40 / 100 {
            pp_40p.replace(price);
        }

        if pp_35p.is_some() {
            return;
        }

        if processed_sat_amount >= total_sat_supply * 35 / 100 {
            pp_35p.replace(price);
        }

        if pp_30p.is_some() {
            return;
        }

        if processed_sat_amount >= total_sat_supply * 30 / 100 {
            pp_30p.replace(price);
        }

        if pp_25p.is_some() {
            return;
        }

        if processed_sat_amount >= total_sat_supply / 4 {
            pp_25p.replace(price);
        }

        if pp_20p.is_some() {
            return;
        }

        if processed_sat_amount >= total_sat_supply / 5 {
            pp_20p.replace(price);
        }

        if pp_15p.is_some() {
            return;
        }

        if processed_sat_amount >= total_sat_supply * 15 / 100 {
            pp_15p.replace(price);
        }

        if pp_10p.is_some() {
            return;
        }

        if processed_sat_amount >= total_sat_supply / 10 {
            pp_10p.replace(price);
        }

        if pp_05p.is_some() {
            return;
        }

        if processed_sat_amount >= total_sat_supply / 20 {
            pp_05p.replace(price);
        }
    }
}
