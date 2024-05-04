use std::future::{self, Future};
use std::pin::Pin;

use http_types::{Request, StatusCode};
use hyper::http;
use hyper::{client::HttpConnector, Body};
use miniserde::json::from_str;
use tokio::time::sleep;

use crate::client::request_strategy::{Outcome, RequestStrategy};
use crate::error::StripeError;

#[cfg(feature = "hyper-rustls-native")]
mod connector {
    use hyper::client::{connect::dns::GaiResolver, HttpConnector};
    pub use hyper_rustls::HttpsConnector;
    use hyper_rustls::HttpsConnectorBuilder;

    pub fn create() -> HttpsConnector<HttpConnector<GaiResolver>> {
        HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build()
    }
}

#[cfg(feature = "hyper-rustls-webpki")]
mod connector {
    use hyper::client::{connect::dns::GaiResolver, HttpConnector};
    pub use hyper_rustls::HttpsConnector;
    use hyper_rustls::HttpsConnectorBuilder;

    pub fn create() -> HttpsConnector<HttpConnector<GaiResolver>> {
        HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build()
    }
}

#[cfg(feature = "hyper-tls")]
mod connector {
    use hyper::client::{connect::dns::GaiResolver, HttpConnector};
    pub use hyper_tls::HttpsConnector;

    pub fn create() -> HttpsConnector<HttpConnector<GaiResolver>> {
        HttpsConnector::new()
    }
}

#[cfg(all(feature = "hyper-tls", feature = "hyper-rustls"))]
compile_error!("You must enable only one TLS implementation");

type HttpClient = hyper::Client<connector::HttpsConnector<HttpConnector>, Body>;

pub type Response<T> = Pin<Box<dyn Future<Output = Result<T, StripeError>> + Send>>;

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn err<T: Send + 'static>(err: StripeError) -> Response<T> {
    Box::pin(future::ready(Err(err)))
}

#[derive(Clone)]
pub struct TokioClient {
    client: HttpClient,
}

impl TokioClient {
    pub fn new() -> Self {
        Self {
            client: hyper::Client::builder().pool_max_idle_per_host(0).build(connector::create()),
        }
    }

    pub fn execute<T: miniserde::Deserialize + Send + 'static>(
        &self,
        request: Request,
        strategy: &RequestStrategy,
    ) -> Response<T> {
        // need to clone here since client could be used across threads.
        // N.B. Client is send sync; cloned clients share the same pool.
        let client = self.client.clone();
        let strategy = strategy.clone();

        Box::pin(async move {
            let bytes = send_inner(&client, request, &strategy).await?;
            let str = std::str::from_utf8(bytes.as_ref())
                .map_err(|_| StripeError::JSONDeserialize("Response was not valid UTF-8".into()))?;
            from_str(str).map_err(|_| {
                StripeError::JSONDeserialize("error deserializing request data".into())
            })
        })
    }
}

impl Default for TokioClient {
    fn default() -> Self {
        Self::new()
    }
}

async fn send_inner(
    client: &HttpClient,
    mut request: Request,
    strategy: &RequestStrategy,
) -> Result<hyper::body::Bytes, StripeError> {
    let mut tries = 0;
    let mut last_status: Option<StatusCode> = None;
    let mut last_retry_header: Option<bool> = None;

    // if we have no last error, then the strategy is invalid
    let mut last_error = StripeError::ClientError("Invalid strategy".to_string());

    if let Some(key) = strategy.get_key() {
        request.insert_header("Idempotency-Key", key);
    }

    let body = request.body_bytes().await?;

    loop {
        return match strategy.test(last_status, last_retry_header, tries) {
            Outcome::Stop => Err(last_error),
            Outcome::Continue(duration) => {
                if let Some(duration) = duration {
                    sleep(duration).await;
                }

                // note: http::Request provides no easy way to clone, so we perform
                //       the conversion from the clonable http_types::Request each time
                //       obviously cloning before the first request is not ideal
                let mut request = request.clone();
                request.set_body(body.clone());

                let response = match client.request(convert_request(request).await).await {
                    Ok(response) => response,
                    Err(err) => {
                        last_error = StripeError::from(err);
                        tries += 1;
                        continue;
                    }
                };

                let status = response.status();
                let retry = response
                    .headers()
                    .get("Stripe-Should-Retry")
                    .and_then(|s| s.to_str().ok())
                    .and_then(|s| s.parse().ok());

                let bytes = hyper::body::to_bytes(response.into_body()).await?;

                if !status.is_success() {
                    tries += 1;
                    let str = std::str::from_utf8(bytes.as_ref()).map_err(|_| {
                        StripeError::JSONDeserialize("Response was not valid UTF-8".into())
                    })?;
                    last_error = from_str(str)
                        .map(|e: stripe_shared::Error| {
                            StripeError::Stripe(*e.error, status.as_u16())
                        })
                        .unwrap_or_else(|_| {
                            StripeError::JSONDeserialize(
                                "Could not deserialize Stripe error".into(),
                            )
                        });
                    last_status = Some(
                        // NOTE: StatusCode::from can panic here, so fall back to InternalServerError
                        //       see https://github.com/http-rs/http-types/blob/ac5d645ce5294554b86ebd49233d3ec01665d1d7/src/hyperium_http.rs#L20-L24
                        StatusCode::try_from(u16::from(status))
                            .unwrap_or(StatusCode::InternalServerError),
                    );
                    last_retry_header = retry;
                    continue;
                }

                Ok(bytes)
            }
        };
    }
}

/// convert an http_types::Request with a http_types::Body into a http::Request<hyper::Body>
///
/// note: this is necesarry because `http` deliberately does not support a `Body` type
///       so hyper has a `Body` for which http_types cannot provide automatic conversion.
async fn convert_request(mut request: http_types::Request) -> http::Request<hyper::Body> {
    let body = request.body_bytes().await.expect("We know the data is a valid bytes object.");
    let request: http::Request<_> = request.into();
    http::Request::from_parts(request.into_parts().0, hyper::Body::from(body))
}

#[cfg(test)]
mod tests {
    use http_types::{Method, Request, Url};
    use httpmock::prelude::*;
    use hyper::{body::to_bytes, Body, Request as HyperRequest};
    use serde_json::json;
    use stripe_shared::api_errors::{ApiErrorsCode, ApiErrorsType};

    use super::convert_request;
    use super::TokioClient;
    use crate::client::request_strategy::RequestStrategy;
    use crate::StripeError;

    const TEST_URL: &str = "https://api.stripe.com/v1/";

    #[tokio::test]
    async fn basic_conversion() {
        req_equal(
            convert_request(Request::new(Method::Get, TEST_URL)).await,
            HyperRequest::builder()
                .method("GET")
                .uri("http://test.com")
                .body(Body::empty())
                .unwrap(),
        )
        .await;
    }

    #[tokio::test]
    async fn bytes_body_conversion() {
        let body = "test".as_bytes();

        let mut req = Request::new(Method::Post, TEST_URL);
        req.set_body(body);

        req_equal(
            convert_request(req).await,
            HyperRequest::builder().method("POST").uri(TEST_URL).body(Body::from(body)).unwrap(),
        )
        .await;
    }

    #[tokio::test]
    async fn string_body_conversion() {
        let body = "test";

        let mut req = Request::new(Method::Post, TEST_URL);
        req.set_body(body);

        req_equal(
            convert_request(req).await,
            HyperRequest::builder().method("POST").uri(TEST_URL).body(Body::from(body)).unwrap(),
        )
        .await;
    }

    async fn req_equal(a: HyperRequest<Body>, b: HyperRequest<Body>) {
        let (a_parts, a_body) = a.into_parts();
        let (b_parts, b_body) = b.into_parts();

        assert_eq!(a_parts.method, b_parts.method);
        assert_eq!(to_bytes(a_body).await.unwrap().len(), to_bytes(b_body).await.unwrap().len());
    }

    #[tokio::test]
    async fn retry() {
        let client = TokioClient::new();

        // Start a lightweight mock server.
        let server = MockServer::start_async().await;

        // Create a mock on the server.
        let hello_mock = server.mock(|when, then| {
            when.method(GET).path("/server-errors");
            then.status(500);
        });

        let req = Request::get(Url::parse(&server.url("/server-errors")).unwrap());
        let res = client.execute::<()>(req, &RequestStrategy::Retry(5)).await;

        hello_mock.assert_hits_async(5).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn user_error() {
        let client = TokioClient::new();

        // Start a lightweight mock server.
        let server = MockServer::start_async().await;

        let mock = server.mock(|when, then| {
            when.method(GET).path("/v1/missing");
            then.status(404).body("{
                \"error\": {
                  \"message\": \"Unrecognized request URL (GET: /v1/missing). Please see https://stripe.com/docs or we can help at https://support.stripe.com/.\",
                  \"type\": \"invalid_request_error\"
                }
              }
              ");
        });

        let req = Request::get(Url::parse(&server.url("/v1/missing")).unwrap());
        let res = client.execute::<()>(req, &RequestStrategy::Retry(3)).await;

        mock.assert_hits_async(1).await;

        match res {
            Err(StripeError::Stripe(x, _)) => println!("{:?}", x),
            _ => panic!("Expected stripe error {:?}", res),
        }
    }

    #[tokio::test]
    async fn nice_serde_error() {
        use serde::Deserialize;

        #[derive(Debug, Deserialize, miniserde::Deserialize)]
        struct DataType {
            // Allowing dead code since used for deserialization
            #[allow(dead_code)]
            id: String,
            #[allow(dead_code)]
            name: String,
        }

        let client = TokioClient::new();

        // Start a lightweight mock server.
        let server = MockServer::start_async().await;

        let mock = server.mock(|when, then| {
            when.method(GET).path("/v1/odd_data");
            then.status(200).body(
                "{
                \"id\": \"test\",
                \"name\": 10
              }
              ",
            );
        });

        let req = Request::get(Url::parse(&server.url("/v1/odd_data")).unwrap());
        let res = client.execute::<DataType>(req, &RequestStrategy::Retry(3)).await;

        mock.assert_hits_async(1).await;

        match res {
            Err(StripeError::JSONDeserialize(_)) => {}
            _ => panic!("Expected stripe error {:?}", res),
        }
    }

    #[tokio::test]
    async fn retry_header() {
        let client = TokioClient::new();

        // Start a lightweight mock server.
        let server = MockServer::start_async().await;

        // Create a mock on the server.
        let hello_mock = server.mock(|when, then| {
            when.method(GET).path("/server-errors");
            then.status(500).header("Stripe-Should-Retry", "false");
        });

        let req = Request::get(Url::parse(&server.url("/server-errors")).unwrap());
        let res = client.execute::<()>(req, &RequestStrategy::Retry(5)).await;

        hello_mock.assert_hits_async(1).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn retry_body() {
        let client = TokioClient::new();

        // Start a lightweight mock server.
        let server = MockServer::start_async().await;

        // Create a mock on the server.
        let hello_mock = server.mock(|when, then| {
            when.method(POST).path("/server-errors").body("body");
            then.status(500);
        });

        let mut req = Request::post(Url::parse(&server.url("/server-errors")).unwrap());
        req.set_body("body");
        let res = client.execute::<()>(req, &RequestStrategy::Retry(5)).await;

        hello_mock.assert_hits_async(5).await;
        assert!(res.is_err());
    }

    // https://github.com/arlyon/async-stripe/issues/384
    #[tokio::test]
    async fn user_error_transfers() {
        let client = TokioClient::new();

        // Start a lightweight mock server.
        let server = MockServer::start_async().await;
        let message = "Your destination account needs to have at least one of the following capabilities enabled: transfers, crypto_transfers, legacy_payments";
        let log_url = "https://dashboard.stripe.com/logs/req_nIhlutaV4amLEs?t=1685040634";
        let mock = server.mock(|when, then| {
            when.method(GET).path("/v1/transfers");
            then.status(400).json_body(json!({
              "error": {
                "code": "insufficient_capabilities_for_transfer",
                "message": message,
                "request_log_url": log_url,
                "type": "invalid_request_error"
              }
            }));
        });

        let req = Request::get(Url::parse(&server.url("/v1/transfers")).unwrap());
        let res = client.execute::<()>(req, &RequestStrategy::Once).await.unwrap_err();
        mock.assert_hits_async(1).await;

        match res {
            StripeError::Stripe(err, status_code) => {
                assert_eq!(status_code, 400);
                assert_eq!(err.type_, ApiErrorsType::InvalidRequestError);
                assert_eq!(err.message.as_deref(), Some(message));
                assert_eq!(err.request_log_url.as_deref(), Some(log_url));
                // NB: `Unknown` here because the error code reported in the issue is not
                // present in the OpenAPI spec. Reporting unknown instead of an error seems
                // better regardless so that stripe adding new variants is not a breaking change
                assert_eq!(err.code, Some(ApiErrorsCode::Unknown));
            }
            _ => panic!("Expected stripe error, got {:?}", res),
        }
    }
}