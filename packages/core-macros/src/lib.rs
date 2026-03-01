mod dto;
mod model;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn with_audit_fields(_attr: TokenStream, item: TokenStream) -> TokenStream {
  model::with_audit_fields(item)
}

#[proc_macro_attribute]
pub fn before_save_uuid_created_updated(_attr: TokenStream, item: TokenStream) -> TokenStream {
  model::before_save_uuid_created_updated(item)
}

#[proc_macro_attribute]
pub fn handle_uuid_timestamps(attr: TokenStream, item: TokenStream) -> TokenStream {
  model::handle_uuid_timestamps(attr, item)
}

#[proc_macro_attribute]
pub fn handle_uuid(attr: TokenStream, item: TokenStream) -> TokenStream {
  model::handle_uuid(attr, item)
}

#[proc_macro_attribute]
pub fn handle_timestamps(attr: TokenStream, item: TokenStream) -> TokenStream {
  model::handle_timestamps(attr, item)
}

#[proc_macro_attribute]
pub fn request_dto(_attr: TokenStream, item: TokenStream) -> TokenStream {
  dto::request_dto(item)
}

#[proc_macro_attribute]
pub fn response_dto(_attr: TokenStream, item: TokenStream) -> TokenStream {
  dto::response_dto(item)
}

#[proc_macro_attribute]
pub fn enum_type(attr: TokenStream, item: TokenStream) -> TokenStream {
  dto::enum_type(attr, item)
}
