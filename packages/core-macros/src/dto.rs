use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
  parse_macro_input,
  parse_quote,
  AngleBracketedGenericArguments,
  Attribute,
  Field,
  Fields,
  GenericArgument,
  ItemEnum,
  ItemStruct,
  Meta,
  PathArguments,
  Type,
  TypePath,
};

fn type_path_ident(type_path: &TypePath) -> Option<&syn::Ident> {
  type_path.path.segments.last().map(|segment| &segment.ident)
}

fn option_inner_type(ty: &Type) -> Option<&Type> {
  let Type::Path(type_path) = ty else {
    return None;
  };

  let Some(ident) = type_path_ident(type_path) else {
    return None;
  };

  if ident != "Option" {
    return None;
  }

  let Some(segment) = type_path.path.segments.last() else {
    return None;
  };

  let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =
    &segment.arguments
  else {
    return None;
  };

  args.first().and_then(|arg| {
    if let GenericArgument::Type(inner) = arg {
      Some(inner)
    } else {
      None
    }
  })
}

fn is_path_type(ty: &Type, target: &str) -> bool {
  let Type::Path(type_path) = ty else {
    return false;
  };

  type_path_ident(type_path)
    .map(|ident| ident == target)
    .unwrap_or(false)
}

fn is_decimal(ty: &Type) -> bool {
  is_path_type(ty, "Decimal")
}

fn is_option_decimal(ty: &Type) -> bool {
  option_inner_type(ty).map(is_decimal).unwrap_or(false)
}

fn is_option_string(ty: &Type) -> bool {
  option_inner_type(ty)
    .map(|inner| is_path_type(inner, "String"))
    .unwrap_or(false)
}

fn has_attr(attrs: &[Attribute], attr_name: &str, needle: &str) -> bool {
  attrs.iter().any(|attr| {
    attr.path().is_ident(attr_name) && attr.meta.to_token_stream().to_string().contains(needle)
  })
}

fn has_schema_attr(field: &Field) -> bool {
  field
    .attrs
    .iter()
    .any(|attr| attr.path().is_ident("schema"))
}

fn field_name(field: &Field) -> Option<String> {
  field.ident.as_ref().map(ToString::to_string)
}

fn is_amount_field(field: &Field) -> bool {
  field_name(field)
    .map(|name| name == "amount" || name.ends_with("_amount"))
    .unwrap_or(false)
}

fn append_schema_entry(field: &mut Field, tokens: proc_macro2::TokenStream) {
  for attr in &mut field.attrs {
    if !attr.path().is_ident("schema") {
      continue;
    }

    if let Meta::List(list) = &attr.meta {
      let existing = list.tokens.clone();
      if existing.is_empty() {
        *attr = parse_quote!(#[schema(#tokens)]);
      } else {
        *attr = parse_quote!(#[schema(#existing, #tokens)]);
      }
      return;
    }
  }

  field.attrs.push(parse_quote!(#[schema(#tokens)]));
}

fn infer_example(
  ty: &Type,
  is_decimal_field: bool,
  is_option_decimal_field: bool,
) -> Option<proc_macro2::TokenStream> {
  if is_decimal_field || is_option_decimal_field {
    return Some(quote!("100.00"));
  }

  if is_path_type(ty, "String") || is_option_string(ty) {
    return Some(quote!("example"));
  }

  if is_path_type(ty, "Uuid") {
    return Some(quote!("550e8400-e29b-41d4-a716-446655440000"));
  }

  if is_path_type(ty, "DateTime") {
    return Some(quote!("2026-01-01T00:00:00Z"));
  }

  if is_path_type(ty, "NaiveDate") {
    return Some(quote!("2026-01-01"));
  }

  if is_path_type(ty, "bool") {
    return Some(quote!(true));
  }

  if is_path_type(ty, "i32")
    || is_path_type(ty, "i64")
    || is_path_type(ty, "u32")
    || is_path_type(ty, "u64")
    || is_path_type(ty, "usize")
    || is_path_type(ty, "isize")
  {
    return Some(quote!(1));
  }

  None
}

fn process_struct(mut item_struct: ItemStruct, is_request: bool) -> ItemStruct {
  if let Fields::Named(fields) = &mut item_struct.fields {
    for field in &mut fields.named {
      let decimal_field = is_decimal(&field.ty);
      let option_decimal_field = is_option_decimal(&field.ty);
      let string_field = is_path_type(&field.ty, "String");
      let option_string_field = is_option_string(&field.ty);

      if decimal_field || option_decimal_field {
        if !has_attr(&field.attrs, "serde", "with") {
          if option_decimal_field {
            field
              .attrs
              .push(parse_quote!(#[serde(with = "rust_decimal::serde::str_option")]));
          } else {
            field
              .attrs
              .push(parse_quote!(#[serde(with = "rust_decimal::serde::str")]));
          }
        }

        if !has_attr(&field.attrs, "schema", "value_type") {
          if option_decimal_field {
            append_schema_entry(field, quote!(value_type = Option<String>));
          } else {
            append_schema_entry(field, quote!(value_type = String));
          }
        }

        if is_request {
          let validator_fn = if is_amount_field(field) {
            "crate::dtos::validators::validate_positive_decimal"
          } else {
            "crate::dtos::validators::validate_non_negative_decimal"
          };

          if !has_attr(&field.attrs, "validate", validator_fn) {
            if is_amount_field(field) {
              field.attrs.push(parse_quote!(
                #[validate(custom(function = "crate::dtos::validators::validate_positive_decimal"))]
              ));
            } else {
              field.attrs.push(parse_quote!(
                #[validate(custom(function = "crate::dtos::validators::validate_non_negative_decimal"))]
              ));
            }
          }
        }
      }

      if is_request
        && (string_field || option_string_field)
        && has_attr(&field.attrs, "validate", "length")
        && !has_attr(&field.attrs, "validate", "validate_non_blank_string")
      {
        field.attrs.push(parse_quote!(
          #[validate(custom(function = "crate::dtos::validators::validate_non_blank_string"))]
        ));
      }

      if !has_attr(&field.attrs, "schema", "example") {
        if let Some(example) = infer_example(&field.ty, decimal_field, option_decimal_field) {
          if has_schema_attr(field) {
            append_schema_entry(field, quote!(example = #example));
          } else {
            field
              .attrs
              .push(parse_quote!(#[schema(example = #example)]));
          }
        }
      }
    }
  }

  item_struct
}

pub(crate) fn request_dto(item: TokenStream) -> TokenStream {
  let item_struct = process_struct(parse_macro_input!(item as ItemStruct), true);

  TokenStream::from(quote! {
    #[derive(Debug, serde::Deserialize, serde::Serialize, validator::Validate, utoipa::ToSchema)]
    #[serde(rename_all = "camelCase")]
    #item_struct
  })
}

pub(crate) fn response_dto(item: TokenStream) -> TokenStream {
  let item_struct = process_struct(parse_macro_input!(item as ItemStruct), false);

  TokenStream::from(quote! {
    #[derive(Debug, serde::Serialize, utoipa::ToSchema)]
    #[serde(rename_all = "camelCase")]
    #item_struct
  })
}

pub(crate) fn enum_dto(item: TokenStream) -> TokenStream {
  let item_enum = parse_macro_input!(item as ItemEnum);

  TokenStream::from(quote! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #item_enum
  })
}
