mod cost;
pub use cost::*;

use ic_cdk::api::management_canister::http_request::{
    http_request, http_request_with_closure, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
    HttpResponse, TransformContext,
};

/// Used to build a request to the Management Canister's `http_request` method.
pub struct HttpRequest(pub CanisterHttpRequestArgument);

impl HttpRequest {
    /// Creates a new request to be built up by having
    pub fn new(url: String) -> Self {
        Self(CanisterHttpRequestArgument {
            url,
            headers: vec![],
            method: HttpMethod::GET,
            max_response_bytes: None,
            transform: None,
            body: None,
        })
    }

    /// A simple wrapper to assign the URL with the `GET` method.
    pub fn get(self) -> Self {
        self.method(HttpMethod::GET)
    }

    /// A simple wrapper to assign the URL with the `POST` method.
    /// The body is set to the `json_string` argument.
    /// The `max_response_bytes` is set to the `max_response_bytes` argument.
    /// The `max_response_bytes` argument is optional.
    /// The `Content-Type` header is set to `application/json`.
    pub fn post(self, json_string: &str, max_response_bytes: Option<u64>) -> Self {
        self.method(HttpMethod::POST)
            .add_headers(vec![(
                "Content-Type".to_string(),
                "application/json".to_string(),
            )])
            .max_response_bytes(max_response_bytes)
            .body(json_string)
    }

    /// Updates the URL.
    pub fn url(mut self, url: &str) -> Self {
        self.0.url = String::from(url);
        self
    }

    /// Updates the HTTP method.
    pub fn method(mut self, http_method: HttpMethod) -> Self {
        self.0.method = http_method;
        self
    }

    /// Updates the body.
    pub fn body(mut self, body: &str) -> Self {
        self.0.body = Some(body.as_bytes().to_vec());
        self
    }

    /// Adds HTTP headers for the request
    pub fn add_headers(mut self, headers: Vec<(String, String)>) -> Self {
        self.0
            .headers
            .extend(headers.iter().map(|(name, value)| HttpHeader {
                name: name.to_string(),
                value: value.to_string(),
            }));
        self
    }

    /// Updates the transform context of the request.
    pub fn transform_context(mut self, method: &str, context: Option<Vec<u8>>) -> Self {
        let context = TransformContext::from_name(method.to_string(), context.unwrap_or_default());

        self.0.transform = Some(context);
        self
    }

    /// Updates the max_response_bytes of the request.
    pub fn max_response_bytes(mut self, max_response_bytes: Option<u64>) -> Self {
        self.0.max_response_bytes = max_response_bytes;
        self
    }

    /// Calculate the cycle cost for this HTTP request
    pub fn calculate_cycle_cost(&self) -> u128 {
        HttpsOutcallCost::total(&self.0)
    }

    /// Wraps around `http_request` to issue a request to the `http_request` endpoint.
    pub async fn send(self) -> Result<HttpResponse, String> {
        let cycle_cost = self.calculate_cycle_cost();

        // You can log or use the cycle_cost here for further actions
        http_request(self.0, cycle_cost)
            .await
            .map(|(response,)| response)
            .map_err(|(_rejection_code, message)| message)
    }

    /// Wraps around `http_request_with_closure` to issue a request to the `http_request` endpoint with a transform closure.
    pub async fn send_with_closure(
        self,
        transform_func: impl FnOnce(HttpResponse) -> HttpResponse + 'static,
    ) -> Result<HttpResponse, String> {
        let cycle_cost = self.calculate_cycle_cost();
        // You can log or use the cycle_cost here for further actions

        http_request_with_closure(self.0, cycle_cost, transform_func)
            .await
            .map(|(response,)| response)
            .map_err(|(_rejection_code, message)| message)
    }
}