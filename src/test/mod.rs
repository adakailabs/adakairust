use std::env;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn test_initialize() {
    INIT.call_once(|| {
        env::set_var("RUST_LOG", "debug");
        pretty_env_logger::init();
    });

}
