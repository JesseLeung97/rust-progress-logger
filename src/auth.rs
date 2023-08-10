use google_sheets4;
use crate::config;

pub async fn auth(
    config: &config::Config,
    client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
) -> google_sheets4::oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
    let secret: google_sheets4::oauth2::ServiceAccountKey = google_sheets4::oauth2::read_service_account_key(&config.priv_key)
        .await
        .expect("Secret not found");

    return google_sheets4::oauth2::ServiceAccountAuthenticator::with_client(secret, client.clone())
        .build()
        .await
        .expect("Failed to create an authenticator");
}
