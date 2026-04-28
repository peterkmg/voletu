use voletu_core::endpoints::paths as api_paths;

use crate::common::{
  catalog_seed::seed_inventory_catalog,
  http::{assert_api_success, get, post_json, setup_seeded_app_with_admin_token, with_auth_token},
  payloads::{
    transport_rail_manifest,
    transport_rail_measurement,
    transport_rail_waybill,
    transport_rail_weight,
    transport_truck_item,
    transport_truck_waybill,
    transport_truck_weight_doc,
  },
};

#[tokio::test]
async fn composite_endpoints_return_nested_rows_and_names_with_embed_param() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let catalog = seed_inventory_catalog(&db).await;

  with_auth_token(token, async {
    let truck_waybill_res = post_json(
      &app,
      api_paths::transport::truck::WAYBILLS,
      transport_truck_waybill(
        "TW-COMPOSITE-NAMES-1",
        "2026-01-04",
        catalog.sender_id,
        catalog.base_id,
      ),
    )
    .await;
    let truck_waybill_json = assert_api_success(truck_waybill_res).await;
    let truck_waybill_id = truck_waybill_json["data"]["id"]
      .as_str()
      .unwrap()
      .to_owned();

    let truck_item_res = post_json(
      &app,
      api_paths::transport::truck::ITEMS,
      transport_truck_item(
        uuid::Uuid::parse_str(&truck_waybill_id).unwrap(),
        catalog.product_a_id,
        "12.5",
      ),
    )
    .await;
    assert_api_success(truck_item_res).await;

    let truck_weight_res = post_json(
      &app,
      api_paths::transport::truck::WEIGHT_DOCS,
      transport_truck_weight_doc(uuid::Uuid::parse_str(&truck_waybill_id).unwrap(), "13.0"),
    )
    .await;
    assert_api_success(truck_weight_res).await;

    let truck_composite_json = assert_api_success(
      get(
        &app,
        api_paths::transport::truck::COMPOSITE_BY_ID.replace("{id}", &truck_waybill_id)
          + "?embed=names",
      )
      .await,
    )
    .await;
    assert_eq!(
      truck_composite_json["data"]["waybill"]["senderIdName"],
      "Sender Co"
    );
    assert_eq!(
      truck_composite_json["data"]["items"][0]["productIdName"],
      "Product A"
    );
    assert_eq!(
      truck_composite_json["data"]["weightDocs"]
        .as_array()
        .unwrap()
        .len(),
      1
    );
    assert_eq!(
      truck_composite_json["data"]["weightDocs"][0]["totalWeight"],
      "13"
    );

    let rail_waybill_res = post_json(
      &app,
      api_paths::transport::rail::WAYBILLS,
      transport_rail_waybill(
        "RW-COMPOSITE-NAMES-1",
        "2026-01-04",
        catalog.sender_id,
        catalog.base_id,
      ),
    )
    .await;
    let rail_waybill_json = assert_api_success(rail_waybill_res).await;
    let rail_waybill_id = rail_waybill_json["data"]["id"].as_str().unwrap().to_owned();

    let manifest_res = post_json(
      &app,
      api_paths::transport::rail::MANIFESTS,
      transport_rail_manifest(
        uuid::Uuid::parse_str(&rail_waybill_id).unwrap(),
        "WG-C1",
        catalog.product_a_id,
        "20.0",
        "0.8",
        "16.0",
      ),
    )
    .await;
    let manifest_json = assert_api_success(manifest_res).await;
    let manifest_id = manifest_json["data"]["id"].as_str().unwrap().to_owned();

    let measurement_res = post_json(
      &app,
      api_paths::transport::rail::MEASUREMENTS,
      transport_rail_measurement(
        uuid::Uuid::parse_str(&manifest_id).unwrap(),
        "2.0",
        "0.79",
        "15.8",
      ),
    )
    .await;
    assert_api_success(measurement_res).await;

    let weight_res = post_json(
      &app,
      api_paths::transport::rail::WEIGHTS,
      transport_rail_weight(
        uuid::Uuid::parse_str(&manifest_id).unwrap(),
        "40.0",
        "20.0",
        "20.0",
      ),
    )
    .await;
    assert_api_success(weight_res).await;

    let rail_composite_json = assert_api_success(
      get(
        &app,
        api_paths::transport::rail::COMPOSITE_BY_ID.replace("{id}", &rail_waybill_id)
          + "?embed=names",
      )
      .await,
    )
    .await;
    assert_eq!(
      rail_composite_json["data"]["waybill"]["senderIdName"],
      "Sender Co"
    );
    assert_eq!(
      rail_composite_json["data"]["wagonManifests"][0]["productIdName"],
      "Product A"
    );
    assert_eq!(
      rail_composite_json["data"]["wagonManifests"][0]["measurements"]
        .as_array()
        .unwrap()
        .len(),
      1
    );
    assert_eq!(
      rail_composite_json["data"]["wagonManifests"][0]["weights"]
        .as_array()
        .unwrap()
        .len(),
      1
    );
  })
  .await;
}

#[tokio::test]
async fn waybill_header_endpoints_return_sender_names_with_embed_param() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let catalog = seed_inventory_catalog(&db).await;

  with_auth_token(token, async {
    let truck_create = post_json(
      &app,
      api_paths::transport::truck::WAYBILLS,
      transport_truck_waybill(
        "TW-NAMES-1",
        "2026-01-03",
        catalog.sender_id,
        catalog.base_id,
      ),
    )
    .await;
    let truck_create_json = assert_api_success(truck_create).await;
    let truck_id = truck_create_json["data"]["id"].as_str().unwrap().to_owned();

    let truck_list_json = assert_api_success(
      get(
        &app,
        format!("{}?embed=names", api_paths::transport::truck::WAYBILLS),
      )
      .await,
    )
    .await;
    let truck_list_item = truck_list_json["data"]
      .as_array()
      .unwrap()
      .iter()
      .find(|item| item["id"].as_str() == Some(truck_id.as_str()))
      .unwrap();
    assert_eq!(truck_list_item["senderIdName"], "Sender Co");

    let truck_query_json = assert_api_success(
      get(
        &app,
        format!(
          "{}?documentNumber=TW-NAMES-1&embed=names",
          api_paths::transport::truck::WAYBILLS_QUERY
        ),
      )
      .await,
    )
    .await;
    assert_eq!(truck_query_json["data"].as_array().unwrap().len(), 1);
    assert_eq!(truck_query_json["data"][0]["senderIdName"], "Sender Co");

    let truck_get_json = assert_api_success(
      get(
        &app,
        format!(
          "{}/{}?embed=names",
          api_paths::transport::truck::WAYBILLS,
          truck_id
        ),
      )
      .await,
    )
    .await;
    assert_eq!(truck_get_json["data"]["senderIdName"], "Sender Co");

    let rail_create = post_json(
      &app,
      api_paths::transport::rail::WAYBILLS,
      transport_rail_waybill(
        "RW-NAMES-1",
        "2026-01-03",
        catalog.sender_id,
        catalog.base_id,
      ),
    )
    .await;
    let rail_create_json = assert_api_success(rail_create).await;
    let rail_id = rail_create_json["data"]["id"].as_str().unwrap().to_owned();

    let rail_list_json = assert_api_success(
      get(
        &app,
        format!("{}?embed=names", api_paths::transport::rail::WAYBILLS),
      )
      .await,
    )
    .await;
    let rail_list_item = rail_list_json["data"]
      .as_array()
      .unwrap()
      .iter()
      .find(|item| item["id"].as_str() == Some(rail_id.as_str()))
      .unwrap();
    assert_eq!(rail_list_item["senderIdName"], "Sender Co");

    let rail_query_json = assert_api_success(
      get(
        &app,
        format!(
          "{}?documentNumber=RW-NAMES-1&embed=names",
          api_paths::transport::rail::WAYBILLS_QUERY
        ),
      )
      .await,
    )
    .await;
    assert_eq!(rail_query_json["data"].as_array().unwrap().len(), 1);
    assert_eq!(rail_query_json["data"][0]["senderIdName"], "Sender Co");

    let rail_get_json = assert_api_success(
      get(
        &app,
        format!(
          "{}/{}?embed=names",
          api_paths::transport::rail::WAYBILLS,
          rail_id
        ),
      )
      .await,
    )
    .await;
    assert_eq!(rail_get_json["data"]["senderIdName"], "Sender Co");
  })
  .await;
}
