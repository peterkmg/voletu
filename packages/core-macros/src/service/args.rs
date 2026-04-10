use syn::{
  parenthesized,
  parse::{Parse, ParseStream},
  Error,
  Ident,
  LitStr,
  Path,
  Result,
  Token,
};

pub(super) struct EntityServiceArgs {
  pub(super) entity: Ident,
  pub(super) entity_mod: Path,
  pub(super) create_req: Option<Path>,
  pub(super) update_req: Option<Path>,
  pub(super) apply_update: Option<Path>,
  pub(super) apply_soft_delete: Option<Path>,
  pub(super) before_create: Option<Path>,
  pub(super) before_update: Option<Path>,
  pub(super) before_soft_delete: Option<Path>,
  pub(super) before_execute: Option<Path>,
  pub(super) before_revert: Option<Path>,
  pub(super) response: Option<Path>,
  pub(super) entity_name: LitStr,
  pub(super) ops: Vec<ServiceOp>,
}

#[derive(Clone, Copy)]
pub(super) enum ServiceOp {
  Create,
  List,
  Get,
  Update,
  SoftDelete,
  HardDelete,
  CreateAndExecute,
  Execute,
  Revert,
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
          ops.push(content.parse()?);
          if content.peek(Token![,]) {
            let _: Token![,] = content.parse()?;
          }
        }
      } else {
        let _: Token![=] = input.parse()?;
        match key.to_string().as_str() {
          "entity" => entity = Some(input.parse()?),
          "entity_mod" => entity_mod = Some(input.parse()?),
          "create_req" => create_req = Some(input.parse()?),
          "update_req" => update_req = Some(input.parse()?),
          "apply_update" => apply_update = Some(input.parse()?),
          "apply_soft_delete" => apply_soft_delete = Some(input.parse()?),
          "before_create" => before_create = Some(input.parse()?),
          "before_update" => before_update = Some(input.parse()?),
          "before_soft_delete" => before_soft_delete = Some(input.parse()?),
          "before_execute" => before_execute = Some(input.parse()?),
          "before_revert" => before_revert = Some(input.parse()?),
          "response" => response = Some(input.parse()?),
          "entity_name" => entity_name = Some(input.parse()?),
          _ => return Err(Error::new_spanned(key, "Unknown entity_service argument")),
        }
      }

      if input.peek(Token![,]) {
        let _: Token![,] = input.parse()?;
      }
    }

    Ok(Self {
      entity: entity.ok_or_else(missing_entity)?,
      entity_mod: entity_mod.ok_or_else(missing_entity_mod)?,
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
      entity_name: entity_name.ok_or_else(missing_entity_name)?,
      ops,
    })
  }
}

impl Parse for ServiceOp {
  fn parse(input: ParseStream) -> Result<Self> {
    let op: Ident = input.parse()?;
    match op.to_string().as_str() {
      "create" => Ok(Self::Create),
      "list" => Ok(Self::List),
      "get" => Ok(Self::Get),
      "update" => Ok(Self::Update),
      "soft_delete" => Ok(Self::SoftDelete),
      "hard_delete" => Ok(Self::HardDelete),
      "create_and_execute" => Ok(Self::CreateAndExecute),
      "execute" => Ok(Self::Execute),
      "revert" => Ok(Self::Revert),
      _ => Err(Error::new_spanned(
        op,
        "Unsupported op in entity_service; supported: create, list, get, update, soft_delete, hard_delete, create_and_execute, execute, revert",
      )),
    }
  }
}

fn missing_entity() -> Error {
  Error::new(proc_macro2::Span::call_site(), "Missing `entity`")
}

fn missing_entity_mod() -> Error {
  Error::new(proc_macro2::Span::call_site(), "Missing `entity_mod`")
}

fn missing_entity_name() -> Error {
  Error::new(proc_macro2::Span::call_site(), "Missing `entity_name`")
}
