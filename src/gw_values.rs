use bc_order_collectors_gw::gw::OrderCollectors;
use bc_order_creator_gw::gw::OrderCreator;
use bc_order_filters_gw::gw::OrderFilters;
use bc_packs::packs::Packs;
use bc_utils_lg::structs::settings::SETTINGS_PIPELINE;

use bc_indicators_gw::gw::Indicators;
use bc_signals_gw::gw::Signals;
use bc_signals_train_gw::gw::SignalsTrain;
use bc_utils_state_gw::gw::UtilsState;

use crate::stage::STAGES;

#[derive(Default)]
pub struct GWValues<'a> {
    pub indicators: Indicators<'a>,
    pub signals_train: SignalsTrain<'a>,
    pub signals: Signals<'a>,
    pub utils_state: UtilsState<'a>,
    pub order_creator: OrderCreator,
    pub order_filters: OrderFilters<'a>,
    pub orders_collectors: OrderCollectors,
}

impl<'a> GWValues<'a> {
    pub fn init_empty_with(&mut self, s: &'a SETTINGS_PIPELINE, packs: &Packs, stage_end: &str) {
        for (stage, func) in STAGES.iter().zip([
            Self::init_empty_ind,
            Self::init_empty_signals_train,
            Self::init_empty_signals,
            Self::init_empty_utils_state,
            Self::init_empty_order_filters,
            Self::init_empty_orders_collectors,
        ]) {
            func(self, s, packs);
            if *stage == stage_end {
                return;
            }
        }
    }
    pub fn init_empty_ind(&mut self, s: &'a SETTINGS_PIPELINE, packs: &Packs) {
        self.indicators.init_empty(&s.indications, &packs.ind);
    }

    pub fn init_empty_signals_train(&mut self, s: &'a SETTINGS_PIPELINE, packs: &Packs) {
        self.signals_train
            .init_empty(&s.signals_train, &packs.signals_train);
    }

    pub fn init_empty_signals(&mut self, s: &'a SETTINGS_PIPELINE, packs: &Packs) {
        self.signals.init_empty(&s.signals, &packs.signals);
    }

    pub fn init_empty_utils_state(&mut self, s: &'a SETTINGS_PIPELINE, packs: &Packs) {
        self.utils_state.init(&s.utils_state, &packs.utils_state);
    }

    pub fn init_empty_order_filters(&mut self, s: &'a SETTINGS_PIPELINE, packs: &Packs) {
        self.order_filters
            .init(&s.order_filters, &packs.order_filters);
    }

    pub fn init_empty_orders_collectors(&mut self, s: &'a SETTINGS_PIPELINE, packs: &Packs) {
        self.orders_collectors
            .init(&s.order_collectors, &packs.orders_collectors);
    }
}

impl<'a> GWValues<'a> {
    pub fn init_bf_with(&mut self, src: &[Vec<f64>], s: &'a SETTINGS_PIPELINE, stage_end: &str) {
        dbg!(src.len());
        for (stage, func) in STAGES.iter().zip([
            Self::init_bf_ind,
            Self::init_bf_signals_train,
            Self::init_bf_signals,
        ]) {
            func(self, src, s);
            if *stage == stage_end {
                return;
            }
        }
    }

    pub fn init_bf_ind(&mut self, src: &[Vec<f64>], s: &'a SETTINGS_PIPELINE) {
        self.indicators.init_bf(src, &s.indications);
    }

    pub fn init_bf_signals_train(&mut self, src: &[Vec<f64>], s: &'a SETTINGS_PIPELINE) {
        self.signals_train
            .init_bf(src, &s.signals_train, &s.indications, &self.indicators);
    }

    pub fn init_bf_signals(&mut self, src: &[Vec<f64>], s: &'a SETTINGS_PIPELINE) {
        self.signals.init_bf(
            src,
            &s.signals,
            &s.indications,
            &s.signals_train,
            &self.indicators,
            &self.signals_train,
        );
    }
}

impl<'a> GWValues<'a> {
    pub fn init_bf(&mut self, buffer: &[Vec<f64>], s: &SETTINGS_PIPELINE) {
        self.indicators.init_bf(buffer, &s.indications);
        self.signals_train
            .init_bf(buffer, &s.signals_train, &s.indications, &self.indicators);
        self.signals.init_bf(
            buffer,
            &s.signals,
            &s.indications,
            &s.signals_train,
            &self.indicators,
            &self.signals_train,
        );
        self.order_filters.init_bf();
    }
}

impl<'a> GWValues<'a> {
    pub fn w_all(&self, s: SETTINGS_PIPELINE) -> usize {
        self.indicators.w_all(&s.indications)
            + self.signals_train.w_all(&s.signals_train)
            + self.signals.w_all(&s.signals)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bc_test_kit::prelude::*;

    #[test]
    fn init_empty_with_res_1() {
        let mut res = GWValues::default();
        res.init_empty_with(&PIPELINE, &PACKS, "");
        assert!(!res.signals.0.is_empty());
    }

    #[test]
    fn init_bf_with_res_1() {
        let mut res = GWValues::default();
        res.init_empty_with(&PIPELINE, &PACKS, "");
        res.init_bf_with(&SRC_TRANSPOSE, &PIPELINE, "");
        assert!(!res.signals.0.is_empty());
    }
}
