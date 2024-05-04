#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeleteAccountCard {}
impl DeleteAccountCard {
    pub fn new() -> Self {
        Self::default()
    }
}
impl DeleteAccountCard {
    /// Delete a specified external account for a given account.
    pub fn send(
        &self,
        client: &stripe::Client,
        account: &stripe_shared::AccountId,
        id: &str,
    ) -> stripe::Response<stripe_shared::DeletedExternalAccount> {
        client.send_form(
            &format!("/accounts/{account}/external_accounts/{id}"),
            self,
            http_types::Method::Delete,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeleteCustomerCard<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> DeleteCustomerCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> DeleteCustomerCard<'a> {
    /// Delete a specified source for a given customer.
    pub fn send(
        &self,
        client: &stripe::Client,
        customer: &stripe_shared::CustomerId,
        id: &str,
    ) -> stripe::Response<DeleteCustomerCardReturned> {
        client.send_form(
            &format!("/customers/{customer}/sources/{id}"),
            self,
            http_types::Method::Delete,
        )
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(untagged))]
pub enum DeleteCustomerCardReturned {
    PaymentSource(stripe_shared::PaymentSource),
    DeletedPaymentSource(stripe_shared::DeletedPaymentSource),
}

#[derive(Default)]
pub struct DeleteCustomerCardReturnedBuilder {
    inner: stripe_types::miniserde_helpers::MaybeDeletedBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::MapBuilder;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<DeleteCustomerCardReturned>,
        builder: DeleteCustomerCardReturnedBuilder,
    }

    impl Deserialize for DeleteCustomerCardReturned {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<DeleteCustomerCardReturned> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for DeleteCustomerCardReturnedBuilder {
        type Out = DeleteCustomerCardReturned;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (deleted, o) = self.inner.finish_inner()?;
            Some(if deleted {
                DeleteCustomerCardReturned::DeletedPaymentSource(FromValueOpt::from_value(
                    Value::Object(o),
                )?)
            } else {
                DeleteCustomerCardReturned::PaymentSource(FromValueOpt::from_value(Value::Object(
                    o,
                ))?)
            })
        }
    }

    impl stripe_types::ObjectDeser for DeleteCustomerCardReturned {
        type Builder = DeleteCustomerCardReturnedBuilder;
    }
};

#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCard<'a> {
    /// The name of the person or business that owns the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<&'a str>,
    /// The type of entity that holds the account. This can be either `individual` or `company`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<UpdateAccountCardAccountHolderType>,
    /// The bank account type.
    /// This can only be `checking` or `savings` in most countries.
    /// In Japan, this can only be `futsu` or `toza`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<UpdateAccountCardAccountType>,
    /// City/District/Suburb/Town/Village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_city: Option<&'a str>,
    /// Billing address country, if provided when creating card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_country: Option<&'a str>,
    /// Address line 1 (Street address/PO Box/Company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<&'a str>,
    /// Address line 2 (Apartment/Suite/Unit/Building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<&'a str>,
    /// State/County/Province/Region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_state: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip: Option<&'a str>,
    /// When set to true, this becomes the default external account for its currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for_currency: Option<bool>,
    /// Documents that may be submitted to satisfy various informational requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<UpdateAccountCardDocuments<'a>>,
    /// Two digit number representing the card’s expiration month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<&'a str>,
    /// Four digit number representing the card’s expiration year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
}
impl<'a> UpdateAccountCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The type of entity that holds the account. This can be either `individual` or `company`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateAccountCardAccountHolderType {
    Company,
    Individual,
}
impl UpdateAccountCardAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use UpdateAccountCardAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for UpdateAccountCardAccountHolderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateAccountCardAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for UpdateAccountCardAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateAccountCardAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateAccountCardAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateAccountCardAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateAccountCardAccountHolderType")
        })
    }
}
/// The bank account type.
/// This can only be `checking` or `savings` in most countries.
/// In Japan, this can only be `futsu` or `toza`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateAccountCardAccountType {
    Checking,
    Futsu,
    Savings,
    Toza,
}
impl UpdateAccountCardAccountType {
    pub fn as_str(self) -> &'static str {
        use UpdateAccountCardAccountType::*;
        match self {
            Checking => "checking",
            Futsu => "futsu",
            Savings => "savings",
            Toza => "toza",
        }
    }
}

impl std::str::FromStr for UpdateAccountCardAccountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateAccountCardAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "futsu" => Ok(Futsu),
            "savings" => Ok(Savings),
            "toza" => Ok(Toza),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for UpdateAccountCardAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateAccountCardAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateAccountCardAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateAccountCardAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for UpdateAccountCardAccountType"))
    }
}
/// Documents that may be submitted to satisfy various informational requests.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCardDocuments<'a> {
    /// One or more documents that support the [Bank account ownership verification](https://support.stripe.com/questions/bank-account-ownership-verification) requirement.
    /// Must be a document associated with the bank account that displays the last 4 digits of the account number, either a statement or a voided check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_ownership_verification:
        Option<UpdateAccountCardDocumentsBankAccountOwnershipVerification<'a>>,
}
impl<'a> UpdateAccountCardDocuments<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// One or more documents that support the [Bank account ownership verification](https://support.stripe.com/questions/bank-account-ownership-verification) requirement.
/// Must be a document associated with the bank account that displays the last 4 digits of the account number, either a statement or a voided check.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCardDocumentsBankAccountOwnershipVerification<'a> {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<&'a [&'a str]>,
}
impl<'a> UpdateAccountCardDocumentsBankAccountOwnershipVerification<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdateAccountCard<'a> {
    /// Updates the metadata, account holder name, account holder type of a bank account belonging to a [Custom account](https://stripe.com/docs/connect/custom-accounts), and optionally sets it as the default for its currency.
    /// Other bank account details are not editable by design.
    ///
    /// You can re-enable a disabled bank account by performing an update call without providing any arguments or changes.
    pub fn send(
        &self,
        client: &stripe::Client,
        account: &stripe_shared::AccountId,
        id: &str,
    ) -> stripe::Response<stripe_shared::ExternalAccount> {
        client.send_form(
            &format!("/accounts/{account}/external_accounts/{id}"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCustomerCard<'a> {
    /// The name of the person or business that owns the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<&'a str>,
    /// The type of entity that holds the account. This can be either `individual` or `company`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<UpdateCustomerCardAccountHolderType>,
    /// City/District/Suburb/Town/Village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_city: Option<&'a str>,
    /// Billing address country, if provided when creating card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_country: Option<&'a str>,
    /// Address line 1 (Street address/PO Box/Company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<&'a str>,
    /// Address line 2 (Apartment/Suite/Unit/Building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<&'a str>,
    /// State/County/Province/Region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_state: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip: Option<&'a str>,
    /// Two digit number representing the card’s expiration month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<&'a str>,
    /// Four digit number representing the card’s expiration year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<UpdateCustomerCardOwner<'a>>,
}
impl<'a> UpdateCustomerCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The type of entity that holds the account. This can be either `individual` or `company`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateCustomerCardAccountHolderType {
    Company,
    Individual,
}
impl UpdateCustomerCardAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use UpdateCustomerCardAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for UpdateCustomerCardAccountHolderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateCustomerCardAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for UpdateCustomerCardAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateCustomerCardAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateCustomerCardAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateCustomerCardAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateCustomerCardAccountHolderType")
        })
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCustomerCardOwner<'a> {
    /// Owner's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdateCustomerCardOwnerAddress<'a>>,
    /// Owner's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// Owner's full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// Owner's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> UpdateCustomerCardOwner<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Owner's address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCustomerCardOwnerAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> UpdateCustomerCardOwnerAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdateCustomerCard<'a> {
    /// Update a specified source for a given customer.
    pub fn send(
        &self,
        client: &stripe::Client,
        customer: &stripe_shared::CustomerId,
        id: &str,
    ) -> stripe::Response<UpdateCustomerCardReturned> {
        client.send_form(
            &format!("/customers/{customer}/sources/{id}"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(tag = "object"))]
pub enum UpdateCustomerCardReturned {
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "card"))]
    Card(stripe_shared::Card),
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(rename = "bank_account")
    )]
    BankAccount(stripe_shared::BankAccount),
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "source"))]
    Source(stripe_shared::Source),
}

#[derive(Default)]
pub struct UpdateCustomerCardReturnedBuilder {
    inner: stripe_types::miniserde_helpers::ObjectBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::MapBuilder;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<UpdateCustomerCardReturned>,
        builder: UpdateCustomerCardReturnedBuilder,
    }

    impl Deserialize for UpdateCustomerCardReturned {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<UpdateCustomerCardReturned> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for UpdateCustomerCardReturnedBuilder {
        type Out = UpdateCustomerCardReturned;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (k, o) = self.inner.finish_inner()?;
            UpdateCustomerCardReturned::construct(&k, o)
        }
    }

    impl stripe_types::ObjectDeser for UpdateCustomerCardReturned {
        type Builder = UpdateCustomerCardReturnedBuilder;
    }
    impl UpdateCustomerCardReturned {
        fn construct(key: &str, o: miniserde::json::Object) -> Option<Self> {
            Some(match key {
                "card" => Self::Card(FromValueOpt::from_value(Value::Object(o))?),
                "bank_account" => Self::BankAccount(FromValueOpt::from_value(Value::Object(o))?),
                "source" => Self::Source(FromValueOpt::from_value(Value::Object(o))?),

                _ => return None,
            })
        }
    }

    impl FromValueOpt for UpdateCustomerCardReturned {
        fn from_value(v: Value) -> Option<Self> {
            let (typ, obj) = stripe_types::miniserde_helpers::extract_object_discr(v)?;
            Self::construct(&typ, obj)
        }
    }
};