use crate::{
    serde::rename::RenameMap,
    types::Field,
    validate::{
        common::{get_str, CustomMessageToken},
        Validator,
    },
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_string_pattern_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
    custom_message: CustomMessageToken,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    inner_extract_string_pattern_validator(field, validation_value, custom_message, rename_map)
}

fn inner_extract_string_pattern_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
    custom_message: CustomMessageToken,
    rename_map: &RenameMap,
) -> Result<TokenStream, crate::Errors> {
    let field_name = field.name();
    let field_ident = field.ident();
    let field_key = field.key();
    let rename = rename_map.get(field_name).unwrap_or(&field_key);
    let errors = field.errors_variable();
    let pattern = get_str(validation_value)?;
    let pattern_ident = syn::Ident::new(
        &format!("{}_PATTERN", &field_ident).to_uppercase(),
        field_ident.span(),
    );
    let custom_message = custom_message.into_token();

    Ok(quote!(
        static #pattern_ident : ::serde_valid::export::OnceCell<::regex::Regex> = ::serde_valid::export::OnceCell::new();
        let __pattern = #pattern_ident.get_or_init(|| ::regex::Regex::new(#pattern).unwrap());
        if let Err(__composited_error_params) = ::serde_valid::validation::ValidateCompositedPattern::validate_composited_pattern(
            #field_ident,
            __pattern,
        ) {
            use ::serde_valid::validation::{IntoError, ToDefaultMessage};

            #errors
                .entry(#rename)
                .or_default()
                .push(__composited_error_params.into_error_by(#custom_message));
        }
    ))
}
