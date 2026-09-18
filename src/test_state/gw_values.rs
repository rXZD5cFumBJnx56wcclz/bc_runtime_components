use crate::{gw_values::*, test_state::buffer::BUFFER};

use bc_packs::test_state::PACKS;
use bc_test_kit::prelude::*;
use bc_utils_lg::test_state::settings::pipeline::PIPELINE;

pub static GW_VALUES: LazyLock<fn() -> GWValues<'static>> = LazyLock::new(|| {
    || {
        let mut gw = GWValues::default();
        gw.init_empty_with(&PIPELINE, &PACKS, "");
        gw.init_bf(&BUFFER, &PIPELINE);
        gw
    }
});
