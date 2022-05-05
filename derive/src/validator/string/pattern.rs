use crate::{
    types::Field,
    validator::{common::get_str, Validator},
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_string_pattern_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
) -> Result<Validator, crate::Error> {
    if let Some(array_field) = field.array_field() {
        Ok(Validator::Array(Box::new(
            extract_string_pattern_validator(&array_field, validation_value)?,
        )))
    } else if let Some(option_field) = field.option_field() {
        Ok(Validator::Option(Box::new(
            extract_string_pattern_validator(&option_field, validation_value)?,
        )))
    } else {
        Ok(Validator::Normal(inner_extract_string_pattern_validator(
            field,
            validation_value,
        )?))
    }
}

fn inner_extract_string_pattern_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
) -> Result<TokenStream, crate::Error> {
    let field_name = field.name();
    let field_ident = field.ident();
    let pattern = get_str(validation_value)?;
    let message = quote!(::serde_valid::PatternParams::to_default_message);
    let pattern_ident = syn::Ident::new(
        &format!("{}_PATTERN", &field_ident).to_uppercase(),
        field_ident.span(),
    );

    Ok(quote!(
        static #pattern_ident : once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        let __pattern = #pattern_ident.get_or_init(|| regex::Regex::new(#pattern).unwrap());
        if !::serde_valid::validate_string_pattern(
            #field_ident,
            __pattern,
        ) {
            use ::serde_valid::error::ToDefaultMessage;
            __errors
                .entry(#field_name)
                .or_default()
                .push(::serde_valid::validation::Error::Pattern(
                    ::serde_valid::error::Message::new(
                        ::serde_valid::PatternParams::new(
                            #field_ident,
                            __pattern,
                        ),
                        #message
                    )
                ));
        }
    ))
}
