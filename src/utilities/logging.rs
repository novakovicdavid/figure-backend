use std::{env, process};
use tracing::Subscriber;
use tracing_subscriber::{EnvFilter, fmt, Layer};
use tracing_subscriber::filter::Filtered;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use url::Url;
use interpol::format as iformat;

pub fn init_logging() -> Result<(), anyhow::Error> {
    // Initialize logging
    let registry = tracing_subscriber::registry();

    let console_output = add_filter_to_layer(fmt::layer())?;

    let (loki_layer, loki_task) = if let Ok(loki_url) = env::var("LOKI_URL") {
        let (loki_logging, task) = tracing_loki::builder()
            .label("host", "test")?
            .extra_field("pid", iformat!("{process::id()}"))?
            .build_url(Url::parse(&loki_url)?)?;

        (Some(add_filter_to_layer(loki_logging)?), Some(task))
    } else {
        (None, None)
    };

    registry
        .with(loki_layer)
        .with(console_output)
        .init();

    if let Some(task) = loki_task {
        tokio::spawn(task);
    }
    Ok(())
}

fn add_filter_to_layer<S: Subscriber>(layer: impl Layer<S>) -> Result<Filtered<impl Layer<S>, EnvFilter, S>, anyhow::Error> {
    Ok(layer.with_filter(EnvFilter::default()
        .add_directive("WARN".parse()?)
        .add_directive("figure_backend=INFO".parse()?)))
}