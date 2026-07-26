use bc_indicators::main_trait::Indicator;
use bc_order_creator_gw::gw::{OrderCreators, OrderCreatorsExt};
use bc_order_filters::main_trait::OrderFilter;
use bc_order_filters_gw::gw::{OrderFilters, OrderFiltersExt};
use bc_orders_collectors::main_trait::OrderCollector;
use bc_orders_collectors_gw::gw::{OrdersCollectors, OrdersCollectorsExt};
use bc_signals::main_trait::SignalReady;
use bc_signals_train::main_trait::SignalTrain;
use bc_utils_lg::{
    structs::settings::{
        SETTINGS, SETTINGS_IND, SETTINGS_ORDER_COLLECTOR, SETTINGS_ORDER_FILTER, SETTINGS_SIGNAL,
        SETTINGS_UTIL_STATE,
    },
    types::maps::PACK,
};

use bc_indicators_gw::gw::Indicators;
use bc_signals_gw::gw::Signals;
use bc_signals_train_gw::gw::SignalsTrain;
use bc_utils_state::main_trait::UtilState;
use bc_utils_state_gw::gw::{UtilsState, UtilsStateExt};

#[derive(Default)]
pub struct GWValues<'a> {
    pub indicators: Indicators<'a>,
    pub signals: Signals<'a>,
    pub signals_train: SignalsTrain<'a>,
    pub order_creators: OrderCreators<'a>,
    pub order_filters: OrderFilters<'a>,
    pub orders_collectors: OrdersCollectors,
    pub utils_state: UtilsState<'a>,
}

impl<'a> GWValues<'a> {
    pub fn new(
        // transposed
        src: &[Vec<f64>],
        s: &'a SETTINGS,
        pack_ind: &PACK<SETTINGS_IND, Box<dyn Indicator>>,
        pack_signals_train: &PACK<SETTINGS_SIGNAL, Box<dyn SignalTrain>>,
        pack_signals: &PACK<SETTINGS_SIGNAL, Box<dyn SignalReady>>,
        pack_order_filters: &PACK<SETTINGS_ORDER_FILTER, Box<dyn OrderFilter>>,
        pack_orders_collectors: &PACK<SETTINGS_ORDER_COLLECTOR, Box<dyn OrderCollector>>,
        pack_utils_state: &PACK<SETTINGS_UTIL_STATE, Box<dyn UtilState>>,
    ) -> Self {
        let indicators = Indicators::new(&s.pipeline.indications, pack_ind, src);
        let signals_train = SignalsTrain::new(
            &s.pipeline.signals_train,
            &s.pipeline.indications,
            pack_signals_train,
            src,
            &indicators.indicators_without_bf,
        );
        Self {
            signals: Signals::new(
                &s.pipeline.signals,
                &s.pipeline.signals_train,
                &s.pipeline.indications,
                pack_signals,
                src,
                &signals_train.signals_train_without_bf,
                &indicators.indicators_without_bf,
            ),
            signals_train,
            indicators,
            order_creators: OrderCreators::new(&s.pipeline.order_creators),
            order_filters: OrderFilters::new(&s.pipeline.order_filters, pack_order_filters),
            orders_collectors: <OrdersCollectors as OrdersCollectorsExt>::new(
                &s.pipeline_execute.order_collectors,
                pack_orders_collectors,
            ),
            utils_state: UtilsState::new(&s.pipeline.utils_state, pack_utils_state),
        }
    }

    pub fn new_with_ind(
        src: &[Vec<f64>],
        s: &'a SETTINGS,
        pack: &PACK<SETTINGS_IND, Box<dyn Indicator>>,
    ) -> Self {
        Self {
            indicators: Indicators::new(&s.pipeline.indications, pack, src),
            ..Default::default()
        }
    }

    pub fn new_with_signals_train(
        src: &[Vec<f64>],
        s: &'a SETTINGS,
        pack_ind: &PACK<SETTINGS_IND, Box<dyn Indicator>>,
        pack_signals_train: &PACK<SETTINGS_SIGNAL, Box<dyn SignalTrain>>,
    ) -> Self {
        let indicators = Indicators::new(&s.pipeline.indications, pack_ind, src);
        Self {
            signals_train: SignalsTrain::new(
                &s.pipeline.signals_train,
                &s.pipeline.indications,
                pack_signals_train,
                src,
                &indicators.indicators_without_bf,
            ),
            indicators,
            ..Default::default()
        }
    }

    pub fn new_with_signals(
        src: &[Vec<f64>],
        s: &'a SETTINGS,
        pack_ind: &PACK<SETTINGS_IND, Box<dyn Indicator>>,
        pack_signals_train: &PACK<SETTINGS_SIGNAL, Box<dyn SignalTrain>>,
        pack_signals: &PACK<SETTINGS_SIGNAL, Box<dyn SignalReady>>,
    ) -> Self {
        let indicators = Indicators::new(&s.pipeline.indications, pack_ind, src);
        let signals_train = SignalsTrain::new(
            &s.pipeline.signals_train,
            &s.pipeline.indications,
            pack_signals_train,
            src,
            &indicators.indicators_without_bf,
        );
        Self {
            signals: Signals::new(
                &s.pipeline.signals,
                &s.pipeline.signals_train,
                &s.pipeline.indications,
                pack_signals,
                src,
                &signals_train.signals_train_without_bf,
                &indicators.indicators_without_bf,
            ),
            indicators,
            signals_train,
            ..Default::default()
        }
    }

    pub fn new_with_utils_state(
        src: &[Vec<f64>],
        s: &'a SETTINGS,
        pack_ind: &PACK<SETTINGS_IND, Box<dyn Indicator>>,
        pack_signals_train: &PACK<SETTINGS_SIGNAL, Box<dyn SignalTrain>>,
        pack_signals: &PACK<SETTINGS_SIGNAL, Box<dyn SignalReady>>,
        pack_utils_state: &PACK<SETTINGS_UTIL_STATE, Box<dyn UtilState>>,
    ) -> Self {
        let indicators = Indicators::new(&s.pipeline.indications, pack_ind, src);
        let signals_train = SignalsTrain::new(
            &s.pipeline.signals_train,
            &s.pipeline.indications,
            pack_signals_train,
            src,
            &indicators.indicators_without_bf,
        );
        Self {
            signals: Signals::new(
                &s.pipeline.signals,
                &s.pipeline.signals_train,
                &s.pipeline.indications,
                pack_signals,
                src,
                &signals_train.signals_train_without_bf,
                &indicators.indicators_without_bf,
            ),
            indicators,
            signals_train,
            utils_state: <UtilsState as UtilsStateExt>::new(
                &s.pipeline.utils_state,
                pack_utils_state,
            ),
            ..Default::default()
        }
    }

    pub fn new_with_order_creators(
        src: &[Vec<f64>],
        s: &'a SETTINGS,
        pack_ind: &PACK<SETTINGS_IND, Box<dyn Indicator>>,
        pack_signals_train: &PACK<SETTINGS_SIGNAL, Box<dyn SignalTrain>>,
        pack_signals: &PACK<SETTINGS_SIGNAL, Box<dyn SignalReady>>,
        pack_utils_state: &PACK<SETTINGS_UTIL_STATE, Box<dyn UtilState>>,
    ) -> Self {
        let indicators = Indicators::new(&s.pipeline.indications, pack_ind, src);
        let signals_train = SignalsTrain::new(
            &s.pipeline.signals_train,
            &s.pipeline.indications,
            pack_signals_train,
            src,
            &indicators.indicators_without_bf,
        );
        Self {
            signals: Signals::new(
                &s.pipeline.signals,
                &s.pipeline.signals_train,
                &s.pipeline.indications,
                pack_signals,
                src,
                &signals_train.signals_train_without_bf,
                &indicators.indicators_without_bf,
            ),
            indicators,
            signals_train,
            utils_state: <UtilsState as UtilsStateExt>::new(
                &s.pipeline.utils_state,
                pack_utils_state,
            ),
            order_creators: OrderCreators::new(&s.pipeline.order_creators),
            ..Default::default()
        }
    }

    pub fn new_with_order_filters(
        src: &[Vec<f64>],
        s: &'a SETTINGS,
        pack_ind: &PACK<SETTINGS_IND, Box<dyn Indicator>>,
        pack_signals_train: &PACK<SETTINGS_SIGNAL, Box<dyn SignalTrain>>,
        pack_signals: &PACK<SETTINGS_SIGNAL, Box<dyn SignalReady>>,
        pack_utils_state: &PACK<SETTINGS_UTIL_STATE, Box<dyn UtilState>>,
        pack_order_filters: &PACK<SETTINGS_ORDER_FILTER, Box<dyn OrderFilter>>,
    ) -> Self {
        let indicators = Indicators::new(&s.pipeline.indications, pack_ind, src);
        let signals_train = SignalsTrain::new(
            &s.pipeline.signals_train,
            &s.pipeline.indications,
            pack_signals_train,
            src,
            &indicators.indicators_without_bf,
        );
        Self {
            signals: Signals::new(
                &s.pipeline.signals,
                &s.pipeline.signals_train,
                &s.pipeline.indications,
                pack_signals,
                src,
                &signals_train.signals_train_without_bf,
                &indicators.indicators_without_bf,
            ),
            indicators,
            signals_train,
            utils_state: <UtilsState as UtilsStateExt>::new(
                &s.pipeline.utils_state,
                pack_utils_state,
            ),
            order_creators: OrderCreators::new(&s.pipeline.order_creators),
            order_filters: OrderFilters::new(&s.pipeline.order_filters, pack_order_filters),
            ..Default::default()
        }
    }
}
