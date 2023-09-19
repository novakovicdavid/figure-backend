use std::process;
use tracing::{info, Subscriber};
use tracing_subscriber::{EnvFilter, fmt, Layer};
use tracing_subscriber::filter::Filtered;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use url::Url;
use interpol::format as iformat;

pub fn init_logging((application_level, library_level): (&str, &str), loki_host: Option<String>, loki_url: Option<String>) -> Result<(), anyhow::Error> {
    let registry = tracing_subscriber::registry();

    let console_output = add_filter_to_layer(fmt::layer(), (application_level, library_level))?;

    let (loki_output, loki_task) = match (&loki_host, loki_url) {
        (Some(host), Some(url)) => {
            let (loki_logging, task) = tracing_loki::builder()
                .label("host", host)?
                .extra_field("pid", iformat!("{process::id()}"))?
                .build_url(Url::parse(&url)?)?;

            (Some(add_filter_to_layer(loki_logging, (application_level, library_level))?), Some(task))
        },
        _ => (None, None)
    };

    registry
        .with(loki_output)
        .with(console_output)
        .init();

    if let Some(task) = loki_task {
        info!("Loki instance registered, sending logs as host \"{}\"", loki_host.unwrap());
        tokio::spawn(task);
    }
    Ok(())
}

fn add_filter_to_layer<S: Subscriber>(layer: impl Layer<S>, (application_level, library_level): (&str, &str)) -> Result<Filtered<impl Layer<S>, EnvFilter, S>, anyhow::Error> {
    Ok(layer
        .with_filter(EnvFilter::default()
        .add_directive(library_level.parse()?)
        .add_directive(format!("figure_backend={}", application_level).parse()?)))
}