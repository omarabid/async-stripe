#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicePaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    pub preferred_language: InvoicePaymentMethodOptionsBancontactPreferredLanguage,
}
#[doc(hidden)]
pub struct InvoicePaymentMethodOptionsBancontactBuilder {
    preferred_language: Option<InvoicePaymentMethodOptionsBancontactPreferredLanguage>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoicePaymentMethodOptionsBancontact {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicePaymentMethodOptionsBancontact>,
        builder: InvoicePaymentMethodOptionsBancontactBuilder,
    }

    impl Visitor for Place<InvoicePaymentMethodOptionsBancontact> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoicePaymentMethodOptionsBancontactBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoicePaymentMethodOptionsBancontactBuilder {
        type Out = InvoicePaymentMethodOptionsBancontact;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "preferred_language" => Deserialize::begin(&mut self.preferred_language),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { preferred_language: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { preferred_language: self.preferred_language? })
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

    impl ObjectDeser for InvoicePaymentMethodOptionsBancontact {
        type Builder = InvoicePaymentMethodOptionsBancontactBuilder;
    }

    impl FromValueOpt for InvoicePaymentMethodOptionsBancontact {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoicePaymentMethodOptionsBancontactBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "preferred_language" => {
                        b.preferred_language = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Preferred language of the Bancontact authorization page that the customer is redirected to.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoicePaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}
impl InvoicePaymentMethodOptionsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        use InvoicePaymentMethodOptionsBancontactPreferredLanguage::*;
        match self {
            De => "de",
            En => "en",
            Fr => "fr",
            Nl => "nl",
        }
    }
}

impl std::str::FromStr for InvoicePaymentMethodOptionsBancontactPreferredLanguage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoicePaymentMethodOptionsBancontactPreferredLanguage::*;
        match s {
            "de" => Ok(De),
            "en" => Ok(En),
            "fr" => Ok(Fr),
            "nl" => Ok(Nl),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for InvoicePaymentMethodOptionsBancontactPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoicePaymentMethodOptionsBancontactPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InvoicePaymentMethodOptionsBancontactPreferredLanguage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InvoicePaymentMethodOptionsBancontactPreferredLanguage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<InvoicePaymentMethodOptionsBancontactPreferredLanguage>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InvoicePaymentMethodOptionsBancontactPreferredLanguage::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(InvoicePaymentMethodOptionsBancontactPreferredLanguage);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InvoicePaymentMethodOptionsBancontactPreferredLanguage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for InvoicePaymentMethodOptionsBancontactPreferredLanguage",
            )
        })
    }
}