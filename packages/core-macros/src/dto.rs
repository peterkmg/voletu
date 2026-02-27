use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemEnum, ItemStruct};

pub(crate) fn request_dto(item: TokenStream) -> TokenStream {
  let item_struct = parse_macro_input!(item as ItemStruct);

  TokenStream::from(quote! {
    #[derive(
      Debug,
      serde::Deserialize,
      serde::Serialize,
      validator::Validate,
      utoipa::ToSchema,
      ts_rs::TS
    )]
    #[serde(rename_all = "camelCase")]
    #[ts(export)]
    #item_struct
  })
}

pub(crate) fn response_dto(item: TokenStream) -> TokenStream {
  let item_struct = parse_macro_input!(item as ItemStruct);

  TokenStream::from(quote! {
    #[derive(Debug, serde::Serialize, utoipa::ToSchema, ts_rs::TS)]
    #[serde(rename_all = "camelCase")]
    #[ts(export)]
    #item_struct
  })
}

pub(crate) fn enum_dto(item: TokenStream) -> TokenStream {
  let item_enum = parse_macro_input!(item as ItemEnum);

  TokenStream::from(quote! {
    #[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, ts_rs::TS)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[ts(export)]
    #item_enum
  })
}
