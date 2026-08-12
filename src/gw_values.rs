use bc_order_creator_gw::gw::OrderCreators;
use bc_order_filters_gw::gw::OrderFilters;
use bc_orders_collectors_gw::gw::OrdersCollectors;
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
    pub order_creators: OrderCreators<'a>,
    pub order_filters: OrderFilters<'a>,
    pub orders_collectors: OrdersCollectors,
}

impl<'a> GWValues<'a> {
    pub fn init_with(
        &mut self,
        src: &[Vec<f64>],
        s: &'a SETTINGS_PIPELINE,
        packs: &Packs,
        stage_end: &str,
    ) {
        for (stage, func) in STAGES.iter().zip([
            Self::init_ind,
            Self::init_signals_train,
            Self::init_signals,
            Self::init_utils_state,
            Self::init_order_creators,
            Self::init_order_filters,
            Self::init_orders_collectors,
        ]) {
            func(self, src, s, packs);
            if *stage == stage_end {
                return;
            }
        }
    }

    pub fn init_ind(&mut self, src: &[Vec<f64>], s: &'a SETTINGS_PIPELINE, packs: &Packs) {
        self.indicators = Indicators::new(src, &s.indications, &packs.ind);
    }

    pub fn init_signals_train(
        &mut self,
        src: &[Vec<f64>],
        s: &'a SETTINGS_PIPELINE,
        packs: &Packs,
    ) {
        self.signals_train = SignalsTrain::new(
            src,
            &s.signals_train,
            &s.indications,
            &self.indicators,
            &packs.signals_train,
        );
    }

    pub fn init_signals(&mut self, src: &[Vec<f64>], s: &'a SETTINGS_PIPELINE, packs: &Packs) {
        self.signals = Signals::new(
            src,
            &s.signals,
            &s.indications,
            &s.signals_train,
            &self.indicators,
            &self.signals_train,
            &packs.signals,
        );
    }

    pub fn init_utils_state(&mut self, _: &[Vec<f64>], s: &'a SETTINGS_PIPELINE, packs: &Packs) {
        self.utils_state = UtilsState::new(&s.utils_state, &packs.utils_state);
    }

    pub fn init_order_creators(&mut self, _: &[Vec<f64>], s: &'a SETTINGS_PIPELINE, _: &Packs) {
        self.order_creators = OrderCreators::new(&s.order_creators);
    }

    pub fn init_order_filters(&mut self, _: &[Vec<f64>], s: &'a SETTINGS_PIPELINE, packs: &Packs) {
        self.order_filters = OrderFilters::new(&s.order_filters, &packs.order_filters);
    }

    pub fn init_orders_collectors(
        &mut self,
        _: &[Vec<f64>],
        s: &'a SETTINGS_PIPELINE,
        packs: &Packs,
    ) {
        self.orders_collectors =
            OrdersCollectors::new(&s.order_collectors, &packs.orders_collectors);
    }
}

impl<'a> GWValues<'a> {
    pub fn init_bf(&mut self, buffer: &[Vec<f64>], s: &SETTINGS_PIPELINE) {
        self
            .indicators
            .init_bf(buffer, &s.indications);
        self.signals_train.init_bf(
            buffer,
            &s.signals_train,
            &s.indications,
            &self.indicators,
        );
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude_tests::prelude::*;

    // #[test]
    // fn init_with_res_1() {
    //     let mut res = GWValues::default();
    //     assert_eq!(res.init_with(&SRC_TRANSPOSE, &S, packs, stage_end));
    // }
}