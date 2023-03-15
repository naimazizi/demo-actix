use std::sync::Arc;

use awc::{http::header, Client, Connector};
use rustls::{ClientConfig, OwnedTrustAnchor, RootCertStore};

/// Create simple rustls client config from root certificates.
pub fn rustls_config() -> ClientConfig {
    let mut root_store = RootCertStore::empty();
    root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));

    rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth()
}

pub fn init(tls_client_config: Arc<ClientConfig>) -> Client {
    let client = Client::builder()
        // Wikipedia requires a User-Agent header to make requests
        .add_default_header((header::USER_AGENT, "actix_demo/1.0"))
        // a "connector" wraps the stream into an encrypted connection
        .connector(Connector::new().rustls(tls_client_config))
        .finish();
    client
}
