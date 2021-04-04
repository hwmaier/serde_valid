use crate::abort::{
    abort_unexpected_list_argument, abort_unexpected_lit_argument,
    abort_unexpected_name_value_argument,
};
use crate::types::NamedField;
use crate::validator::Validator;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::spanned::Spanned;

const VALIDATION_LABEL: &'static str = "custom";

pub fn extract_generic_custom_validator(
    field: &NamedField,
    attribute: &syn::Attribute,
    meta_list: &syn::MetaList,
) -> Validator {
    let mut custom_validation_fn = None;
    let mut custom_validation_args = None;
    let field_ident = field.ident();
    for meta_item in &meta_list.nested {
        match meta_item {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::Path(path) => {
                    update_custom_validator_from_meta_path(&mut custom_validation_fn, path)
                }
                syn::Meta::List(list) => update_custom_validator_from_meta_list(
                    &mut custom_validation_fn,
                    &mut custom_validation_args,
                    field_ident,
                    list,
                ),
                syn::Meta::NameValue(name_value) => abort_unexpected_name_value_argument(
                    VALIDATION_LABEL,
                    field_ident,
                    attribute.span(),
                    name_value,
                ),
            },
            syn::NestedMeta::Lit(lit) => {
                abort_unexpected_lit_argument(VALIDATION_LABEL, field_ident, attribute.span(), &lit)
            }
        }
    }

    let custom_validation_fn = custom_validation_fn.unwrap_or_else(|| {
        abort!(
            attribute.span(),
            &format!(
                "'{}' literal meta items size must be greater than 0.",
                VALIDATION_LABEL
            )
        )
    });

    let field_string = field_ident.to_string();
    return Validator::Normal(quote!(
        if let Err(error) = #custom_validation_fn(#field_ident, #custom_validation_args) {
            errors
                .entry(::serde_valid::FieldName::new(#field_string))
                .or_default()
                .push(error);
        };
    ));
}

fn update_custom_validator_from_meta_path(
    custom_validation_fn: &mut Option<TokenStream>,
    path: &syn::Path,
) {
    check_duplicated_custom_validation_fn(custom_validation_fn, path);
    *custom_validation_fn = Some(quote!(#path));
}

fn update_custom_validator_from_meta_list(
    custom_validation_fn: &mut Option<TokenStream>,
    custom_validation_args: &mut Option<TokenStream>,
    field_ident: &syn::Ident,
    meta_list: &syn::MetaList,
) {
    let fn_name = &meta_list.path;
    let args = extract_custom_validator_args(field_ident, &meta_list.nested);
    check_duplicated_custom_validation_fn(custom_validation_fn, fn_name);
    check_duplicated_custom_validation_args(custom_validation_args, fn_name, &args);

    *custom_validation_fn = Some(quote!(#fn_name));
    *custom_validation_args = Some(quote!(#args));
}

fn extract_custom_validator_args(
    field_ident: &syn::Ident,
    nested: &syn::punctuated::Punctuated<syn::NestedMeta, syn::Token![,]>,
) -> syn::punctuated::Punctuated<TokenStream, syn::Token![,]> {
    nested
        .iter()
        .map(|nested_meta| extract_custom_validator_arg(field_ident, nested_meta))
        .collect()
}

fn extract_custom_validator_arg(
    field_ident: &syn::Ident,
    nested_meta: &syn::NestedMeta,
) -> TokenStream {
    match nested_meta {
        syn::NestedMeta::Lit(lit) => quote!(#lit),
        syn::NestedMeta::Meta(meta) => match meta {
            syn::Meta::Path(path) => quote!(&self.#path),
            syn::Meta::List(list) => abort_unexpected_list_argument(
                VALIDATION_LABEL,
                field_ident,
                nested_meta.span(),
                &list,
            ),
            syn::Meta::NameValue(name_value) => abort_unexpected_name_value_argument(
                VALIDATION_LABEL,
                field_ident,
                nested_meta.span(),
                &name_value,
            ),
        },
    }
}

fn check_duplicated_custom_validation_fn(
    custom_validation_fn: &Option<TokenStream>,
    path: &syn::Path,
) {
    if custom_validation_fn.is_some() {
        abort!(
            path.span(),
            &format!("'{}' validator allow 1 custom function.", VALIDATION_LABEL)
        )
    }
}

fn check_duplicated_custom_validation_args(
    custom_validation_args: &Option<TokenStream>,
    path: &syn::Path,
    nested: &syn::punctuated::Punctuated<TokenStream, syn::Token![,]>,
) {
    if custom_validation_args.is_some() {
        abort!(
            path.span(),
            &format!(
                "'{}' validator allow to define custom function args only once.",
                VALIDATION_LABEL
            )
        )
    }
    if nested.is_empty() {
        abort!(
            path.span(),
            &format!(
                "'{}' validator need 1 or more custom function args.",
                VALIDATION_LABEL
            )
        )
    }
}
