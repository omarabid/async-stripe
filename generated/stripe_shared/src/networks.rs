#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Networks {
    /// All available networks for the card.
    pub available: Vec<String>,
    /// The preferred network for co-branded cards.
    /// Can be `cartes_bancaires`, `mastercard`, `visa` or `invalid_preference` if requested network is not valid for the card.
    pub preferred: Option<String>,
}
#[doc(hidden)]
pub struct NetworksBuilder {
    available: Option<Vec<String>>,
    preferred: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Networks {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Networks>,
        builder: NetworksBuilder,
    }

    impl Visitor for Place<Networks> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: NetworksBuilder::deser_default() }))
        }
    }

    impl MapBuilder for NetworksBuilder {
        type Out = Networks;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "available" => Deserialize::begin(&mut self.available),
                "preferred" => Deserialize::begin(&mut self.preferred),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { available: Deserialize::default(), preferred: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { available: self.available.take()?, preferred: self.preferred.take()? })
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

    impl ObjectDeser for Networks {
        type Builder = NetworksBuilder;
    }

    impl FromValueOpt for Networks {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = NetworksBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "available" => b.available = Some(FromValueOpt::from_value(v)?),
                    "preferred" => b.preferred = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};