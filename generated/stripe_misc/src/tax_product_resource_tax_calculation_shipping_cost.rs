#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceTaxCalculationShippingCost {
    /// The shipping amount in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes were calculated on top of this amount.
    pub amount: i64,
    /// The amount of tax calculated for shipping, in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_tax: i64,
    /// The ID of an existing [ShippingRate](https://stripe.com/docs/api/shipping_rates/object).
    pub shipping_rate: Option<String>,
    /// Specifies whether the `amount` includes taxes.
    /// If `tax_behavior=inclusive`, then the amount includes taxes.
    pub tax_behavior: TaxProductResourceTaxCalculationShippingCostTaxBehavior,
    /// Detailed account of taxes relevant to shipping cost.
    pub tax_breakdown: Option<Vec<stripe_misc::TaxProductResourceLineItemTaxBreakdown>>,
    /// The [tax code](https://stripe.com/docs/tax/tax-categories) ID used for shipping.
    pub tax_code: String,
}
#[doc(hidden)]
pub struct TaxProductResourceTaxCalculationShippingCostBuilder {
    amount: Option<i64>,
    amount_tax: Option<i64>,
    shipping_rate: Option<Option<String>>,
    tax_behavior: Option<TaxProductResourceTaxCalculationShippingCostTaxBehavior>,
    tax_breakdown: Option<Option<Vec<stripe_misc::TaxProductResourceLineItemTaxBreakdown>>>,
    tax_code: Option<String>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductResourceTaxCalculationShippingCost {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxCalculationShippingCost>,
        builder: TaxProductResourceTaxCalculationShippingCostBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxCalculationShippingCost> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductResourceTaxCalculationShippingCostBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxProductResourceTaxCalculationShippingCostBuilder {
        type Out = TaxProductResourceTaxCalculationShippingCost;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "amount_tax" => Deserialize::begin(&mut self.amount_tax),
                "shipping_rate" => Deserialize::begin(&mut self.shipping_rate),
                "tax_behavior" => Deserialize::begin(&mut self.tax_behavior),
                "tax_breakdown" => Deserialize::begin(&mut self.tax_breakdown),
                "tax_code" => Deserialize::begin(&mut self.tax_code),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                amount_tax: Deserialize::default(),
                shipping_rate: Deserialize::default(),
                tax_behavior: Deserialize::default(),
                tax_breakdown: Deserialize::default(),
                tax_code: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                amount: self.amount?,
                amount_tax: self.amount_tax?,
                shipping_rate: self.shipping_rate.take()?,
                tax_behavior: self.tax_behavior?,
                tax_breakdown: self.tax_breakdown.take()?,
                tax_code: self.tax_code.take()?,
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

    impl ObjectDeser for TaxProductResourceTaxCalculationShippingCost {
        type Builder = TaxProductResourceTaxCalculationShippingCostBuilder;
    }

    impl FromValueOpt for TaxProductResourceTaxCalculationShippingCost {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductResourceTaxCalculationShippingCostBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = Some(FromValueOpt::from_value(v)?),
                    "amount_tax" => b.amount_tax = Some(FromValueOpt::from_value(v)?),
                    "shipping_rate" => b.shipping_rate = Some(FromValueOpt::from_value(v)?),
                    "tax_behavior" => b.tax_behavior = Some(FromValueOpt::from_value(v)?),
                    "tax_breakdown" => b.tax_breakdown = Some(FromValueOpt::from_value(v)?),
                    "tax_code" => b.tax_code = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Specifies whether the `amount` includes taxes.
/// If `tax_behavior=inclusive`, then the amount includes taxes.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceTaxCalculationShippingCostTaxBehavior {
    Exclusive,
    Inclusive,
}
impl TaxProductResourceTaxCalculationShippingCostTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceTaxCalculationShippingCostTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
        }
    }
}

impl std::str::FromStr for TaxProductResourceTaxCalculationShippingCostTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceTaxCalculationShippingCostTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TaxProductResourceTaxCalculationShippingCostTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceTaxCalculationShippingCostTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxProductResourceTaxCalculationShippingCostTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxProductResourceTaxCalculationShippingCostTaxBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TaxProductResourceTaxCalculationShippingCostTaxBehavior>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TaxProductResourceTaxCalculationShippingCostTaxBehavior::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxProductResourceTaxCalculationShippingCostTaxBehavior);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxProductResourceTaxCalculationShippingCostTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TaxProductResourceTaxCalculationShippingCostTaxBehavior",
            )
        })
    }
}