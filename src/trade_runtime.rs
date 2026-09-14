/// Provides a unified structure for managing all components within the runtime.
use std::error::Error;

use bc_packs::packs::Packs;
use bc_trade_state::prelude::*;
use bc_utils_lg::structs::settings::SETTINGS;

use crate::buffer::Buffer;
use crate::gw_values::GWValues;
use crate::stage::{STAGES_EXECUTE, STAGES_STEP};
use crate::state::State;

#[derive(Default)]
pub struct TradeRuntime<'a, 'b> {
    pub buffer: Buffer,
    pub gw_values: GWValues<'a>,
    pub state: State<'a>,
    pub trade_state: TradeState<'a>,
    pub symbol: &'b str,
}

impl<'a, 'b> TradeRuntime<'a, 'b> {
    pub fn init_with(
        &mut self,
        buffer: Buffer,
        s: &'a SETTINGS,
        packs: &Packs,
        stage_end: &str,
        symbol: &'b str,
    ) {
        self.buffer = buffer;
        self.buffer.transpose_set();
        self.gw_values
            .init_empty_with(&s.pipeline, packs, stage_end);
        self.gw_values
            .init_bf_with(&self.buffer, &s.pipeline, stage_end);
        self.buffer.transpose_set();
        self.symbol = symbol;
    }
}

impl<'a, 'b> TradeRuntime<'a, 'b> {
    pub fn step_with(&mut self, s: &'a SETTINGS, stage_end: &str) {
        self.buffer.transpose_set();
        for (stage, func) in STAGES_STEP.iter().zip([
            TradeRuntime::step_ind,
            TradeRuntime::step_signals_train,
            TradeRuntime::step_signals,
            TradeRuntime::step_utils_state,
            TradeRuntime::step_order_creators,
            TradeRuntime::step_order_filters,
        ]) {
            func(self, s);
            if *stage == stage_end {
                self.buffer.transpose_set();
                return;
            }
        }
        self.buffer.transpose_set();
    }

    pub fn step_ind(&mut self, s: &'a SETTINGS) {
        self.state.indications = self
            .gw_values
            .indicators
            .series(&self.buffer, &s.pipeline.indications);
    }

    pub fn step_signals_train(&mut self, s: &'a SETTINGS) {
        self.state.signals_train = self.gw_values.signals_train.series(
            &self.buffer,
            &s.pipeline.signals_train,
            &self.state.indications,
        );
    }

    pub fn step_signals(&mut self, s: &'a SETTINGS) {
        self.state.signals = self.gw_values.signals.series(
            &self.buffer,
            &s.pipeline.signals,
            &self.state.indications,
            &self.state.signals_train,
        );
    }

    pub fn step_utils_state(&mut self, s: &'a SETTINGS) {
        self.state.utils_state = self.gw_values.utils_state.series(
            &self.trade_state,
            &self.buffer,
            &s.global.trade,
            &s.pipeline.utils_state,
            &self.state.indications,
            &self.state.signals,
        );
    }

    pub fn step_order_creators(&mut self, s: &'a SETTINGS) {
        self.state.orders = self.gw_values.order_creator.series(
            &s.pipeline.order_creators,
            &s.global.trade,
            self.symbol,
            &self.state.signals,
            &self.state.indications,
            &self.state.utils_state,
        );
    }

    pub fn step_order_filters(&mut self, s: &'a SETTINGS) {
        self.state.orders_filtered = self.gw_values.order_filters.series(
            &self.state.orders,
            &self.buffer,
            &s.pipeline.order_filters,
            &self.state.indications,
            &self.state.utils_state,
            &self.state.signals,
            &self.trade_state,
        );
    }
}

impl<'a, 'b> TradeRuntime<'a, 'b> {
    pub fn step_execute_with(&mut self, stage_end: &str) -> Result<(), Box<dyn Error>> {
        for (stage, func) in STAGES_EXECUTE
            .iter()
            .zip([TradeRuntime::step_execute_order_collectors])
        {
            func(self)?;
            if *stage == stage_end {
                return Ok(());
            }
        }
        Ok(())
    }

    pub fn step_execute_order_collectors(&mut self) -> Result<(), Box<dyn Error>> {
        self.gw_values
            .orders_collectors
            .collect_orders(&self.trade_state);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bc_test_kit::prelude::*;

    #[test]
    fn init_with_res_1() {
        let mut t = TradeRuntime::default();
        t.init_with(
            Buffer::from_row(SRC.clone(), 50),
            &SETTINGS_ALL,
            &PACKS,
            "",
            "",
        );
        assert!(!t.buffer.is_empty());
        assert!(!t.gw_values.indicators.0.is_empty());
        assert!(!t.gw_values.signals.0.is_empty());
        assert!(!t.gw_values.signals_train.0.is_empty());
        assert!(!t.gw_values.order_filters.0.is_empty());
        assert!(!t.gw_values.orders_collectors.0.is_empty());
    }

    #[test]
    fn step_with_res_1() {
        let mut t = TradeRuntime::default();
        t.init_with(
            Buffer::from_row(SRC[..49].to_vec(), 49),
            &SETTINGS_ALL,
            &PACKS,
            "",
            "",
        );
        t.buffer.update(SRC_EL.clone());
        t.step_with(&SETTINGS_ALL, "indications");
        assert_eq_pr!(dbg!(&INDICATIONS_STATE.clone()), dbg!(&t.state.indications));
        t.init_with(
            Buffer::from_row(SRC[..49].to_vec(), 49),
            &SETTINGS_ALL,
            &PACKS,
            "",
            "",
        );
        t.step_with(&SETTINGS_ALL, "signals");
        assert_eq_pr!(&SIGNALS_STATE.clone(), &t.state.signals);
    }

    #[test]
    fn step_execute_with_res_1() {
        let mut t = TradeRuntime::default();
        t.init_with(
            Buffer::from_row(SRC[..49].to_vec(), 49),
            &SETTINGS_ALL,
            &PACKS,
            "",
            "",
        );
        t.buffer.update(SRC_EL.clone());
        t.step_with(&SETTINGS_ALL, "");
        t.trade_state
            .orders
            .borrow_mut()
            .insert("order", vec![Order::default()]);
        assert!(!t.trade_state.orders.borrow().is_empty());
        t.trade_state.positions.borrow_mut().clear();
        assert!(t.trade_state.positions.borrow().is_empty());
        t.step_execute_with("").unwrap();
        t.trade_state.clear();
        for orders in t.trade_state.orders.borrow().values() {
            assert!(orders.is_empty());
        }
    }
}
