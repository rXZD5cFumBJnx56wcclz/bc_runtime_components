use std::error::Error;

use bc_packs::packs::Packs;
use bc_trade_state::state::StepState;
use bc_utils_lg::structs::settings::{SETTINGS, SETTINGS_PIPELINE};

use crate::buffer::Buffer;
use crate::gw_values::GWValues;
use crate::stage::{STAGES_EXECUTE, STAGES_STEP};
use crate::state::State;

#[derive(Default)]
pub struct TradeRuntime<'a, 'b> {
    pub gw_values: GWValues<'a>,
    pub state: State<'a>,
    pub symbol: &'b str,
}

impl<'a, 'b> TradeRuntime<'a, 'b> {
    pub fn init_with(
        &mut self,
        buffer: &mut Buffer,
        s: &'a SETTINGS,
        packs: &Packs,
        stage_end: &str,
        symbol: &'b str,
    ) {
        self.state.init(&s.global.trade, &buffer);
        buffer.transpose_set();
        self.gw_values
            .init_with(&buffer, &s.pipeline, packs, stage_end);
        buffer.transpose_set();
        self.symbol = symbol;
    }
}

impl<'a, 'b> TradeRuntime<'a, 'b> {
    pub fn step_with(&mut self, buffer: &mut Buffer, s: &'a SETTINGS_PIPELINE, stage_end: &str) {
        buffer.transpose_set();
        for (stage, func) in STAGES_STEP.iter().zip([
            TradeRuntime::step_ind,
            TradeRuntime::step_signals_train,
            TradeRuntime::step_signals,
            TradeRuntime::step_utils_state,
            TradeRuntime::step_order_creators,
            TradeRuntime::step_order_filters,
        ]) {
            func(self, buffer, s);
            if *stage == stage_end {
                buffer.transpose_set();
                return;
            }
        }
        buffer.transpose_set();
    }

    pub fn step_ind(&mut self, buffer: &[Vec<f64>], s: &'a SETTINGS_PIPELINE) {
        self.state.indications = self.gw_values.indicators.series(buffer, &s.indications);
    }

    pub fn step_signals_train(&mut self, buffer: &[Vec<f64>], s: &'a SETTINGS_PIPELINE) {
        self.state.signals_train =
            self.gw_values
                .signals_train
                .series(buffer, &s.signals_train, &self.state.indications);
    }

    pub fn step_signals(&mut self, buffer: &[Vec<f64>], s: &'a SETTINGS_PIPELINE) {
        self.state.signals = self.gw_values.signals.series(
            buffer,
            &s.signals,
            &self.state.indications,
            &self.state.signals_train,
        );
    }

    pub fn step_utils_state(&mut self, buffer: &[Vec<f64>], s: &'a SETTINGS_PIPELINE) {
        self.state.utils_state = self.gw_values.utils_state.series(
            &self.state.trade_state,
            buffer,
            &s.utils_state,
            &self.state.indications,
            &self.state.signals,
        );
    }

    pub fn step_order_creators(&mut self, _: &[Vec<f64>], s: &'a SETTINGS_PIPELINE) {
        self.state.orders = self.gw_values.order_creators.series(
            &s.order_creators,
            self.symbol,
            &self.state.signals,
            &self.state.indications,
            &self.state.utils_state,
        );
    }

    pub fn step_order_filters(&mut self, buffer: &[Vec<f64>], s: &'a SETTINGS_PIPELINE) {
        self.state
            .trade_state
            .step(self.gw_values.order_filters.series(
                &self.state.orders,
                buffer,
                &s.order_filters,
                &self.state.indications,
                &self.state.utils_state,
                &self.state.signals,
                &self.state.trade_state,
            ));
    }
}

impl<'a, 'b> TradeRuntime<'a, 'b> {
    pub fn execute_with(&mut self, buffer: &Buffer, stage_end: &str) -> Result<(), Box<dyn Error>> {
        for (stage, func) in STAGES_EXECUTE
            .iter()
            .zip([TradeRuntime::execute_orders_collectors])
        {
            func(self, buffer)?;
            if *stage == stage_end {
                return Ok(());
            }
        }
        Ok(())
    }

    pub fn execute_orders_collectors(&mut self, _: &Buffer) -> Result<(), Box<dyn Error>> {
        self.gw_values
            .orders_collectors
            .collect_orders(&self.state.trade_state);
        Ok(())
    }
}

impl<'a, 'b> TradeRuntime<'a, 'b> {
    pub fn update_src(&mut self, buffer: &Buffer) {
        self.state.src = buffer.last().unwrap().to_vec();
    }
}