use std::error::Error;
use std::marker::PhantomPinned;
use std::pin::Pin;

use bc_indicators_gw::gw::{Indicators, IndicatorsGateway};
use bc_order_creator_gw::gw::{OrderCreators, OrderCreatorsExt, OrderCreatorsGateway};
use bc_order_filters_gw::gw::{
    OrderFilterGateway, OrderFilterUpdateBf, OrderFilters, OrderFiltersExt,
};
use bc_orders_collectors_gw::gw::{OrdersCollectors, OrdersCollectorsExt, OrdersCollectorsGateway};
use bc_packs::packs::Packs;
use bc_signals_gw::gw::{Signals, SignalsGateway};
use bc_signals_train_gw::gw::{SignalsTrain, SignalsTrainGateway};
use bc_trade_state::state::StepState;
use bc_utils_lg::structs::settings::SETTINGS;
use bc_utils_state_gw::gw::{UtilsState, UtilsStateExt, UtilsStateGateway};
use pin_project::pin_project;

use crate::buffer::Buffer;
use crate::gw::GW;
use crate::gw_values::GWValues;
use crate::stage::{STAGES, STAGES_EXECUTE, STAGES_STEP};
use crate::state::State;

#[derive(Default)]
#[pin_project]
pub struct TradeRuntime<'a, 'b> {
    #[pin]
    pub gw_values: GWValues<'a>,
    pub gw: GW<'a>,
    pub state: State<'a>,
    pub symbol: &'b str,
    #[pin]
    _pin: PhantomPinned,
}

impl<'a, 'b> TradeRuntime<'a, 'b> {
    pub fn init_with(
        src: &mut Buffer,
        s: &'a SETTINGS,
        symbol: &'b str,
        packs: &Packs,
        stage_end: &str,
    ) -> Pin<Box<Self>> {
        let mut res = Box::pin(Self::default());
        *res.as_mut().project().symbol = symbol;
        res.as_mut().project().state.trade_state.capital = s.global.trade.capital;
        src.transpose_set();
        for (stage, func) in STAGES.iter().zip([
            TradeRuntime::init_src,
            TradeRuntime::init_ind,
            TradeRuntime::init_signals_train,
            TradeRuntime::init_signals,
            TradeRuntime::init_utils_state,
            TradeRuntime::init_order_creators,
            TradeRuntime::init_order_filters,
            TradeRuntime::init_orders_collectors,
        ]) {
            func(res.as_mut(), src, s, packs);
            if *stage == stage_end {
                src.transpose_set();
                return res;
            }
        }
        src.transpose_set();
        res
    }

    pub fn init_src(self: Pin<&mut Self>, src: &Buffer, _: &'a SETTINGS, _: &Packs) {
        self.project().state.src = src.iter().map(|v| *v.last().unwrap()).collect::<Vec<f64>>();
    }

    pub fn init_ind(self: Pin<&mut Self>, src: &Buffer, s: &'a SETTINGS, packs: &Packs) {
        let mut project = self.project();
        unsafe { project.gw_values.as_mut().get_unchecked_mut() }.indicators =
            Indicators::new(&s.pipeline.indications, &packs.ind, src);
        project.gw.indicators_gw =
            IndicatorsGateway::new(&project.gw_values.indicators, &s.pipeline.indications);
    }

    pub fn init_signals_train(self: Pin<&mut Self>, src: &Buffer, s: &'a SETTINGS, packs: &Packs) {
        let mut project = self.project();
        unsafe { project.gw_values.as_mut().get_unchecked_mut() }.signals_train = SignalsTrain::new(
            &s.pipeline.signals_train,
            &s.pipeline.indications,
            &packs.signals_train,
            src,
            &project.gw_values.indicators.indicators_without_bf,
        );
        project.gw.signals_train_gw = SignalsTrainGateway::new(
            &project.gw_values.signals_train,
            &project.gw_values.indicators,
            &s.pipeline.signals_train,
            &s.pipeline.indications,
        );
    }

    pub fn init_signals(self: Pin<&mut Self>, src: &Buffer, s: &'a SETTINGS, packs: &Packs) {
        let mut project = self.project();
        unsafe { project.gw_values.as_mut().get_unchecked_mut() }.signals = Signals::new(
            &s.pipeline.signals,
            &s.pipeline.signals_train,
            &s.pipeline.indications,
            &packs.signals,
            src,
            &project.gw_values.signals_train.signals_train_without_bf,
            &project.gw_values.indicators.indicators_without_bf,
        );
        project.gw.signals_gw = SignalsGateway::new(
            &project.gw_values.signals,
            &project.gw_values.signals_train,
            &project.gw_values.indicators,
            &s.pipeline.signals_train,
            &s.pipeline.signals,
            &s.pipeline.indications,
        );
    }

    pub fn init_utils_state(self: Pin<&mut Self>, _: &Buffer, s: &'a SETTINGS, packs: &Packs) {
        let mut project = self.project();
        unsafe { project.gw_values.as_mut().get_unchecked_mut() }.utils_state =
            UtilsState::new(&s.pipeline.utils_state, &packs.utils_state);
        project.gw.utils_state_gw =
            UtilsStateGateway::new(&project.gw_values.utils_state, &s.pipeline.utils_state);
    }

    pub fn init_order_creators(self: Pin<&mut Self>, _: &Buffer, s: &'a SETTINGS, _: &Packs) {
        let mut project = self.project();
        unsafe { project.gw_values.as_mut().get_unchecked_mut() }.order_creators =
            OrderCreators::new(&s.pipeline.order_creators);
        project.gw.order_creators_gw = OrderCreatorsGateway::new(
            &project.gw_values.order_creators,
            &s.pipeline.order_creators,
        );
    }

    pub fn init_order_filters(self: Pin<&mut Self>, _: &Buffer, s: &'a SETTINGS, packs: &Packs) {
        let mut project = self.project();
        unsafe { project.gw_values.as_mut().get_unchecked_mut() }.order_filters =
            OrderFilters::new(&s.pipeline.order_filters, &packs.order_filters);
        project.gw.order_filters_gw =
            OrderFilterGateway::new(&project.gw_values.order_filters, &s.pipeline.order_filters);
    }

    pub fn init_orders_collectors(
        self: Pin<&mut Self>,
        _: &Buffer,
        s: &'a SETTINGS,
        packs: &Packs,
    ) {
        let mut project = self.project();
        unsafe { project.gw_values.as_mut().get_unchecked_mut() }.orders_collectors =
            <OrdersCollectors as OrdersCollectorsExt>::new(
                &s.pipeline_execute.order_collectors,
                &packs.orders_collectors,
            );
        project.gw.orders_collectors_gw =
            OrdersCollectorsGateway::new(&project.gw_values.orders_collectors);
    }
}

impl<'a, 'b> TradeRuntime<'a, 'b> {
    pub fn step_with(self: &mut Pin<Box<Self>>, buffer: &mut Buffer, stage_end: &str) {
        buffer.transpose_set();
        for (stage, func) in STAGES_STEP.iter().zip([
            TradeRuntime::step_ind,
            TradeRuntime::step_signals_train,
            TradeRuntime::step_signals,
            TradeRuntime::step_utils_state,
            TradeRuntime::step_order_creators,
            TradeRuntime::step_order_filters,
        ]) {
            func(self.as_mut(), buffer);
            if *stage == stage_end {
                buffer.transpose_set();
                return;
            }
        }
        buffer.transpose_set();
    }

    pub fn step_ind(self: Pin<&mut Self>, buffer: &mut Buffer) {
        let project = self.project();
        project.state.indications = project.gw.indicators_gw.indications_series(buffer);
    }

    pub fn step_signals_train(self: Pin<&mut Self>, buffer: &mut Buffer) {
        let project = self.project();
        project.state.signals_train = project
            .gw
            .signals_train_gw
            .signals_series(&project.state.indications, buffer);
    }

    pub fn step_signals(self: Pin<&mut Self>, buffer: &mut Buffer) {
        let project = self.project();
        project.state.signals = project.gw.signals_gw.signals_series(
            &project.state.indications,
            &project.state.signals_train,
            buffer,
        );
    }

    pub fn step_utils_state(self: Pin<&mut Self>, buffer: &mut Buffer) {
        let project = self.project();
        project.state.utils_state = project.gw.utils_state_gw.series(
            &project.state.trade_state,
            buffer,
            &project.state.indications,
            &project.state.signals,
        );
    }

    pub fn step_order_creators(self: Pin<&mut Self>, _: &mut Buffer) {
        let project = self.project();
        project.state.orders = project.gw.order_creators_gw.series(
            *project.symbol,
            &project.state.signals,
            &project.state.indications,
            &project.state.utils_state,
        );
    }

    pub fn step_order_filters(self: Pin<&mut Self>, buffer: &mut Buffer) {
        let project = self.project();
        project
            .state
            .trade_state
            .step(project.gw.order_filters_gw.series(
                &project.state.orders,
                buffer,
                &project.state.indications,
                &project.state.utils_state,
                &project.state.signals,
                &project.state.trade_state,
            ));
    }
}

impl<'a, 'b> TradeRuntime<'a, 'b> {
    pub fn execute_with(
        self: &mut Pin<Box<Self>>,
        buffer: &Buffer,
        stage_end: &str,
    ) -> Result<(), Box<dyn Error>> {
        self.as_mut()
            .project()
            .state
            .trade_state
            .execute(&buffer[buffer.len() - 1], &buffer[buffer.len() - 2])?;
        for (stage, func) in STAGES_EXECUTE
            .iter()
            .zip([TradeRuntime::execute_orders_collectors])
        {
            func(self.as_mut(), buffer)?;
            if *stage == stage_end {
                return Ok(());
            }
        }
        Ok(())
    }

    pub fn execute_orders_collectors(
        self: Pin<&mut Self>,
        _: &Buffer,
    ) -> Result<(), Box<dyn Error>> {
        let project = self.project();
        project
            .gw
            .orders_collectors_gw
            .collect_orders(&project.state.trade_state);
        Ok(())
    }
}

impl<'a, 'b> TradeRuntime<'a, 'b> {
    pub fn clear(self: Pin<&mut Self>) {
        let project = self.project();
        project.state.trade_state.clear();
    }
}

impl<'a, 'b> TradeRuntime<'a, 'b> {
    pub fn update_bf(self: Pin<&mut Self>, s: &'a SETTINGS, buffer: &[Vec<f64>], packs: &Packs) {
        let outer_mut = unsafe { self.get_unchecked_mut() };
        outer_mut
            .gw_values
            .indicators
            .update_bf(buffer, &s.pipeline.indications, &packs.ind);
        outer_mut.gw_values.signals_train.update_bf(
            buffer,
            s,
            &packs.signals_train,
            &outer_mut.gw_values.indicators.indicators_without_bf,
        );
        outer_mut.gw_values.signals.update_bf(
            buffer,
            s,
            &packs.signals,
            &outer_mut.gw_values.signals_train.signals_train_without_bf,
            &outer_mut.gw_values.indicators.indicators_without_bf,
        );
        outer_mut.gw_values.order_filters.update_bf();
    }
}
