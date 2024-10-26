use crate::structs::Epoch;

use super::{SplitByUTXOCohort, UTXOFilter};

pub const UTXO_FILTERS: SplitByUTXOCohort<UTXOFilter> = SplitByUTXOCohort {
    up_to_1d: UTXOFilter::To(1),
    up_to_1w: UTXOFilter::To(7),
    up_to_1m: UTXOFilter::To(30),
    up_to_2m: UTXOFilter::To(2 * 30),
    up_to_3m: UTXOFilter::To(3 * 30),
    up_to_4m: UTXOFilter::To(4 * 30),
    up_to_5m: UTXOFilter::To(5 * 30),
    up_to_6m: UTXOFilter::To(6 * 30),
    up_to_1y: UTXOFilter::To(365),
    up_to_2y: UTXOFilter::To(2 * 365),
    up_to_3y: UTXOFilter::To(3 * 365),
    up_to_5y: UTXOFilter::To(5 * 365),
    up_to_7y: UTXOFilter::To(7 * 365),
    up_to_10y: UTXOFilter::To(10 * 365),
    up_to_15y: UTXOFilter::To(15 * 365),

    from_1d_to_1w: UTXOFilter::FromTo { from: 1, to: 7 },
    from_1w_to_1m: UTXOFilter::FromTo { from: 7, to: 30 },
    from_1m_to_3m: UTXOFilter::FromTo {
        from: 30,
        to: 3 * 30,
    },
    from_3m_to_6m: UTXOFilter::FromTo {
        from: 3 * 30,
        to: 6 * 30,
    },
    from_6m_to_1y: UTXOFilter::FromTo {
        from: 6 * 30,
        to: 365,
    },
    from_1y_to_2y: UTXOFilter::FromTo {
        from: 365,
        to: 2 * 365,
    },
    from_2y_to_3y: UTXOFilter::FromTo {
        from: 2 * 365,
        to: 3 * 365,
    },
    from_3y_to_5y: UTXOFilter::FromTo {
        from: 3 * 365,
        to: 5 * 365,
    },
    from_5y_to_7y: UTXOFilter::FromTo {
        from: 5 * 365,
        to: 7 * 365,
    },
    from_7y_to_10y: UTXOFilter::FromTo {
        from: 7 * 365,
        to: 10 * 365,
    },
    from_10y_to_15y: UTXOFilter::FromTo {
        from: 10 * 365,
        to: 15 * 365,
    },

    from_1y: UTXOFilter::From(365),
    from_2y: UTXOFilter::From(2 * 365),
    from_4y: UTXOFilter::From(4 * 365),
    from_10y: UTXOFilter::From(10 * 365),
    from_15y: UTXOFilter::From(15 * 365),

    epoch_1: UTXOFilter::Epoch(Epoch(1)),
    epoch_2: UTXOFilter::Epoch(Epoch(2)),
    epoch_3: UTXOFilter::Epoch(Epoch(3)),
    epoch_4: UTXOFilter::Epoch(Epoch(4)),
    epoch_5: UTXOFilter::Epoch(Epoch(5)),

    sth: UTXOFilter::To(155),
    lth: UTXOFilter::From(155),
};
