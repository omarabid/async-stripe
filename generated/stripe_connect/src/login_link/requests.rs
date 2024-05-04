#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateAccountLoginLink<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CreateAccountLoginLink<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> CreateAccountLoginLink<'a> {
    /// Creates a single-use login link for an Express account to access their Stripe dashboard.
    ///
    /// **You may only create login links for [Express accounts](https://stripe.com/docs/connect/express-accounts) connected to your platform**.
    pub fn send(
        &self,
        client: &stripe::Client,
        account: &stripe_shared::AccountId,
    ) -> stripe::Response<stripe_connect::LoginLink> {
        client.send_form(
            &format!("/accounts/{account}/login_links"),
            self,
            http_types::Method::Post,
        )
    }
}