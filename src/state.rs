use bc_trade_state::prelude::*;
use bc_utils_lg::prelude::*;

#[derive(Default)]
pub struct State<'a> {
    pub indications: MAP<&'a str, f64>,
    pub signals_train: MAP<&'a str, f64>,
    pub signals: MAP<&'a str, Signal>,
    pub utils_state: MAP<&'a str, f64>,
    pub orders: MAP<&'a str, OrderWrap>,
    pub orders_filtered: MAP<&'a str, (Option<*const OrderWrap>, bool)>,
}
