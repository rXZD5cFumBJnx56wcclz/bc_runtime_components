use crate::state::*;
use bc_indicators_gw::test_state::INDICATIONS_STATE;
use bc_order_creator_gw::test_state::ORDER_CREATOR_STATE;
use bc_order_filters_gw::test_state::ORDER_FILTERS_STATE_RAW;
use bc_signals_gw::test_state::SIGNALS_STATE;
use bc_signals_train_gw::test_state::SIGNALS_TRAIN_STATE;
use bc_utils_lg::prelude::*;
use bc_utils_state_gw::test_state::UTILS_STATE_STATE;

pub static STATE: LazyLock<fn() -> State<'static>> = LazyLock::new(|| {
    || State {
        indications: INDICATIONS_STATE.clone(),
        signals_train: SIGNALS_TRAIN_STATE.clone(),
        signals: SIGNALS_STATE.clone(),
        utils_state: UTILS_STATE_STATE.clone(),
        orders: ORDER_CREATOR_STATE.clone(),
        orders_filtered: ORDER_FILTERS_STATE_RAW(),
    }
});
