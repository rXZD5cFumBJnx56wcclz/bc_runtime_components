use bc_utils_lg::{
    structs::{
        settings::SETTINGS_TRADE,
        signals::Signal,
        trade::{Order, TradeState, Trigger},
    },
    types::maps::MAP,
};

pub struct State<'a> {
    pub indications: MAP<&'a str, f64>,
    pub signals: MAP<&'a str, Signal>,
    // pub signals_train: MAP<&'a str, f64>,
    pub res_utils_state: MAP<&'a str, f64>,
    pub orders: MAP<&'a str, (Order, bool, Option<Trigger>)>,
    pub trade_state: TradeState<'a>,
}

impl<'a> State<'a> {
    pub fn new(s: &SETTINGS_TRADE, src: &[Vec<f64>]) -> Self {
        Self {
            trade_state: TradeState::new(
                s.capital,
                src[src.len() - 1].to_vec(),
                src[src.len() - 2].to_vec(),
            ),
            indications: Default::default(),
            signals: Default::default(),
            res_utils_state: Default::default(),
            orders: Default::default(),
            // signals_train: Default::default(),
        }
    }
}
