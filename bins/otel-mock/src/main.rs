use std::time::Duration;

use opentelemetry_otlp::WithExportConfig;
use tracing_subscriber::layer::SubscriberExt;

#[tokio::main]
async fn main() {
    let trace_exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("http://127.0.0.1:4317");

    let otel_tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(trace_exporter)
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .unwrap();

    let layer = tracing_opentelemetry::layer().with_tracer(otel_tracer);

    tracing::subscriber::set_global_default(tracing_subscriber::registry().with(layer)).unwrap();

    loop {
        let span = tracing::trace_span!("my-span");

        println!("mock");
        tracing::info!("mock");

        drop(span);
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
