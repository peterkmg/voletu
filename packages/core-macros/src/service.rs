use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
  parenthesized,
  parse::{Parse, ParseStream},
  parse_macro_input,
  Error,
  Ident,
  ItemImpl,
  LitStr,
  Path,
  Result,
  Token,
};

struct EntityServiceArgs {
  entity: Ident,
  entity_mod: Path,
  create_req: Option<Path>,
  update_req: Option<Path>,
  apply_update: Option<Path>,
  apply_soft_delete: Option<Path>,
  before_create: Option<Path>,
  before_update: Option<Path>,
  before_soft_delete: Option<Path>,
  before_execute: Option<Path>,
  before_revert: Option<Path>,
  response: Option<Path>,
  entity_name: LitStr,
  ops: Vec<Ident>,
}

impl Parse for EntityServiceArgs {
  fn parse(input: ParseStream) -> Result<Self> {
    let mut entity = None;
    let mut entity_mod = None;
    let mut create_req = None;
    let mut update_req = None;
    let mut apply_update = None;
    let mut apply_soft_delete = None;
    let mut before_create = None;
    let mut before_update = None;
    let mut before_soft_delete = None;
    let mut before_execute = None;
    let mut before_revert = None;
    let mut response = None;
    let mut entity_name = None;
    let mut ops = Vec::new();

    while !input.is_empty() {
      let key: Ident = input.parse()?;

      if key == "ops" {
        let content;
        parenthesized!(content in input);
        while !content.is_empty() {
          let op: Ident = content.parse()?;
          ops.push(op);
          if content.peek(Token![,]) {
            let _: Token![,] = content.parse()?;
          }
        }
      } else {
        let _: Token![=] = input.parse()?;
        if key == "entity" {
          entity = Some(input.parse()?);
        } else if key == "entity_mod" {
          entity_mod = Some(input.parse()?);
        } else if key == "create_req" {
          create_req = Some(input.parse()?);
        } else if key == "update_req" {
          update_req = Some(input.parse()?);
        } else if key == "apply_update" {
          apply_update = Some(input.parse()?);
        } else if key == "apply_soft_delete" {
          apply_soft_delete = Some(input.parse()?);
        } else if key == "before_create" {
          before_create = Some(input.parse()?);
        } else if key == "before_update" {
          before_update = Some(input.parse()?);
        } else if key == "before_soft_delete" {
          before_soft_delete = Some(input.parse()?);
        } else if key == "before_execute" {
          before_execute = Some(input.parse()?);
        } else if key == "before_revert" {
          before_revert = Some(input.parse()?);
        } else if key == "response" {
          response = Some(input.parse()?);
        } else if key == "entity_name" {
          entity_name = Some(input.parse()?);
        } else {
          return Err(Error::new_spanned(key, "Unknown entity_service argument"));
        }
      }

      if input.peek(Token![,]) {
        let _: Token![,] = input.parse()?;
      }
    }

    let entity =
      entity.ok_or_else(|| Error::new(proc_macro2::Span::call_site(), "Missing `entity`"))?;
    let entity_mod = entity_mod
      .ok_or_else(|| Error::new(proc_macro2::Span::call_site(), "Missing `entity_mod`"))?;
    let entity_name = entity_name
      .ok_or_else(|| Error::new(proc_macro2::Span::call_site(), "Missing `entity_name`"))?;

    Ok(Self {
      entity,
      entity_mod,
      create_req,
      update_req,
      apply_update,
      apply_soft_delete,
      before_create,
      before_update,
      before_soft_delete,
      before_execute,
      before_revert,
      response,
      entity_name,
      ops,
    })
  }
}

pub(crate) fn entity_service(attr: TokenStream, item: TokenStream) -> TokenStream {
  let args = parse_macro_input!(attr as EntityServiceArgs);
  let input_impl = parse_macro_input!(item as ItemImpl);

  if args.ops.is_empty() {
    return Error::new(
      proc_macro2::Span::call_site(),
      "Missing `ops(...)` for entity_service",
    )
    .to_compile_error()
    .into();
  }

  let self_ty = input_impl.self_ty.clone();
  let entity = args.entity;
  let entity_mod = args.entity_mod;
  let create_req = args.create_req;
  let update_req = args.update_req;
  let apply_update = args.apply_update;
  let apply_soft_delete = args.apply_soft_delete;
  let before_create = args.before_create;
  let before_update = args.before_update;
  let before_soft_delete = args.before_soft_delete;
  let before_execute = args.before_execute;
  let before_revert = args.before_revert;
  let response = args.response;
  let entity_name = args.entity_name;
  let ops = args.ops;
  let entity_name_value = entity.to_string();

  let create_name = format_ident!("{}_create", entity);
  let create_no_tx_name = format_ident!("{}_create_no_tx", entity);
  let list_name = format_ident!("{}_list", entity);
  let get_name = format_ident!("{}_get", entity);
  let update_name = format_ident!("{}_update", entity);
  let update_no_tx_name = format_ident!("{}_update_no_tx", entity);
  let soft_delete_name = format_ident!("{}_soft_delete", entity);
  let soft_delete_undo_name = format_ident!("{}_soft_delete_undo", entity);
  let soft_delete_set_state_name = format_ident!("{}_set_soft_deleted_state", entity);
  let hard_delete_name = format_ident!("{}_hard_delete", entity);
  let execute_name = format_ident!("{}_execute", entity);
  let execute_no_tx_name = format_ident!("{}_execute_no_tx", entity);
  let revert_name = format_ident!("{}_revert", entity);
  let revert_no_tx_name = format_ident!("{}_revert_no_tx", entity);
  let create_and_execute_name = format_ident!("{}_create_and_execute", entity);

  let mut methods = Vec::new();
  for op in ops {
    if op == "create" {
      let Some(create_req) = create_req.as_ref() else {
        return Error::new(
          proc_macro2::Span::call_site(),
          "`create` op requires `create_req = ...`",
        )
        .to_compile_error()
        .into();
      };
      let Some(response) = response.as_ref() else {
        return Error::new(
          proc_macro2::Span::call_site(),
          "`create` op requires `response = ...`",
        )
        .to_compile_error()
        .into();
      };

      let before_create_call = if let Some(before_create) = before_create.as_ref() {
        quote! {
          #before_create(self, conn, req).await?;
        }
      } else {
        quote! {}
      };

      methods.push(quote! {
        pub async fn #create_name(&self, req: &#create_req) -> Result<#response, crate::api::ApiError> {
          let txn = sea_orm::TransactionTrait::begin(self.db.as_ref()).await?;
          let saved = self.#create_no_tx_name(&txn, req).await?;
          txn.commit().await?;
          Ok(saved)
        }

        pub(crate) async fn #create_no_tx_name(
          &self,
          conn: &impl sea_orm::ConnectionTrait,
          req: &#create_req,
        ) -> Result<#response, crate::api::ApiError> {
          #before_create_call

          let saved = crate::services::common::insert_with_audit(
            conn,
            self.audit.as_ref(),
            #entity_mod::ActiveModel::from(req),
            |row| row.id,
          )
          .await?;
          Ok(saved.into())
        }
      });
    } else if op == "list" {
      let Some(response) = response.as_ref() else {
        return Error::new(
          proc_macro2::Span::call_site(),
          "`list` op requires `response = ...`",
        )
        .to_compile_error()
        .into();
      };
      methods.push(quote! {
        pub async fn #list_name(&self, pagination: Option<(u64, u64)>) -> Result<Vec<#response>, crate::api::ApiError> {
          use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
          let query = #entity_mod::Entity::find()
            .filter(#entity_mod::Column::DeletedAt.is_null());
          let rows = if let Some((page, per_page)) = pagination {
            use sea_orm::PaginatorTrait;
            query
              .paginate(self.db.as_ref(), per_page)
              .fetch_page(page.saturating_sub(1))
              .await?
          } else {
            query.all(self.db.as_ref()).await?
          };
          Ok(rows.into_iter().map(Into::into).collect())
        }
      });
    } else if op == "get" {
      let Some(response) = response.as_ref() else {
        return Error::new(
          proc_macro2::Span::call_site(),
          "`get` op requires `response = ...`",
        )
        .to_compile_error()
        .into();
      };
      methods.push(quote! {
        pub async fn #get_name(&self, id: uuid::Uuid) -> Result<#response, crate::api::ApiError> {
          let row = crate::services::common::get_active_by_id::<#entity_mod::Entity, _>(
            self.db.as_ref(),
            id,
            #entity_mod::Column::Id,
            #entity_mod::Column::DeletedAt,
            format!("{} '{}' not found", #entity_name, id),
          )
          .await?;
          Ok(row.into())
        }
      });
    } else if op == "update" {
      let Some(update_req) = update_req.as_ref() else {
        return Error::new(
          proc_macro2::Span::call_site(),
          "`update` op requires `update_req = ...`",
        )
        .to_compile_error()
        .into();
      };

      let Some(apply_update) = apply_update.as_ref() else {
        return Error::new(
          proc_macro2::Span::call_site(),
          "`update` op requires `apply_update = ...`",
        )
        .to_compile_error()
        .into();
      };
      let Some(response) = response.as_ref() else {
        return Error::new(
          proc_macro2::Span::call_site(),
          "`update` op requires `response = ...`",
        )
        .to_compile_error()
        .into();
      };

      let before_update_call = if let Some(before_update) = before_update.as_ref() {
        quote! {
          #before_update(self, conn, &existing, req).await?;
        }
      } else {
        quote! {}
      };

      methods.push(quote! {
        pub async fn #update_name(&self, id: uuid::Uuid, req: &#update_req) -> Result<#response, crate::api::ApiError> {
          let txn = sea_orm::TransactionTrait::begin(self.db.as_ref()).await?;
          let saved = self.#update_no_tx_name(&txn, id, req).await?;
          txn.commit().await?;
          Ok(saved)
        }

        pub(crate) async fn #update_no_tx_name(
          &self,
          conn: &impl sea_orm::ConnectionTrait,
          id: uuid::Uuid,
          req: &#update_req,
        ) -> Result<#response, crate::api::ApiError> {
          let existing: #entity_mod::Model = crate::services::common::get_active_by_id::<#entity_mod::Entity, _>(
            conn,
            id,
            #entity_mod::Column::Id,
            #entity_mod::Column::DeletedAt,
            format!("{} '{}' not found", #entity_name, id),
          )
          .await?;

          #before_update_call

          let mut model: #entity_mod::ActiveModel = existing.clone().into();
          #apply_update(&mut model, req);

          let saved = sea_orm::ActiveModelTrait::update(model, conn).await?;
          self
            .audit
            .register_update(conn, saved.id, &existing, &saved)
            .await?;
          Ok(saved.into())
        }
      });
    } else if op == "soft_delete" {
      let apply_soft_delete_call = if let Some(apply_soft_delete) = apply_soft_delete.as_ref() {
        quote! {
          #apply_soft_delete(&mut model, &existing, undo);
        }
      } else {
        quote! {}
      };

      let before_soft_delete_call = if let Some(before_soft_delete) = before_soft_delete.as_ref() {
        quote! {
          #before_soft_delete(self, &txn, &existing, undo).await?;
        }
      } else {
        quote! {}
      };

      methods.push(quote! {
        pub async fn #soft_delete_name(&self, id: uuid::Uuid) -> Result<(), crate::api::ApiError> {
          self.#soft_delete_set_state_name(id, false).await
        }

        pub async fn #soft_delete_undo_name(&self, id: uuid::Uuid) -> Result<(), crate::api::ApiError> {
          self.#soft_delete_set_state_name(id, true).await
        }

        async fn #soft_delete_set_state_name(&self, id: uuid::Uuid, undo: bool) -> Result<(), crate::api::ApiError> {
          let actor_id = crate::context::audit::current_actor_id().ok_or_else(|| {
            crate::api::ApiError::Unauthorized("Missing authenticated actor context".to_string())
          })?;

          let txn = sea_orm::TransactionTrait::begin(self.db.as_ref()).await?;
          let existing: #entity_mod::Model = crate::services::common::get_soft_delete_target_by_id::<#entity_mod::Entity, _>(
            &txn,
            id,
            #entity_mod::Column::Id,
            #entity_mod::Column::DeletedAt,
            undo,
            format!("{} '{}' not found", #entity_name, id),
          )
          .await?;

          #before_soft_delete_call

          let mut model: #entity_mod::ActiveModel = existing.clone().into();
          crate::services::common::set_soft_deleted_fields(
            &mut model.deleted_at,
            &mut model.deleted_by,
            undo,
            actor_id,
          );

          #apply_soft_delete_call

          let saved = sea_orm::ActiveModelTrait::update(model, &txn).await?;
          self
            .audit
            .register_update(&txn, saved.id, &existing, &saved)
            .await?;

          txn.commit().await?;
          Ok(())
        }
      });
    } else if op == "hard_delete" {
      methods.push(quote! {
        pub async fn #hard_delete_name(&self, id: uuid::Uuid) -> Result<(), crate::api::ApiError> {
          crate::services::common::hard_delete_with_audit::<#entity_mod::Entity>(
            self.db.as_ref(),
            self.audit.as_ref(),
            id,
            #entity_name_value,
            format!("{} '{}' not found", #entity_name, id),
          )
          .await
        }
      });
    } else if op == "create_and_execute" {
      let Some(create_req) = create_req.as_ref() else {
        return Error::new(
          proc_macro2::Span::call_site(),
          "`create_and_execute` op requires `create_req = ...`",
        )
        .to_compile_error()
        .into();
      };
      let Some(response) = response.as_ref() else {
        return Error::new(
          proc_macro2::Span::call_site(),
          "`create_and_execute` op requires `response = ...`",
        )
        .to_compile_error()
        .into();
      };

      let before_create_txn_call = if let Some(before_create) = before_create.as_ref() {
        quote! {
          #before_create(self, &txn, req).await?;
        }
      } else {
        quote! {}
      };

      methods.push(quote! {
        pub async fn #create_and_execute_name(
          &self,
          req: &#create_req,
          actor_id: uuid::Uuid,
        ) -> Result<#response, crate::api::ApiError> {
          let txn = sea_orm::TransactionTrait::begin(self.db.as_ref()).await?;

          #before_create_txn_call

          let created = crate::services::common::insert_with_audit(
            &txn,
            self.audit.as_ref(),
            #entity_mod::ActiveModel::from(req),
            |row| row.id,
          )
          .await?;

          self
            .#execute_no_tx_name(&txn, created.id, actor_id)
            .await?;

          let updated = crate::db::ops::find_by_id_required::<#entity_mod::Entity>(
            &txn,
            created.id,
            format!("{} '{}' not found", #entity_name, created.id),
          )
          .await?;

          txn.commit().await?;
          Ok(updated.into())
        }
      });
    } else if op == "execute" {
      let before_execute_call = if let Some(before_execute) = before_execute.as_ref() {
        quote! {
          #before_execute(self, conn, &existing, actor_id).await?;
        }
      } else {
        quote! {}
      };

      methods.push(quote! {
        pub async fn #execute_name(
          &self,
          document_id: uuid::Uuid,
          actor_id: uuid::Uuid,
        ) -> Result<(), crate::api::ApiError> {
          tracing::trace!(document_id = %document_id, "Executing {}", #entity_name);

          let txn = sea_orm::TransactionTrait::begin(self.db.as_ref()).await?;
          self.#execute_no_tx_name(&txn, document_id, actor_id).await?;
          txn.commit().await?;

          tracing::trace!(document_id = %document_id, "{} executed", #entity_name);
          Ok(())
        }

        pub(crate) async fn #execute_no_tx_name(
          &self,
          conn: &impl sea_orm::ConnectionTrait,
          document_id: uuid::Uuid,
          actor_id: uuid::Uuid,
        ) -> Result<(), crate::api::ApiError> {
          let existing = #entity_mod::Entity::find_by_id(document_id)
            .one(conn)
            .await?
            .ok_or_else(|| crate::api::ApiError::NotFound(format!("{} '{}' not found", #entity_name, document_id)))?;

          if existing.deleted_at.is_some() {
            return Err(crate::api::ApiError::Conflict(format!(
              "Soft-deleted {} cannot be executed",
              #entity_name
            )));
          }

          if existing.status == crate::enums::DocumentStatus::Posted {
            return Err(crate::api::ApiError::BadRequest(format!(
              "Attempted to execute already executed {}",
              #entity_name
            )));
          }

          #before_execute_call

          let mut active = existing.clone().into_active_model();
          active.status = sea_orm::ActiveValue::Set(crate::enums::DocumentStatus::Posted);
          active.executed_at = sea_orm::ActiveValue::Set(Some(chrono::Utc::now()));
          active.executed_by = sea_orm::ActiveValue::Set(Some(actor_id));
          active.reverted_at = sea_orm::ActiveValue::Set(None);
          active.reverted_by = sea_orm::ActiveValue::Set(None);

          let updated = sea_orm::ActiveModelTrait::update(active, conn).await?;

          self
            .audit
            .register_update(conn, updated.id, &existing, &updated)
            .await?;

          Ok(())
        }
      });
    } else if op == "revert" {
      let before_revert_call = if let Some(before_revert) = before_revert.as_ref() {
        quote! {
          #before_revert(self, conn, &existing, actor_id).await?;
        }
      } else {
        quote! {}
      };

      methods.push(quote! {
        pub async fn #revert_name(
          &self,
          document_id: uuid::Uuid,
          actor_id: uuid::Uuid,
        ) -> Result<(), crate::api::ApiError> {
          tracing::trace!(document_id = %document_id, "Reverting {}", #entity_name);

          let txn = sea_orm::TransactionTrait::begin(self.db.as_ref()).await?;
          self.#revert_no_tx_name(&txn, document_id, actor_id).await?;
          txn.commit().await?;

          tracing::trace!(document_id = %document_id, "{} reverted to draft", #entity_name);
          Ok(())
        }

        pub(crate) async fn #revert_no_tx_name(
          &self,
          conn: &impl sea_orm::ConnectionTrait,
          document_id: uuid::Uuid,
          actor_id: uuid::Uuid,
        ) -> Result<(), crate::api::ApiError> {
          let existing = #entity_mod::Entity::find_by_id(document_id)
            .one(conn)
            .await?
            .ok_or_else(|| crate::api::ApiError::NotFound(format!("{} '{}' not found", #entity_name, document_id)))?;

          if existing.deleted_at.is_some() {
            return Err(crate::api::ApiError::Conflict(format!(
              "Soft-deleted {} cannot be reverted",
              #entity_name
            )));
          }

          if existing.status == crate::enums::DocumentStatus::Draft {
            return Err(crate::api::ApiError::BadRequest(format!(
              "Attempted to revert non-executed {}",
              #entity_name
            )));
          }

          #before_revert_call

          let mut active = existing.clone().into_active_model();
          active.status = sea_orm::ActiveValue::Set(crate::enums::DocumentStatus::Draft);
          active.reverted_at = sea_orm::ActiveValue::Set(Some(chrono::Utc::now()));
          active.reverted_by = sea_orm::ActiveValue::Set(Some(actor_id));
          active.executed_at = sea_orm::ActiveValue::Set(None);
          active.executed_by = sea_orm::ActiveValue::Set(None);

          let updated = sea_orm::ActiveModelTrait::update(active, conn).await?;

          self
            .audit
            .register_update(conn, updated.id, &existing, &updated)
            .await?;

          Ok(())
        }
      });
    } else {
      return Error::new_spanned(
        op,
        "Unsupported op in entity_service; supported: create, list, get, update, soft_delete, hard_delete, create_and_execute, execute, revert",
      )
      .to_compile_error()
      .into();
    }
  }

  quote! {
    #input_impl

    impl #self_ty {
      #(#methods)*
    }
  }
  .into()
}
