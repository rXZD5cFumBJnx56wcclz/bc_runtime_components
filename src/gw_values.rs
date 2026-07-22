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
    types::maps::PACK_TYPE as FA,
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
        s: &'a SETTINGS,
        fa_indicators: &FA<SETTINGS_IND, Box<dyn Indicator>>,
        fa_signals: &FA<SETTINGS_SIGNAL, Box<dyn SignalReady>>,
        fa_signals_train: &FA<SETTINGS_SIGNAL, Box<dyn SignalTrain>>,
        fa_order_filters: &FA<SETTINGS_ORDER_FILTER, Box<dyn OrderFilter>>,
        fa_orders_collectors: &FA<SETTINGS_ORDER_COLLECTOR, Box<dyn OrderCollector>>,
        fa_utils_state: &FA<SETTINGS_UTIL_STATE, Box<dyn UtilState>>,
        // transposed
        src: &[Vec<f64>],
    ) -> Self {
        let bind = Indicators::new(&s.indications, fa_indicators, src);
        Self {
            signals: Signals::new(
                &s.signals,
                &s.indications,
                fa_signals,
                src,
                &bind.indicators_without_bf,
            ),
            signals_train: SignalsTrain::new(
                &s.signals_train,
                &s.indications,
                fa_signals_train,
                src,
                &bind.indicators_without_bf,
            ),
            indicators: bind,
            order_creators: OrderCreators::new(&s.order_creators),
            order_filters: OrderFilters::new(&s.order_filters, fa_order_filters),
            orders_collectors: <OrdersCollectors as OrdersCollectorsExt>::new(
                &s.order_collectors,
                fa_orders_collectors,
            ),
            utils_state: UtilsState::new(&s.utils_state, fa_utils_state),
        }
    }
}
