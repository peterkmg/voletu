use proc_macro::TokenStream;
use quote::quote;
use syn::{
  parse::Parser, parse_macro_input, parse_quote, Expr, Fields, ItemStruct, Meta, Path, Token,
};

fn has_named_field(item_struct: &ItemStruct, name: &str) -> bool {
  match &item_struct.fields {
    Fields::Named(fields) => fields
      .named
      .iter()
      .any(|field| field.ident.as_ref().is_some_and(|ident| ident == name)),
    _ => false,
  }
}

fn parse_custom_before_save(attr: TokenStream) -> syn::Result<Option<Path>> {
  if attr.is_empty() {
    return Ok(None);
  }

  if let Ok(path) = syn::parse::<Path>(attr.clone()) {
    return Ok(Some(path));
  }

  let parser = syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated;
  let metas = parser.parse(attr)?;

  let mut custom = None;
  for meta in metas {
    match meta {
      Meta::NameValue(nv) if nv.path.is_ident("before_save") => match nv.value {
        Expr::Path(path_expr) => custom = Some(path_expr.path),
        Expr::Lit(expr_lit) => {
          if let syn::Lit::Str(s) = expr_lit.lit {
            custom = Some(syn::parse_str::<Path>(&s.value())?);
          } else {
            return Err(syn::Error::new_spanned(
              expr_lit,
              "before_save must be a function path",
            ));
          }
        }
        other => {
          return Err(syn::Error::new_spanned(
            other,
            "before_save must be a function path",
          ));
        }
      },
      other => {
        return Err(syn::Error::new_spanned(
          other,
          "unsupported attribute. Use #[handle_uuid_timestamps], #[handle_uuid], or #[handle_timestamps] optionally with (before_save = path::to::fn)",
        ));
      }
    }
  }

  Ok(custom)
}

enum HookMode {
  Uuid,
  Timestamps,
  UuidTimestamps,
}

fn expand_before_save(
  item: TokenStream,
  mode: HookMode,
  custom_before_save: Option<Path>,
) -> TokenStream {
  let item_struct = parse_macro_input!(item as ItemStruct);

  let has_id = has_named_field(&item_struct, "id");
  let has_created_at = has_named_field(&item_struct, "created_at");
  let has_updated_at = has_named_field(&item_struct, "updated_at");
  let has_created_by = has_named_field(&item_struct, "created_by");
  let has_updated_by = has_named_field(&item_struct, "updated_by");
  let has_origin_db_id = has_named_field(&item_struct, "origin_db_id");

  let custom_call = if let Some(path) = custom_before_save {
    quote! {
      self = #path(self, _db, insert).await?;
    }
  } else {
    quote! {}
  };

  let uuid_step = if has_id {
    quote! {
      crate::utils::model::apply_uuid_on_insert(&mut self.id, insert);
    }
  } else {
    quote! {}
  };

  let timestamps_now = if has_created_at || has_updated_at {
    quote! {
      let now = sea_orm::prelude::ChronoUtc::now();
    }
  } else {
    quote! {}
  };

  let created_at_step = if has_created_at {
    quote! {
      crate::utils::model::set_on_insert(&mut self.created_at, insert, now);
    }
  } else {
    quote! {}
  };

  let updated_at_step = if has_updated_at {
    quote! {
      crate::utils::model::set_if_not_set_or_unchanged(&mut self.updated_at, now);
    }
  } else {
    quote! {}
  };

  let created_by_step = if has_created_by {
    quote! {
      if insert && matches!(self.created_by, sea_orm::ActiveValue::NotSet) {
        let actor_id = crate::context::audit::current_actor_id_or_err()?;
        crate::utils::model::set_on_insert(&mut self.created_by, insert, actor_id);
      }
    }
  } else {
    quote! {}
  };

  let updated_by_step = if has_updated_by {
    quote! {
      if matches!(self.updated_by, sea_orm::ActiveValue::NotSet | sea_orm::ActiveValue::Unchanged(_)) {
        let actor_id = crate::context::audit::current_actor_id_or_err()?;
        crate::utils::model::set_if_not_set_or_unchanged(&mut self.updated_by, actor_id);
      }
    }
  } else {
    quote! {}
  };

  let origin_step = if has_origin_db_id {
    quote! {
      if insert && matches!(self.origin_db_id, sea_orm::ActiveValue::NotSet) {
        let origin_db_id = crate::context::audit::current_origin_db_id_or_err()?;
        crate::utils::model::set_on_insert(&mut self.origin_db_id, insert, origin_db_id);
      }
    }
  } else {
    quote! {}
  };

  let hook_call = match mode {
    HookMode::Uuid => quote! {
      #uuid_step
    },
    HookMode::Timestamps => quote! {
      #timestamps_now
      #created_at_step
      #updated_at_step
    },
    HookMode::UuidTimestamps => quote! {
      #uuid_step
      #timestamps_now
      #created_at_step
      #updated_at_step
      #created_by_step
      #updated_by_step
      #origin_step
    },
  };

  TokenStream::from(quote! {
    #item_struct

    #[sea_orm::prelude::async_trait::async_trait]
    impl sea_orm::ActiveModelBehavior for ActiveModel {
      async fn before_save<C: sea_orm::ConnectionTrait>(
        mut self,
        _db: &C,
        insert: bool,
      ) -> Result<Self, sea_orm::DbErr> {
        #custom_call
        #hook_call
        Ok(self)
      }
    }
  })
}

#[proc_macro_attribute]
pub fn with_audit_fields(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let mut item_struct = parse_macro_input!(item as ItemStruct);

  if let Fields::Named(fields) = &mut item_struct.fields {
    fields
      .named
      .push(parse_quote! { pub created_at: DateTimeUtc });
    fields
      .named
      .push(parse_quote! { pub updated_at: DateTimeUtc });
    fields
      .named
      .push(parse_quote! { pub deleted_at: Option<DateTimeUtc> });
    fields.named.push(parse_quote! { pub created_by: Uuid });
    fields.named.push(parse_quote! { pub updated_by: Uuid });
    fields
      .named
      .push(parse_quote! { pub deleted_by: Option<Uuid> });
    fields.named.push(parse_quote! { pub origin_db_id: Uuid });
  }

  TokenStream::from(quote! {
    #item_struct
  })
}

#[proc_macro_attribute]
pub fn before_save_uuid_created_updated(_attr: TokenStream, item: TokenStream) -> TokenStream {
  expand_before_save(item, HookMode::UuidTimestamps, None)
}

#[proc_macro_attribute]
pub fn handle_uuid_timestamps(attr: TokenStream, item: TokenStream) -> TokenStream {
  let custom_before_save = match parse_custom_before_save(attr) {
    Ok(v) => v,
    Err(e) => return e.to_compile_error().into(),
  };
  expand_before_save(item, HookMode::UuidTimestamps, custom_before_save)
}

#[proc_macro_attribute]
pub fn handle_uuid(attr: TokenStream, item: TokenStream) -> TokenStream {
  let custom_before_save = match parse_custom_before_save(attr) {
    Ok(v) => v,
    Err(e) => return e.to_compile_error().into(),
  };
  expand_before_save(item, HookMode::Uuid, custom_before_save)
}

#[proc_macro_attribute]
pub fn handle_timestamps(attr: TokenStream, item: TokenStream) -> TokenStream {
  let custom_before_save = match parse_custom_before_save(attr) {
    Ok(v) => v,
    Err(e) => return e.to_compile_error().into(),
  };
  expand_before_save(item, HookMode::Timestamps, custom_before_save)
}
