pub(crate) mod types {
    pub(crate) mod r_u128;
}

pub use crate::{
    types::{
        r_u128::r_u128,
    },
};

pub(crate) use std::{
    str::FromStr,
};

pub(crate) use pgrx::prelude::*;


::pgrx::pg_module_magic!(name, version);

#[pg_extern]
fn srcpg_ready() -> bool {
    true
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_ready() {
        assert_eq!(true, crate::srcpg_ready());
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
