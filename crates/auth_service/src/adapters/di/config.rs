use froodi::{DefaultScope::App, Registry, instance, registry};

use shared::config::Config;

pub(crate) fn create_config_registry(config: Config) -> Registry {
    registry! {
        provide(App, instance(config)),
    }
}
