use fluent_0::{FluentArgs, FluentBundle, FluentResource};

use crate::validation::{
    ArrayErrors, Errors, ItemErrorsMap, ObjectErrors, PropertyErrorsMap, VecErrors,
};

pub trait IntoLocalization {
    type Target;

    fn into_localization(self, bundle: &FluentBundle<FluentResource>) -> Self::Target;
}

impl IntoLocalization for Errors<crate::validation::Error> {
    type Target = Errors<String>;

    fn into_localization(self, bundle: &FluentBundle<FluentResource>) -> Self::Target {
        match self {
            Errors::Array(array) => Errors::Array(array.into_localization(bundle)),
            Errors::Object(object) => Errors::Object(object.into_localization(bundle)),
            Errors::NewType(newtype) => Errors::NewType(newtype.into_localization(bundle)),
        }
    }
}

impl IntoLocalization for ArrayErrors<crate::validation::Error> {
    type Target = ArrayErrors<String>;

    fn into_localization(self, bundle: &FluentBundle<FluentResource>) -> Self::Target {
        ArrayErrors {
            errors: self.errors.into_localization(bundle),
            items: self.items.into_localization(bundle),
        }
    }
}

impl IntoLocalization for ObjectErrors<crate::validation::Error> {
    type Target = ObjectErrors<String>;

    fn into_localization(self, bundle: &FluentBundle<FluentResource>) -> Self::Target {
        ObjectErrors {
            errors: self.errors.into_localization(bundle),
            properties: self.properties.into_localization(bundle),
        }
    }
}

impl IntoLocalization for VecErrors<crate::validation::Error> {
    type Target = VecErrors<String>;

    fn into_localization(self, bundle: &FluentBundle<FluentResource>) -> Self::Target {
        self.into_iter()
            .map(|error| error.into_localization(bundle))
            .collect()
    }
}

impl IntoLocalization for ItemErrorsMap<crate::validation::Error> {
    type Target = ItemErrorsMap<String>;

    fn into_localization(self, bundle: &FluentBundle<FluentResource>) -> Self::Target {
        self.into_iter()
            .map(|(index, error)| (index, error.into_localization(bundle)))
            .collect()
    }
}

impl IntoLocalization for PropertyErrorsMap<crate::validation::Error> {
    type Target = PropertyErrorsMap<String>;

    fn into_localization(self, bundle: &FluentBundle<FluentResource>) -> Self::Target {
        self.into_iter()
            .map(|(property, error)| (property, error.into_localization(bundle)))
            .collect()
    }
}

impl IntoLocalization for crate::validation::Error {
    type Target = String;

    fn into_localization(self, bundle: &FluentBundle<FluentResource>) -> Self::Target {
        match self {
            Self::Fluent(error) => {
                if let Some(message) = bundle.get_message(error.id) {
                    if let Some(pattern) = message.value() {
                        let mut errors = vec![];
                        let args = FluentArgs::from_iter(error.args);
                        let value = bundle
                            .format_pattern(pattern, Some(&args), &mut errors)
                            .to_string();

                        if errors.is_empty() {
                            value
                        } else {
                            error.id.to_string()
                        }
                    } else {
                        error.id.to_string()
                    }
                } else {
                    error.id.to_string()
                }
            }
            _ => format!("{self}"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::fluent::FluentError;

    use super::*;
    use fluent_0::{FluentResource, FluentValue};
    use unic_langid::LanguageIdentifier;

    #[test]
    fn into_localization_without_args() {
        let ftl_string = "hello-world = Hello, world!".to_string();
        let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");

        let langid_en: LanguageIdentifier = "en-US".parse().expect("Parsing failed");
        let mut bundle = FluentBundle::new(vec![langid_en]);
        bundle.add_resource(res).unwrap();

        let error = crate::validation::Error::Fluent(FluentError {
            id: "hello-world",
            args: vec![],
        });

        assert_eq!(error.into_localization(&bundle), "Hello, world!");
    }

    #[test]
    fn into_localization_with_args() {
        let ftl_string = "intro = Welcome, { $name }.".to_string();
        let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");

        let langid_en: LanguageIdentifier = "en-US".parse().expect("Parsing failed");
        let mut bundle = FluentBundle::new(vec![langid_en]);
        bundle.add_resource(res).unwrap();

        let error = crate::validation::Error::Fluent(FluentError {
            id: "intro",
            args: vec![("name", FluentValue::from("John"))],
        });

        assert_eq!(
            error.into_localization(&bundle),
            "Welcome, \u{2068}John\u{2069}."
        );
    }
}
