use bc_utils_lg::{
    structs::{
        settings::SETTINGS_TRADE,
        signals::Signal,
        trade::{Order, TradeState, Trigger},
    },
    types::maps::MAP,
};

#[derive(Default)]
pub struct State<'a> {
    pub src: Vec<f64>,
    pub indications: MAP<&'a str, f64>,
    pub signals_train: MAP<&'a str, f64>,
    pub signals: MAP<&'a str, Signal>,
    pub utils_state: MAP<&'a str, f64>,
    pub orders: MAP<&'a str, (Order, bool, Option<Trigger>)>,
    pub trade_state: TradeState<'a>,
}

impl<'a, 'b> State<'a> {
    pub fn init(&mut self, s: &SETTINGS_TRADE, src: &'b [Vec<f64>]) {
        self.src = src[src.len() - 1].to_vec();
        self.trade_state = TradeState::new(s.capital);
    }
}
