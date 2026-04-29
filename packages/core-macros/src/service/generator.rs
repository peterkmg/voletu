use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Error, ItemImpl, Path, Result};

use super::args::{EntityServiceArgs, ServiceOp};

pub(super) fn expand_entity_service(
  args: EntityServiceArgs,
  input_impl: ItemImpl,
) -> Result<TokenStream> {
  if args.ops.is_empty() {
    return Err(Error::new(
      proc_macro2::Span::call_site(),
      "Missing `ops(...)` for entity_service",
    ));
  }

  let generator = EntityServiceGenerator::new(args, input_impl);
  generator.expand()
}

struct EntityServiceGenerator {
  args: EntityServiceArgs,
  input_impl: ItemImpl,
  names: MethodNames,
  entity_name_value: String,
}

impl EntityServiceGenerator {
  fn new(args: EntityServiceArgs, input_impl: ItemImpl) -> Self {
    let names = MethodNames::new(&args.entity);
    let entity_name_value = args.entity.to_string();
    Self {
      args,
      input_impl,
      names,
      entity_name_value,
    }
  }

  fn expand(&self) -> Result<TokenStream> {
    let self_ty = &self.input_impl.self_ty;
    let input_impl = &self.input_impl;
    let methods = self.generate_methods()?;

    Ok(quote! {
      #input_impl

      impl #self_ty {
        #(#methods)*
      }
    })
  }

  fn generate_methods(&self) -> Result<Vec<TokenStream>> {
    self
      .args
      .ops
      .iter()
      .map(|op| self.generate_method(*op))
      .collect()
  }

  fn generate_method(&self, op: ServiceOp) -> Result<TokenStream> {
    match op {
      ServiceOp::Create => self.generate_create_method(),
      ServiceOp::List => self.generate_list_method(),
      ServiceOp::Get => self.generate_get_method(),
      ServiceOp::Update => self.generate_update_method(),
      ServiceOp::SoftDelete => self.generate_soft_delete_method(),
      ServiceOp::HardDelete => self.generate_hard_delete_method(),
      ServiceOp::CreateAndExecute => self.generate_create_and_execute_method(),
      ServiceOp::Execute => self.generate_execute_method(),
      ServiceOp::Revert => self.generate_revert_method(),
    }
  }

  fn generate_create_method(&self) -> Result<TokenStream> {
    let create_req = self.require_create_req("create")?;
    let response = self.require_response("create")?;
    let entity_mod = &self.args.entity_mod;
    let create_name = &self.names.create_name;
    let create_no_tx_name = &self.names.create_no_tx_name;
    let before_create_call = if let Some(before_create) = self.args.before_create.as_ref() {
      quote! {
        #before_create(self, conn, req).await?;
      }
    } else {
      quote! {}
    };

    Ok(quote! {
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

        let saved = sea_orm::ActiveModelTrait::insert(
          #entity_mod::ActiveModel::from(req), conn,
        ).await?;
        Ok(saved.into())
      }
    })
  }

  fn generate_list_method(&self) -> Result<TokenStream> {
    let response = self.require_response("list")?;
    let entity_mod = &self.args.entity_mod;
    let list_name = &self.names.list_name;

    Ok(quote! {
      pub async fn #list_name(&self, pagination: Option<(u64, u64)>) -> Result<Vec<#response>, crate::api::ApiError> {
        use sea_orm::{ColumnTrait, EntityLoaderTrait, PaginatorTrait, QueryFilter};
        let query = #entity_mod::Entity::load()
          .filter(#entity_mod::Column::DeletedAt.is_null());
        let rows: Vec<#entity_mod::ModelEx> = if let Some((page, per_page)) = pagination {
          query
            .paginate(self.db.as_ref(), per_page)
            .fetch_page(page.saturating_sub(1))
            .await?
        } else {
          query.all(self.db.as_ref()).await?
        };
        Ok(rows.into_iter().map(#entity_mod::Model::from).map(Into::into).collect())
      }
    })
  }

  fn generate_get_method(&self) -> Result<TokenStream> {
    let response = self.require_response("get")?;
    let entity_mod = &self.args.entity_mod;
    let entity_name = &self.args.entity_name;
    let get_name = &self.names.get_name;

    Ok(quote! {
      pub async fn #get_name(&self, id: uuid::Uuid) -> Result<#response, crate::api::ApiError> {
        use sea_orm::{ColumnTrait, EntityLoaderTrait, QueryFilter};

        let row: #entity_mod::ModelEx = #entity_mod::Entity::load()
          .filter_by_id(id)
          .filter(#entity_mod::Column::DeletedAt.is_null())
          .one(self.db.as_ref())
          .await?
          .ok_or_else(|| crate::api::ApiError::NotFound(format!("{} '{}' not found", #entity_name, id)))?;

        Ok(#entity_mod::Model::from(row).into())
      }
    })
  }

  fn generate_update_method(&self) -> Result<TokenStream> {
    let update_req = self.require_update_req("update")?;
    let apply_update = self.require_apply_update("update")?;
    let response = self.require_response("update")?;
    let entity_mod = &self.args.entity_mod;
    let entity_name = &self.args.entity_name;
    let update_name = &self.names.update_name;
    let update_no_tx_name = &self.names.update_no_tx_name;
    let before_update_call = if let Some(before_update) = self.args.before_update.as_ref() {
      quote! {
        #before_update(self, conn, &existing, req).await?;
      }
    } else {
      quote! {}
    };

    Ok(quote! {
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
        use sea_orm::{ColumnTrait, EntityLoaderTrait, QueryFilter};

        let existing_loaded: #entity_mod::ModelEx = #entity_mod::Entity::load()
          .filter_by_id(id)
          .filter(#entity_mod::Column::DeletedAt.is_null())
          .one(conn)
          .await?
          .ok_or_else(|| crate::api::ApiError::NotFound(format!("{} '{}' not found", #entity_name, id)))?;
        let existing: #entity_mod::Model = #entity_mod::Model::from(existing_loaded);

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
    })
  }

  fn generate_soft_delete_method(&self) -> Result<TokenStream> {
    let entity_mod = &self.args.entity_mod;
    let entity_name = &self.args.entity_name;
    let soft_delete_name = &self.names.soft_delete_name;
    let soft_delete_undo_name = &self.names.soft_delete_undo_name;
    let soft_delete_set_state_name = &self.names.soft_delete_set_state_name;
    let apply_soft_delete_call =
      if let Some(apply_soft_delete) = self.args.apply_soft_delete.as_ref() {
        quote! {
          #apply_soft_delete(&mut model, &existing, undo);
        }
      } else {
        quote! {}
      };
    let before_soft_delete_call =
      if let Some(before_soft_delete) = self.args.before_soft_delete.as_ref() {
        quote! {
          #before_soft_delete(self, &txn, &existing, undo).await?;
        }
      } else {
        quote! {}
      };

    Ok(quote! {
      pub async fn #soft_delete_name(&self, id: uuid::Uuid) -> Result<(), crate::api::ApiError> {
        self.#soft_delete_set_state_name(id, false).await
      }

      pub async fn #soft_delete_undo_name(&self, id: uuid::Uuid) -> Result<(), crate::api::ApiError> {
        self.#soft_delete_set_state_name(id, true).await
      }

      async fn #soft_delete_set_state_name(&self, id: uuid::Uuid, undo: bool) -> Result<(), crate::api::ApiError> {
        use sea_orm::{ColumnTrait, EntityLoaderTrait, QueryFilter};

        let actor_id = crate::context::audit::current_actor_id().ok_or_else(|| {
          crate::api::ApiError::Unauthorized("Missing authenticated actor context".to_string())
        })?;

        let txn = sea_orm::TransactionTrait::begin(self.db.as_ref()).await?;
        let deleted_filter = if undo {
          #entity_mod::Column::DeletedAt.is_not_null()
        } else {
          #entity_mod::Column::DeletedAt.is_null()
        };
        let existing_loaded: #entity_mod::ModelEx = #entity_mod::Entity::load()
          .filter_by_id(id)
          .filter(deleted_filter)
          .one(&txn)
          .await?
          .ok_or_else(|| crate::api::ApiError::NotFound(format!("{} '{}' not found", #entity_name, id)))?;
        let existing: #entity_mod::Model = #entity_mod::Model::from(existing_loaded);

        #before_soft_delete_call

        let mut model: #entity_mod::ActiveModel = existing.clone().into();
        let now = sea_orm::prelude::ChronoUtc::now();
        model.deleted_at = sea_orm::ActiveValue::Set(if undo { None } else { Some(now) });
        model.deleted_by = sea_orm::ActiveValue::Set(if undo { None } else { Some(actor_id) });

        #apply_soft_delete_call

        let saved = sea_orm::ActiveModelTrait::update(model, &txn).await?;
        self
          .audit
          .register_update(&txn, saved.id, &existing, &saved)
          .await?;

        txn.commit().await?;
        Ok(())
      }
    })
  }

  fn generate_hard_delete_method(&self) -> Result<TokenStream> {
    let entity_mod = &self.args.entity_mod;
    let entity_name = &self.args.entity_name;
    let entity_name_value = &self.entity_name_value;
    let hard_delete_name = &self.names.hard_delete_name;

    Ok(quote! {
      pub async fn #hard_delete_name(&self, id: uuid::Uuid) -> Result<(), crate::api::ApiError> {
        use sea_orm::{EntityLoaderTrait, EntityTrait, TransactionTrait};

        let txn = sea_orm::TransactionTrait::begin(self.db.as_ref()).await?;
        let existing_loaded: #entity_mod::ModelEx = #entity_mod::Entity::load()
          .filter_by_id(id)
          .one(&txn)
          .await?
          .ok_or_else(|| crate::api::ApiError::NotFound(format!("{} '{}' not found", #entity_name, id)))?;
        let existing: #entity_mod::Model = #entity_mod::Model::from(existing_loaded);

        #entity_mod::Entity::delete_by_id(id)
          .exec(&txn)
          .await
          .map_err(|err| {
            let message = err.to_string().to_lowercase();
            let has_dependency_violation = message.contains("foreign key")
              || message.contains("constraint failed")
              || message.contains("violates foreign key constraint")
              || message.contains("violates constraint");

            if has_dependency_violation {
              crate::api::ApiError::Conflict(format!(
                "Cannot hard delete {} because dependent records exist",
                #entity_name_value
              ))
            } else {
              crate::api::ApiError::Database(err)
            }
          })?;

        self.audit.register_delete(&txn, id, &existing).await?;
        txn.commit().await?;
        Ok(())
      }
    })
  }

  fn generate_create_and_execute_method(&self) -> Result<TokenStream> {
    let create_req = self.require_create_req("create_and_execute")?;
    let response = self.require_response("create_and_execute")?;
    let entity_mod = &self.args.entity_mod;
    let entity_name = &self.args.entity_name;
    let create_and_execute_name = &self.names.create_and_execute_name;
    let execute_no_tx_name = &self.names.execute_no_tx_name;
    let before_create_txn_call = if let Some(before_create) = self.args.before_create.as_ref() {
      quote! {
        #before_create(self, &txn, req).await?;
      }
    } else {
      quote! {}
    };

    Ok(quote! {
      pub async fn #create_and_execute_name(
        &self,
        req: &#create_req,
        actor_id: uuid::Uuid,
      ) -> Result<#response, crate::api::ApiError> {
        use sea_orm::EntityLoaderTrait;

        let txn = sea_orm::TransactionTrait::begin(self.db.as_ref()).await?;

        #before_create_txn_call

        let created = sea_orm::ActiveModelTrait::insert(
          #entity_mod::ActiveModel::from(req), &txn,
        ).await?;

        self
          .#execute_no_tx_name(&txn, created.id, actor_id)
          .await?;

        let updated_loaded: #entity_mod::ModelEx = #entity_mod::Entity::load()
          .filter_by_id(created.id)
          .one(&txn)
          .await?
          .ok_or_else(|| crate::api::ApiError::NotFound(format!("{} '{}' not found", #entity_name, created.id)))?;
        let updated: #entity_mod::Model = #entity_mod::Model::from(updated_loaded);

        txn.commit().await?;
        Ok(updated.into())
      }
    })
  }

  fn generate_execute_method(&self) -> Result<TokenStream> {
    let entity_mod = &self.args.entity_mod;
    let entity_name = &self.args.entity_name;
    let execute_name = &self.names.execute_name;
    let execute_no_tx_name = &self.names.execute_no_tx_name;
    let before_execute_call = if let Some(before_execute) = self.args.before_execute.as_ref() {
      quote! {
        #before_execute(self, conn, &existing, actor_id).await?;
      }
    } else {
      quote! {}
    };

    Ok(quote! {
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
        use sea_orm::EntityLoaderTrait;

        let existing_loaded: #entity_mod::ModelEx = #entity_mod::Entity::load()
          .filter_by_id(document_id)
          .one(conn)
          .await?
          .ok_or_else(|| crate::api::ApiError::NotFound(format!("{} '{}' not found", #entity_name, document_id)))?;
        let existing: #entity_mod::Model = #entity_mod::Model::from(existing_loaded);

        if existing.deleted_at.is_some() {
          return Err(crate::api::ApiError::Conflict(format!(
            "Soft-deleted {} cannot be executed",
            #entity_name
          )));
        }

        if existing.status == crate::enums::DocumentStatus::Executed {
          return Err(crate::api::ApiError::BadRequest(format!(
            "Attempted to execute already executed {}",
            #entity_name
          )));
        }

        #before_execute_call

        let mut active = existing.clone().into_active_model();
        active.status = sea_orm::ActiveValue::Set(crate::enums::DocumentStatus::Executed);
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
    })
  }

  fn generate_revert_method(&self) -> Result<TokenStream> {
    let entity_mod = &self.args.entity_mod;
    let entity_name = &self.args.entity_name;
    let revert_name = &self.names.revert_name;
    let revert_no_tx_name = &self.names.revert_no_tx_name;
    let before_revert_call = if let Some(before_revert) = self.args.before_revert.as_ref() {
      quote! {
        #before_revert(self, conn, &existing, actor_id).await?;
      }
    } else {
      quote! {}
    };

    Ok(quote! {
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
        use sea_orm::EntityLoaderTrait;

        let existing_loaded: #entity_mod::ModelEx = #entity_mod::Entity::load()
          .filter_by_id(document_id)
          .one(conn)
          .await?
          .ok_or_else(|| crate::api::ApiError::NotFound(format!("{} '{}' not found", #entity_name, document_id)))?;
        let existing: #entity_mod::Model = #entity_mod::Model::from(existing_loaded);

        if existing.deleted_at.is_some() {
          return Err(crate::api::ApiError::Conflict(format!(
            "Soft-deleted {} cannot be reverted",
            #entity_name
          )));
        }

        if existing.status == crate::enums::DocumentStatus::Draft {
          if existing.reverted_at.is_some() {
            tracing::trace!(document_id = %document_id, "{} already reverted to draft", #entity_name);
            return Ok(());
          }

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
    })
  }

  fn require_create_req(&self, op_name: &str) -> Result<&Path> {
    self
      .args
      .create_req
      .as_ref()
      .ok_or_else(|| missing_required_arg(op_name, "create_req"))
  }

  fn require_update_req(&self, op_name: &str) -> Result<&Path> {
    self
      .args
      .update_req
      .as_ref()
      .ok_or_else(|| missing_required_arg(op_name, "update_req"))
  }

  fn require_apply_update(&self, op_name: &str) -> Result<&Path> {
    self
      .args
      .apply_update
      .as_ref()
      .ok_or_else(|| missing_required_arg(op_name, "apply_update"))
  }

  fn require_response(&self, op_name: &str) -> Result<&Path> {
    self
      .args
      .response
      .as_ref()
      .ok_or_else(|| missing_required_arg(op_name, "response"))
  }
}

struct MethodNames {
  create_name: syn::Ident,
  create_no_tx_name: syn::Ident,
  list_name: syn::Ident,
  get_name: syn::Ident,
  update_name: syn::Ident,
  update_no_tx_name: syn::Ident,
  soft_delete_name: syn::Ident,
  soft_delete_undo_name: syn::Ident,
  soft_delete_set_state_name: syn::Ident,
  hard_delete_name: syn::Ident,
  execute_name: syn::Ident,
  execute_no_tx_name: syn::Ident,
  revert_name: syn::Ident,
  revert_no_tx_name: syn::Ident,
  create_and_execute_name: syn::Ident,
}

impl MethodNames {
  fn new(entity: &syn::Ident) -> Self {
    Self {
      create_name: format_ident!("{}_create", entity),
      create_no_tx_name: format_ident!("{}_create_no_tx", entity),
      list_name: format_ident!("{}_list", entity),
      get_name: format_ident!("{}_get", entity),
      update_name: format_ident!("{}_update", entity),
      update_no_tx_name: format_ident!("{}_update_no_tx", entity),
      soft_delete_name: format_ident!("{}_soft_delete", entity),
      soft_delete_undo_name: format_ident!("{}_soft_delete_undo", entity),
      soft_delete_set_state_name: format_ident!("{}_set_soft_deleted_state", entity),
      hard_delete_name: format_ident!("{}_hard_delete", entity),
      execute_name: format_ident!("{}_execute", entity),
      execute_no_tx_name: format_ident!("{}_execute_no_tx", entity),
      revert_name: format_ident!("{}_revert", entity),
      revert_no_tx_name: format_ident!("{}_revert_no_tx", entity),
      create_and_execute_name: format_ident!("{}_create_and_execute", entity),
    }
  }
}

fn missing_required_arg(op_name: &str, arg_name: &str) -> Error {
  Error::new(
    proc_macro2::Span::call_site(),
    format!("`{op_name}` op requires `{arg_name} = ...`"),
  )
}
