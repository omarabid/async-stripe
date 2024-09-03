use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListTreasuryTransactionEntryBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_at: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    financial_account: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_by: Option<ListTreasuryTransactionEntryOrderBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transaction: Option<&'a str>,
}
impl<'a> ListTreasuryTransactionEntryBuilder<'a> {
    fn new(financial_account: &'a str) -> Self {
        Self {
            created: None,
            effective_at: None,
            ending_before: None,
            expand: None,
            financial_account,
            limit: None,
            order_by: None,
            starting_after: None,
            transaction: None,
        }
    }
}
/// The results are in reverse chronological order by `created` or `effective_at`.
/// The default is `created`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTreasuryTransactionEntryOrderBy {
    Created,
    EffectiveAt,
}
impl ListTreasuryTransactionEntryOrderBy {
    pub fn as_str(self) -> &'static str {
        use ListTreasuryTransactionEntryOrderBy::*;
        match self {
            Created => "created",
            EffectiveAt => "effective_at",
        }
    }
}

impl std::str::FromStr for ListTreasuryTransactionEntryOrderBy {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryTransactionEntryOrderBy::*;
        match s {
            "created" => Ok(Created),
            "effective_at" => Ok(EffectiveAt),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ListTreasuryTransactionEntryOrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTreasuryTransactionEntryOrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTreasuryTransactionEntryOrderBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListTreasuryTransactionEntryOrderBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ListTreasuryTransactionEntryOrderBy")
        })
    }
}
/// Retrieves a list of TransactionEntry objects.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTreasuryTransactionEntry<'a> {
    inner: ListTreasuryTransactionEntryBuilder<'a>,
}
impl<'a> ListTreasuryTransactionEntry<'a> {
    /// Construct a new `ListTreasuryTransactionEntry`.
    pub fn new(financial_account: &'a str) -> Self {
        Self { inner: ListTreasuryTransactionEntryBuilder::new(financial_account) }
    }
    /// Only return TransactionEntries that were created during the given date interval.
    pub fn created(mut self, created: stripe_types::RangeQueryTs) -> Self {
        self.inner.created = Some(created);
        self
    }
    pub fn effective_at(mut self, effective_at: stripe_types::RangeQueryTs) -> Self {
        self.inner.effective_at = Some(effective_at);
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// The results are in reverse chronological order by `created` or `effective_at`.
    /// The default is `created`.
    pub fn order_by(mut self, order_by: ListTreasuryTransactionEntryOrderBy) -> Self {
        self.inner.order_by = Some(order_by);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
    /// Only return TransactionEntries associated with this Transaction.
    pub fn transaction(mut self, transaction: &'a str) -> Self {
        self.inner.transaction = Some(transaction);
        self
    }
}
impl ListTreasuryTransactionEntry<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<
        stripe_types::List<stripe_treasury::TreasuryTransactionEntry>,
    > {
        stripe_client_core::ListPaginator::new_list("/treasury/transaction_entries", self.inner)
    }
}

impl StripeRequest for ListTreasuryTransactionEntry<'_> {
    type Output = stripe_types::List<stripe_treasury::TreasuryTransactionEntry>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/treasury/transaction_entries").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveTreasuryTransactionEntryBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryTransactionEntryBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a TransactionEntry object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTreasuryTransactionEntry<'a> {
    inner: RetrieveTreasuryTransactionEntryBuilder<'a>,
    id: &'a stripe_treasury::TreasuryTransactionEntryId,
}
impl<'a> RetrieveTreasuryTransactionEntry<'a> {
    /// Construct a new `RetrieveTreasuryTransactionEntry`.
    pub fn new(id: &'a stripe_treasury::TreasuryTransactionEntryId) -> Self {
        Self { id, inner: RetrieveTreasuryTransactionEntryBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveTreasuryTransactionEntry<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrieveTreasuryTransactionEntry<'_> {
    type Output = stripe_treasury::TreasuryTransactionEntry;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/treasury/transaction_entries/{id}"))
            .query(&self.inner)
    }
}