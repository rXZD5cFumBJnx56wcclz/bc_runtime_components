use crate::buffer::*;

use bc_test_kit::prelude::*;

pub static BUFFER: LazyLock<Buffer> = LazyLock::new(|| SRC.to_buff());
