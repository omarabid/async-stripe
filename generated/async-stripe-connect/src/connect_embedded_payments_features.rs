#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectEmbeddedPaymentsFeatures {
    /// Whether to allow capturing and cancelling payment intents. This is `true` by default.
    pub capture_payments: bool,
    /// Whether to allow connected accounts to manage destination charges that are created on behalf of them.
    /// This is `false` by default.
    pub destination_on_behalf_of_charge_management: bool,
    /// Whether to allow responding to disputes, including submitting evidence and accepting disputes.
    /// This is `true` by default.
    pub dispute_management: bool,
    /// Whether to allow sending refunds. This is `true` by default.
    pub refund_management: bool,
}
#[doc(hidden)]
pub struct ConnectEmbeddedPaymentsFeaturesBuilder {
    capture_payments: Option<bool>,
    destination_on_behalf_of_charge_management: Option<bool>,
    dispute_management: Option<bool>,
    refund_management: Option<bool>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ConnectEmbeddedPaymentsFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConnectEmbeddedPaymentsFeatures>,
        builder: ConnectEmbeddedPaymentsFeaturesBuilder,
    }

    impl Visitor for Place<ConnectEmbeddedPaymentsFeatures> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConnectEmbeddedPaymentsFeaturesBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ConnectEmbeddedPaymentsFeaturesBuilder {
        type Out = ConnectEmbeddedPaymentsFeatures;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "capture_payments" => Deserialize::begin(&mut self.capture_payments),
                "destination_on_behalf_of_charge_management" => {
                    Deserialize::begin(&mut self.destination_on_behalf_of_charge_management)
                }
                "dispute_management" => Deserialize::begin(&mut self.dispute_management),
                "refund_management" => Deserialize::begin(&mut self.refund_management),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                capture_payments: Deserialize::default(),
                destination_on_behalf_of_charge_management: Deserialize::default(),
                dispute_management: Deserialize::default(),
                refund_management: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(capture_payments),
                Some(destination_on_behalf_of_charge_management),
                Some(dispute_management),
                Some(refund_management),
            ) = (
                self.capture_payments,
                self.destination_on_behalf_of_charge_management,
                self.dispute_management,
                self.refund_management,
            )
            else {
                return None;
            };
            Some(Self::Out {
                capture_payments,
                destination_on_behalf_of_charge_management,
                dispute_management,
                refund_management,
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

    impl ObjectDeser for ConnectEmbeddedPaymentsFeatures {
        type Builder = ConnectEmbeddedPaymentsFeaturesBuilder;
    }

    impl FromValueOpt for ConnectEmbeddedPaymentsFeatures {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ConnectEmbeddedPaymentsFeaturesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "capture_payments" => b.capture_payments = FromValueOpt::from_value(v),
                    "destination_on_behalf_of_charge_management" => {
                        b.destination_on_behalf_of_charge_management = FromValueOpt::from_value(v)
                    }
                    "dispute_management" => b.dispute_management = FromValueOpt::from_value(v),
                    "refund_management" => b.refund_management = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};