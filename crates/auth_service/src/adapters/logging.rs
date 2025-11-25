use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub(crate) fn init_logging() {
    tracing_subscriber::registry()
        .with(EnvFilter::builder().parse_lossy(
            "debug,froodi=warn,reqwest=warn,hyper=warn,tokio_postgres=warn,tokio_util::codec=warn",
        ))
        .with(
            fmt::layer()
                .json()
                .with_current_span(true)
                .with_span_list(true)
                .with_file(true)
                .with_line_number(true)
                .with_target(true),
        )
        .init();
}
