use std::error::Error;
use std::marker::PhantomPinned;
use std::pin::Pin;

use bc_indicators::main_trait::Indicator;
use bc_order_filters::main_trait::OrderFilter;
use bc_order_filters_gw::gw::OrderFilterUpdateBf;
use bc_orders_collectors::main_trait::OrderCollector;
use bc_signals::main_trait::SignalReady;
use bc_signals_train::main_trait::SignalTrain;
use bc_trade_state::state::StepState;
use bc_utils_lg::structs::settings::{SETTINGS_ORDER_FILTER, SETTINGS_UTIL_STATE};
use bc_utils_lg::{
    structs::settings::{SETTINGS, SETTINGS_IND, SETTINGS_ORDER_COLLECTOR, SETTINGS_SIGNAL},
    types::maps::PACK_TYPE as FA,
};
use bc_utils_state::main_trait::UtilState;
use pin_project::pin_project;

use crate::buffer::Buffer;
use crate::gw::GW;
use crate::gw_values::GWValues;
use crate::state::State;

#[pin_project]
pub struct TradeRuntime<'a,> {
    pub gw_values: GWValues<'a>,
    #[pin]
    pub gw: GW<'a>,
    pub state: State<'a>,
    _pin: PhantomPinned,
}

impl<'a,> TradeRuntime<'a,> {
    pub fn new(
        src: &mut Buffer,
        s: &'a SETTINGS,
        fa_indicators: &FA<SETTINGS_IND, Box<dyn Indicator>>,
        fa_signals: &FA<SETTINGS_SIGNAL, Box<dyn SignalReady>>,
        fa_signals_train: &FA<SETTINGS_SIGNAL, Box<dyn SignalTrain>>,
        fa_order_filters: &FA<SETTINGS_ORDER_FILTER, Box<dyn OrderFilter>>,
        fa_orders_collectors: &FA<SETTINGS_ORDER_COLLECTOR, Box<dyn OrderCollector>>,
        fa_utils_state: &FA<SETTINGS_UTIL_STATE, Box<dyn UtilState>>,
    ) -> Pin<Box<Self>> {
        let mut res = Box::pin(Self {
            gw_values: {
                src.transpose_set();
                let bind = GWValues::new(
                    s,
                    fa_indicators,
                    fa_signals,
                    fa_signals_train,
                    fa_order_filters,
                    fa_orders_collectors,
                    fa_utils_state,
                    src,
                );
                src.transpose_set();
                bind
            },
            gw: GW::new_with_s(s),
            state: State::new(&s.trade, src),
            _pin: PhantomPinned,
        });
        let outer_mut = unsafe { Pin::as_mut(&mut res).get_unchecked_mut() };
        outer_mut.gw.update(&outer_mut.gw_values);
        res
    }
}

impl<'a,> TradeRuntime<'a,> {
    pub fn step(self: Pin<&mut Self>, buffer: &mut Buffer, symbol: &str,) -> () {
        let project = self.project();
        buffer.transpose_set();
        project.state.indications = project.gw.indicators_gw.indications_series(&buffer);
        project.state.signals = project
            .gw
            .signals_gw
            .signals_series(&project.state.indications, &buffer);
        // let signals_train = project
        //     .signals_train_gw
        //     .signals_series(&indications, &buffer);
        project.state.res_utils_state = project.gw.utils_state_gw.series(
            &project.state.trade_state,
            buffer,
            &project.state.indications,
            &project.state.signals,
        );
        project.state.orders = project.gw.order_creators_gw.series(
            symbol,
            &project.state.signals,
            &project.state.indications,
            &project.state.res_utils_state,
        );
        project
            .state
            .trade_state
            .step(project.gw.order_filters_gw.series(
                &project.state.orders,
                buffer,
                &project.state.indications,
                &project.state.res_utils_state,
                &project.state.signals,
                &project.state.trade_state,
            ));
        buffer.transpose_set();
    }

    pub fn execute(self: Pin<&mut Self>, buffer: &mut Buffer) -> Result<(), Box<dyn Error>> {
        let project = self.project();
        project
            .state
            .trade_state
            .execute(&buffer[buffer.len() - 1], &buffer[buffer.len() - 2])
    }

    pub fn clear(self: Pin<&mut Self>) {
        let project = self.project();
        project.state.trade_state.clear();
    }

    pub fn update_bf(
        self: Pin<&mut Self>,
        s: &'a SETTINGS,
        buffer: &[Vec<f64>],
        fa_indicators: &FA<SETTINGS_IND, Box<dyn Indicator>>,
        fa_signals: &FA<SETTINGS_SIGNAL, Box<dyn SignalReady>>,
        fa_signals_train: &FA<SETTINGS_SIGNAL, Box<dyn SignalTrain>>,
    ) {
        let project = self.project();
        project
            .gw_values
            .indicators
            .update_bf(buffer, &s.indications, &fa_indicators);
        project.gw_values.signals.update_bf(
            buffer,
            s,
            &fa_signals,
            &project.gw_values.indicators.indicators_without_bf,
        );
        project.gw_values.signals_train.update_bf(
            buffer,
            s,
            &fa_signals_train,
            &project.gw_values.indicators.indicators_without_bf,
        );
        project.gw_values.order_filters.update_bf();
    }
}
