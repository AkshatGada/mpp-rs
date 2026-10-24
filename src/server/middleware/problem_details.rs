//! Response-body construction for Tower payment responses.

use super::*;

// The default preserves empty bodies without constructing problem metadata.
// Opt-in constructors are shared across clones and need not implement Clone.
pub(super) trait ResponseBody<B>: Clone + Send + Sync + 'static {
    fn render(
        &self,
        response: Response<B>,
        problem: impl FnOnce() -> PaymentErrorDetails,
    ) -> Response<B>;
}

impl<B> ResponseBody<B> for () {
    fn render(
        &self,
        response: Response<B>,
        _problem: impl FnOnce() -> PaymentErrorDetails,
    ) -> Response<B> {
        response
    }
}

impl<B: Default, F> ResponseBody<B> for Arc<F>
where
    F: Fn(Bytes) -> B + Send + Sync + 'static,
{
    fn render(
        &self,
        mut response: Response<B>,
        problem: impl FnOnce() -> PaymentErrorDetails,
    ) -> Response<B> {
        let mut problem = problem();
        // Match Axum's first-offer association without changing custom headers.
        if let Some(challenge) = response
            .headers()
            .get_all(WWW_AUTHENTICATE_HEADER)
            .iter()
            .filter_map(|value| value.to_str().ok())
            .flat_map(|value| crate::protocol::core::parse_www_authenticate_all([value]))
            .find_map(Result::ok)
        {
            problem.challenge_id = Some(challenge.id);
        }
        match serde_json::to_vec(&problem) {
            Ok(json) => {
                *response.body_mut() = self(Bytes::from(json));
                response.headers_mut().insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("application/problem+json"),
                );
            }
            Err(_) => {
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                *response.body_mut() = B::default();
                response.headers_mut().remove(WWW_AUTHENTICATE_HEADER);
                response.headers_mut().remove(header::CONTENT_TYPE);
            }
        }
        response
    }
}

#[cfg(test)]
mod tests;
