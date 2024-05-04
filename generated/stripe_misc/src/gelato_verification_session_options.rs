#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoVerificationSessionOptions {
    pub document: Option<stripe_misc::GelatoSessionDocumentOptions>,
    pub id_number: Option<stripe_misc::GelatoSessionIdNumberOptions>,
}
#[doc(hidden)]
pub struct GelatoVerificationSessionOptionsBuilder {
    document: Option<Option<stripe_misc::GelatoSessionDocumentOptions>>,
    id_number: Option<Option<stripe_misc::GelatoSessionIdNumberOptions>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for GelatoVerificationSessionOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoVerificationSessionOptions>,
        builder: GelatoVerificationSessionOptionsBuilder,
    }

    impl Visitor for Place<GelatoVerificationSessionOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoVerificationSessionOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoVerificationSessionOptionsBuilder {
        type Out = GelatoVerificationSessionOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "document" => Deserialize::begin(&mut self.document),
                "id_number" => Deserialize::begin(&mut self.id_number),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { document: Deserialize::default(), id_number: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { document: self.document.take()?, id_number: self.id_number? })
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

    impl ObjectDeser for GelatoVerificationSessionOptions {
        type Builder = GelatoVerificationSessionOptionsBuilder;
    }

    impl FromValueOpt for GelatoVerificationSessionOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoVerificationSessionOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "document" => b.document = Some(FromValueOpt::from_value(v)?),
                    "id_number" => b.id_number = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};