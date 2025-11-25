use froodi::{DefaultScope::App, Inject, Registry, instance, registry};

use shared::config::Config;

pub(crate) fn create_config_registry(config: Config) -> Registry {
    registry! {
        provide(App, instance(config)),
        provide(App, |Inject(app_config): Inject<Config>| Ok(app_config.external_auth.clone()))
    }
}
