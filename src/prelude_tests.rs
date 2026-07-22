#![allow(unused_imports)]

#[cfg(test)]
pub mod prelude {
    pub use std::sync::LazyLock;

    pub use bc_test_kit::prelude::*;
    pub use pretty_assertions::assert_eq as assert_eq_pr;
}
