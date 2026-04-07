use proc_macro::TokenStream;
use quote::quote;
use syn::{
  parse::Parser,
  parse_macro_input,
  parse_quote,
  Expr,
  Fields,
  ItemStruct,
  Meta,
  Path,
  Token,
};

const BEFORE_SAVE_PATH_ERROR: &str = "before_save must be a function path";
const UNSUPPORTED_ATTR_ERROR: &str = "unsupported attribute. Use #[handle_service_fields], #[handle_uuid], or #[handle_timestamps] optionally with (before_save = path::to::fn)";

enum HookMode {
  Uuid,
  Timestamps,
  UuidTimestamps,
}

pub(crate) fn handle_audit(item: TokenStream) -> TokenStream {
  let mut item_struct = parse_macro_input!(item as ItemStruct);

  if let Fields::Named(fields) = &mut item_struct.fields {
    let audit_fields: [syn::Field; 7] = [
      parse_quote! { pub created_at: DateTimeUtc },
      parse_quote! { pub updated_at: DateTimeUtc },
      parse_quote! { pub deleted_at: Option<DateTimeUtc> },
      parse_quote! { pub created_by: Uuid },
      parse_quote! { pub updated_by: Uuid },
      parse_quote! { pub deleted_by: Option<Uuid> },
      parse_quote! { pub origin_db_id: Uuid },
    ];
    fields.named.extend(audit_fields);
  }

  TokenStream::from(quote! {
    #item_struct
  })
}

pub(crate) fn before_save_uuid_created_updated(item: TokenStream) -> TokenStream {
  expand_before_save(item, HookMode::UuidTimestamps, None)
}

pub(crate) fn handle_service_fields(attr: TokenStream, item: TokenStream) -> TokenStream {
  parse_and_expand(attr, item, HookMode::UuidTimestamps)
}

pub(crate) fn handle_uuid(attr: TokenStream, item: TokenStream) -> TokenStream {
  parse_and_expand(attr, item, HookMode::Uuid)
}

pub(crate) fn handle_timestamps(attr: TokenStream, item: TokenStream) -> TokenStream {
  parse_and_expand(attr, item, HookMode::Timestamps)
}

fn parse_and_expand(attr: TokenStream, item: TokenStream, mode: HookMode) -> TokenStream {
  let custom_before_save = match parse_custom_before_save(attr) {
    Ok(value) => value,
    Err(error) => return error.to_compile_error().into(),
  };

  expand_before_save(item, mode, custom_before_save)
}

fn has_named_field(item_struct: &ItemStruct, name: &str) -> bool {
  let Fields::Named(fields) = &item_struct.fields else {
    return false;
  };

  fields
    .named
    .iter()
    .any(|field| field.ident.as_ref().is_some_and(|ident| ident == name))
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
      Meta::NameValue(nv) if nv.path.is_ident("before_save") => {
        let parsed = parse_before_save_value(nv.value)?;
        custom = Some(parsed);
      }
      other => {
        return Err(syn::Error::new_spanned(other, UNSUPPORTED_ATTR_ERROR));
      }
    }
  }

  Ok(custom)
}

fn parse_before_save_value(value: Expr) -> syn::Result<Path> {
  match value {
    Expr::Path(path_expr) => Ok(path_expr.path),
    Expr::Lit(expr_lit) => {
      if let syn::Lit::Str(lit_str) = expr_lit.lit {
        syn::parse_str::<Path>(&lit_str.value())
      } else {
        Err(syn::Error::new_spanned(expr_lit, BEFORE_SAVE_PATH_ERROR))
      }
    }
    other => Err(syn::Error::new_spanned(other, BEFORE_SAVE_PATH_ERROR)),
  }
}

fn expand_before_save(
  item: TokenStream,
  mode: HookMode,
  custom_before_save: Option<Path>,
) -> TokenStream {
  let mut item_struct = parse_macro_input!(item as ItemStruct);
  item_struct
    .attrs
    .push(parse_quote!(#[derive(serde::Serialize, serde::Deserialize)]));

  let has_id = has_named_field(&item_struct, "id");
  let has_created_at = has_named_field(&item_struct, "created_at");
  let has_updated_at = has_named_field(&item_struct, "updated_at");
  let has_created_by = has_named_field(&item_struct, "created_by");
  let has_updated_by = has_named_field(&item_struct, "updated_by");
  let has_origin_db_id = has_named_field(&item_struct, "origin_db_id");
  let has_version = has_named_field(&item_struct, "version");

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

  let version_step = if has_version {
    quote! {
      if insert {
        crate::utils::model::set_on_insert(&mut self.version, true, 1_i32);
      } else {
        match self.version.clone() {
          sea_orm::ActiveValue::Set(version) | sea_orm::ActiveValue::Unchanged(version) => {
            self.version = sea_orm::ActiveValue::Set(version + 1);
          }
          sea_orm::ActiveValue::NotSet => {}
        }
      }
    }
  } else {
    quote! {}
  };

  let hook_call = match mode {
    HookMode::Uuid => {
      quote! {
        #uuid_step
      }
    }
    HookMode::Timestamps => {
      quote! {
        #timestamps_now
        #created_at_step
        #updated_at_step
      }
    }
    HookMode::UuidTimestamps => {
      quote! {
        #uuid_step
        #timestamps_now
        #created_at_step
        #updated_at_step
        #created_by_step
        #updated_by_step
        #origin_step
        #version_step
      }
    }
  };

  let after_save_hook = if matches!(mode, HookMode::UuidTimestamps) && has_id {
    quote! {
      async fn after_save<C: sea_orm::ConnectionTrait>(
        model: <Entity as sea_orm::EntityTrait>::Model,
        db: &C,
        insert: bool,
      ) -> Result<<Entity as sea_orm::EntityTrait>::Model, sea_orm::DbErr> {
        if insert {
          let _ = crate::services::audit::register::entity_after_insert_hook(
            db, model.id, &model,
          ).await;
        }
        Ok(model)
      }
    }
  } else {
    quote! {}
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

      #after_save_hook
    }
  })
}
