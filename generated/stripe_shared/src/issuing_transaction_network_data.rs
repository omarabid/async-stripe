#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingTransactionNetworkData {
    /// A code created by Stripe which is shared with the merchant to validate the authorization.
    /// This field will be populated if the authorization message was approved.
    /// The code typically starts with the letter "S", followed by a six-digit number.
    /// For example, "S498162".
    /// Please note that the code is not guaranteed to be unique across authorizations.
    pub authorization_code: Option<String>,
    /// The date the transaction was processed by the card network.
    /// This can be different from the date the seller recorded the transaction depending on when the acquirer submits the transaction to the network.
    pub processing_date: Option<String>,
    /// Unique identifier for the authorization assigned by the card network used to match subsequent messages, disputes, and transactions.
    pub transaction_id: Option<String>,
}
#[doc(hidden)]
pub struct IssuingTransactionNetworkDataBuilder {
    authorization_code: Option<Option<String>>,
    processing_date: Option<Option<String>>,
    transaction_id: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingTransactionNetworkData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionNetworkData>,
        builder: IssuingTransactionNetworkDataBuilder,
    }

    impl Visitor for Place<IssuingTransactionNetworkData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingTransactionNetworkDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingTransactionNetworkDataBuilder {
        type Out = IssuingTransactionNetworkData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "authorization_code" => Deserialize::begin(&mut self.authorization_code),
                "processing_date" => Deserialize::begin(&mut self.processing_date),
                "transaction_id" => Deserialize::begin(&mut self.transaction_id),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                authorization_code: Deserialize::default(),
                processing_date: Deserialize::default(),
                transaction_id: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                authorization_code: self.authorization_code.take()?,
                processing_date: self.processing_date.take()?,
                transaction_id: self.transaction_id.take()?,
            })
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

    impl ObjectDeser for IssuingTransactionNetworkData {
        type Builder = IssuingTransactionNetworkDataBuilder;
    }

    impl FromValueOpt for IssuingTransactionNetworkData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingTransactionNetworkDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "authorization_code" => {
                        b.authorization_code = Some(FromValueOpt::from_value(v)?)
                    }
                    "processing_date" => b.processing_date = Some(FromValueOpt::from_value(v)?),
                    "transaction_id" => b.transaction_id = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};