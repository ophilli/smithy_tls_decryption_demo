use std::sync::Arc;

use aws_config::BehaviorVersion;
use hyper_rustls::ConfigBuilderExt;

fn create_ssl_key_log_client() -> aws_smithy_runtime_api::client::http::SharedHttpClient {
    // Create a custom TLS configuration with key logging enabled
    // legacy_hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().
    let mut tls_config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_native_roots()
        .with_no_client_auth();

    // Enable SSLKEYLOGFILE for TLS debugging
    tls_config.key_log = Arc::new(rustls::KeyLogFile::new());

    // Create a hyper-rustls connector with our TLS config
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_tls_config(tls_config)
        .https_or_http()
        .enable_http1()
        .enable_http2()
        .build();

    // Build the client with our wrapped connector
    aws_smithy_runtime::client::http::hyper_014::HyperClientBuilder::new().build(https)
}

#[tokio::main]
async fn main() {
    let sdk_config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(create_ssl_key_log_client())
        .region(aws_sdk_sts::config::Region::new("us-west-2"))
        .load()
        .await;

    let client = aws_sdk_sts::Client::new(&sdk_config);
    let caller_identity = client.get_caller_identity().send().await;

    println!("Caller Identity Result: {caller_identity:#?}");
}
