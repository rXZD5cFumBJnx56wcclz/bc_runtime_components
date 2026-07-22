use std::ptr;

use bc_indicators_gw::gw::IndicatorsGateway;
use bc_order_creator_gw::gw::OrderCreatorsGateway;
use bc_order_filters_gw::gw::OrderFilterGateway;
use bc_orders_collectors_gw::gw::OrdersCollectorsGateway;
use bc_signals_gw::gw::SignalsGateway;
use bc_signals_train_gw::gw::SignalsTrainGateway;
use bc_utils_lg::structs::settings::SETTINGS;
use bc_utils_state_gw::gw::UtilsStateGateway;

use crate::gw_values::GWValues;

pub struct GW<'a> {
    pub indicators_gw: IndicatorsGateway<'a>,
    pub signals_gw: SignalsGateway<'a>,
    pub signals_train_gw: SignalsTrainGateway<'a>,
    pub order_creators_gw: OrderCreatorsGateway<'a>,
    pub order_filters_gw: OrderFilterGateway<'a>,
    pub orders_collectors_gw: OrdersCollectorsGateway,
    pub utils_state_gw: UtilsStateGateway<'a>,
}

impl<'a> GW<'a> {
    pub fn new_with_s(s: &'a SETTINGS) -> Self {
        Self {
            indicators_gw: IndicatorsGateway::new(ptr::null(), &s.indications),
            signals_gw: SignalsGateway::new(ptr::null(), ptr::null(), &s.signals, &s.indications),
            signals_train_gw: SignalsTrainGateway::new(
                ptr::null(),
                ptr::null(),
                &s.signals_train,
                &s.indications,
            ),
            orders_collectors_gw: OrdersCollectorsGateway::new(ptr::null()),
            order_creators_gw: OrderCreatorsGateway::new(ptr::null(), &s.order_creators),
            order_filters_gw: OrderFilterGateway::new(ptr::null(), &s.order_filters),
            utils_state_gw: UtilsStateGateway::new(ptr::null(), &s.utils_state),
        }
    }

    pub fn update(&mut self, gw_values: &GWValues<'a>) {
        self.indicators_gw.indicators = &gw_values.indicators;
        self.signals_gw.indicators = &gw_values.indicators;
        self.signals_train_gw.indicators = &gw_values.indicators;
        self.signals_gw.signals = &gw_values.signals;
        self.signals_train_gw.signals_train = &gw_values.signals_train;
        self.order_creators_gw.order_creators = &gw_values.order_creators;
        self.order_filters_gw.order_filters = &gw_values.order_filters;
        self.orders_collectors_gw.orders_collectors = &gw_values.orders_collectors;
        self.utils_state_gw.utils_state = &gw_values.utils_state;
    }
}
