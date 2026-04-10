mod args;
mod generator;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemImpl};

use self::{args::EntityServiceArgs, generator::expand_entity_service};

pub(crate) fn entity_service(attr: TokenStream, item: TokenStream) -> TokenStream {
  let args = parse_macro_input!(attr as EntityServiceArgs);
  let input_impl = parse_macro_input!(item as ItemImpl);

  expand_entity_service(args, input_impl)
    .unwrap_or_else(|error| error.to_compile_error())
    .into()
}
