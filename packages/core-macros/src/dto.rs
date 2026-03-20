use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
  parenthesized,
  parse::{Parse, ParseStream},
  parse_macro_input,
  parse_quote,
  AngleBracketedGenericArguments,
  Attribute,
  Error,
  Field,
  Fields,
  GenericArgument,
  Ident,
  ItemEnum,
  ItemStruct,
  Meta,
  PathArguments,
  Result,
  Token,
  Type,
  TypePath,
};

struct ResponseDtoArgs {
  service_fields: Vec<Ident>,
  service_fields_specified: bool,
}

fn default_service_field_idents() -> Vec<Ident> {
  vec![
    Ident::new("created_at", proc_macro2::Span::call_site()),
    Ident::new("updated_at", proc_macro2::Span::call_site()),
    Ident::new("deleted_at", proc_macro2::Span::call_site()),
    Ident::new("created_by", proc_macro2::Span::call_site()),
    Ident::new("updated_by", proc_macro2::Span::call_site()),
    Ident::new("deleted_by", proc_macro2::Span::call_site()),
    Ident::new("origin_db_id", proc_macro2::Span::call_site()),
  ]
}

fn document_service_field_idents() -> Vec<Ident> {
  let mut fields = default_service_field_idents();
  fields.push(Ident::new("status", proc_macro2::Span::call_site()));
  fields.push(Ident::new("executed_at", proc_macro2::Span::call_site()));
  fields.push(Ident::new("executed_by", proc_macro2::Span::call_site()));
  fields.push(Ident::new("reverted_at", proc_macro2::Span::call_site()));
  fields.push(Ident::new("reverted_by", proc_macro2::Span::call_site()));
  fields
}

fn resolve_service_fields(input: Vec<Ident>) -> Result<Vec<Ident>> {
  if input.is_empty() {
    return Ok(Vec::new());
  }

  if input.len() == 1 {
    let key = input[0].to_string();
    return match key.as_str() {
      "common" => Ok(default_service_field_idents()),
      "document" => Ok(document_service_field_idents()),
      "all" => {
        let mut fields = document_service_field_idents();
        fields.push(Ident::new("version", proc_macro2::Span::call_site()));
        Ok(fields)
      }
      _ => Ok(input),
    };
  }

  Ok(input)
}

impl Parse for ResponseDtoArgs {
  fn parse(input: ParseStream) -> Result<Self> {
    let mut service_fields = Vec::new();
    let mut service_fields_specified = false;

    while !input.is_empty() {
      let key: Ident = input.parse()?;
      if key == "service_fields" {
        service_fields_specified = true;
        let content;
        parenthesized!(content in input);
        while !content.is_empty() {
          service_fields.push(content.parse()?);
          if content.peek(Token![,]) {
            let _: Token![,] = content.parse()?;
          }
        }
      } else {
        return Err(Error::new_spanned(
          key,
          "Unknown response_dto argument. Supported: service_fields(...)",
        ));
      }

      if input.peek(Token![,]) {
        let _: Token![,] = input.parse()?;
      }
    }

    Ok(Self {
      service_fields,
      service_fields_specified,
    })
  }
}

fn inject_service_fields(
  mut item_struct: ItemStruct,
  service_fields: &[Ident],
) -> Result<ItemStruct> {
  if service_fields.is_empty() {
    return Ok(item_struct);
  }

  let Fields::Named(fields) = &mut item_struct.fields else {
    return Err(Error::new_spanned(
      item_struct,
      "response_dto service_fields(...) is supported only for structs with named fields",
    ));
  };

  for field in service_fields {
    let name = field.to_string();
    let already_present = fields
      .named
      .iter()
      .any(|existing| existing.ident.as_ref().is_some_and(|ident| ident == &name));
    if already_present {
      continue;
    }

    let parsed: Field = match name.as_str() {
      "created_at" | "updated_at" => parse_quote! { pub #field: String },
      "deleted_at" => parse_quote! { pub #field: Option<String> },
      "created_by" | "updated_by" | "origin_db_id" => {
        parse_quote! { pub #field: uuid::Uuid }
      }
      "deleted_by" | "executed_by" | "reverted_by" => {
        parse_quote! { pub #field: Option<uuid::Uuid> }
      }
      "status" => parse_quote! { pub #field: crate::enums::DocumentStatus },
      "executed_at" | "reverted_at" => parse_quote! { pub #field: Option<String> },
      "version" => parse_quote! { pub #field: i32 },
      _ => {
        return Err(Error::new_spanned(
          field,
          "Unsupported service field for response_dto. Supported: created_at, updated_at, deleted_at, created_by, updated_by, deleted_by, origin_db_id, status, executed_at, executed_by, reverted_at, reverted_by, version",
        ));
      }
    };

    fields.named.push(parsed);
  }

  Ok(item_struct)
}

fn to_snake_case(input: &str) -> String {
  let chars: Vec<char> = input.chars().collect();
  let mut out = String::with_capacity(input.len() + 8);

  for (index, ch) in chars.iter().enumerate() {
    if *ch == '-' || *ch == ' ' {
      if !out.ends_with('_') {
        out.push('_');
      }
      continue;
    }

    if ch.is_uppercase() {
      if index > 0 {
        let prev = chars[index - 1];
        let next = chars.get(index + 1).copied();
        let boundary = prev.is_lowercase()
          || prev.is_ascii_digit()
          || (prev.is_uppercase() && next.is_some_and(|c| c.is_lowercase()));
        if boundary && !out.ends_with('_') {
          out.push('_');
        }
      }
      out.extend(ch.to_lowercase());
      continue;
    }

    out.push(*ch);
  }

  out
}

fn to_screaming_snake_case(input: &str) -> String {
  to_snake_case(input).to_ascii_uppercase()
}

fn type_path_ident(type_path: &TypePath) -> Option<&syn::Ident> {
  type_path.path.segments.last().map(|segment| &segment.ident)
}

fn option_inner_type(ty: &Type) -> Option<&Type> {
  let Type::Path(type_path) = ty else {
    return None;
  };

  let ident = type_path_ident(type_path)?;

  if ident != "Option" {
    return None;
  }

  let segment = type_path.path.segments.last()?;

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
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize, validator::Validate, utoipa::ToSchema)]
    #[serde(rename_all = "camelCase")]
    #item_struct
  })
}

pub(crate) fn response_dto(attr: TokenStream, item: TokenStream) -> TokenStream {
  let args = parse_macro_input!(attr as ResponseDtoArgs);
  let service_fields = if args.service_fields_specified {
    match resolve_service_fields(args.service_fields) {
      Ok(value) => value,
      Err(error) => return error.to_compile_error().into(),
    }
  } else {
    Vec::new()
  };
  let item_struct = parse_macro_input!(item as ItemStruct);
  let item_struct = match inject_service_fields(item_struct, &service_fields) {
    Ok(value) => value,
    Err(error) => return error.to_compile_error().into(),
  };
  let item_struct = process_struct(item_struct, false);

  TokenStream::from(quote! {
    #[derive(Debug, serde::Serialize, utoipa::ToSchema)]
    #[serde(rename_all = "camelCase")]
    #item_struct
  })
}

pub(crate) fn enum_type(attr: TokenStream, item: TokenStream) -> TokenStream {
  if !attr.is_empty() {
    return syn::Error::new(
      proc_macro2::Span::call_site(),
      "`#[enum_type]` does not accept arguments",
    )
    .to_compile_error()
    .into();
  }

  let mut item_enum = parse_macro_input!(item as ItemEnum);
  let enum_name = to_snake_case(&item_enum.ident.to_string());

  for variant in &mut item_enum.variants {
    let has_string_value = variant.attrs.iter().any(|attr| {
      attr.path().is_ident("sea_orm")
        && attr
          .meta
          .to_token_stream()
          .to_string()
          .contains("string_value")
    });
    if has_string_value {
      continue;
    }

    let string_value = to_screaming_snake_case(&variant.ident.to_string());
    variant
      .attrs
      .push(parse_quote!(#[sea_orm(string_value = #string_value)]));
  }

  TokenStream::from(quote! {
    #[derive(
      Clone,
      Copy,
      Debug,
      PartialEq,
      Eq,
      ::sea_orm::EnumIter,
      ::sea_orm::DeriveActiveEnum,
      serde::Serialize,
      serde::Deserialize,
      utoipa::ToSchema,
      strum::EnumString,
      strum::Display,
      strum::AsRefStr,
      strum::VariantArray,
    )]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[strum(serialize_all = "SCREAMING_SNAKE_CASE", ascii_case_insensitive)]
    #[sea_orm(rs_type = "String", db_type = "Enum", enum_name = #enum_name)]
    #item_enum
  })
}
