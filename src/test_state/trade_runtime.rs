use crate::{
    test_state::{buffer::BUFFER, gw_values::GW_VALUES, state::STATE},
    trade_runtime::*,
};

use bc_trade_state::test_state::trade_state::TRADE_STATE;
use bc_utils_lg::prelude::*;

pub static TRADE_RUNTIME: LazyLock<fn() -> TradeRuntime<'static, 'static>> = LazyLock::new(|| {
    || TradeRuntime {
        buffer: BUFFER.clone(),
        gw_values: GW_VALUES(),
        state: STATE(),
        trade_state: TRADE_STATE(),
        symbol: "",
    }
});
