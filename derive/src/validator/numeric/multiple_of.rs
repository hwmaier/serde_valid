mod multiple_of_from_meta_list;
mod multiple_of_from_meta_name_value;

pub use multiple_of_from_meta_list::extract_numeric_multiple_of_validator_from_meta_list;
pub use multiple_of_from_meta_name_value::extract_numeric_multiple_of_validator_from_meta_name_value;
use proc_macro2::TokenStream;
use quote::quote;

const VALIDATION_LABEL: &'static str = "multiple_of";

fn inner_extract_numeric_multiple_of_validator(
    field_ident: &syn::Ident,
    multiple_of: crate::lit::LitNumeric,
    message: TokenStream,
) -> TokenStream {
    let field_string = field_ident.to_string();
    quote!(
        if !::serde_valid::validate_numeric_multiple_of(
            *#field_ident,
            #multiple_of,
        ) {
            errors
                .entry(::serde_valid::FieldName::new(#field_string))
                .or_default()
                .push(::serde_valid::validation::Error::MultipleOfError(
                    ::serde_valid::validation::error::Message::new(
                        ::serde_valid::validation::error::MultipleOfErrorParams::new(
                            *#field_ident,
                            #multiple_of,
                        ),
                        #message
                    )
                ));
        }
    )
}