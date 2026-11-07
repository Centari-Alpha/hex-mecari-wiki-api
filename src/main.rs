use axum::{Json, Router, extract::Query, routing::get};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct BiomeInfo {
  pub id: &'static str,
  pub display_name: &'static str,
  pub movement_modifier_bp: u32,
  pub primary_resource: &'static str,
  pub description: &'static str,
}

#[derive(Serialize)]
pub struct BuildingInfo {
  pub id: &'static str,
  pub display_name: &'static str,
}

#[derive(Serialize)]
pub struct PathCostResult {
  pub biome: String,
  pub road_tier: u8,
  pub movement_modifier_bp: u32,
  pub road_multiplier_bp: u32,
  pub effective_speed_bp: u32,
}

#[derive(Deserialize)]
pub struct PathCostQuery {
  pub biome: String,
  pub road_tier: u8,
}

#[tokio::main]
async fn main() {
  let app = Router::new()
    .route("/api/biomes", get(get_biomes))
    .route("/api/buildings", get(get_buildings))
    .route("/api/pathcost", get(get_pathcost));

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

  println!("Hex Mecari Wiki API - listening on port 3001");
  axum::serve(listener, app).await.unwrap();
}

fn biome_catalog() -> Vec<BiomeInfo> {
  vec![
    BiomeInfo {
      id: "temperate_grassland",
      display_name: "Temperate Grassland & Woodlands",
      movement_modifier_bp: 10_000,
      primary_resource: "Timber",
      description: "Rich timber and fertile soil, Balanced Movement. No Gold Ore.",
    },
    BiomeInfo {
      id: "arid_badlands",
      display_name: "Arid Badlands",
      movement_modifier_bp: 8_500,
      primary_resource: "Gold Ore",
      description: "Sparse timber and crops. Dedicated Gold Ore nodes and stone canyons. Heat and dust penalites off-road.",
    },
    BiomeInfo {
      id: "alpine_peaks",
      display_name: "Alpine & Boreal Iron Peaks",
      movement_modifier_bp: 7_000,
      primary_resource: "Iron Ore",
      description: "Rich in Iron Ore, Coal and Deep Stone. Steep elevation requires engineered roads.",
    },
    BiomeInfo {
      id: "river_waterway",
      display_name: "River Waterway",
      movement_modifier_bp: 12_500,
      primary_resource: "Food Ration",
      description: "Navigable waterways with downstream barge speed bonus. Post-MVP expansion.",
    },
    BiomeInfo {
      id: "ocean",
      display_name: "Ocean",
      movement_modifier_bp: 11_000,
      primary_resource: "Food Ration",
      description: "Open water maritime travel. Post-MVP expansion",
    },
  ]
}

async fn get_biomes() -> Json<Vec<BiomeInfo>> {
  Json(biome_catalog())
}

async fn get_buildings() -> Json<Vec<BuildingInfo>> {
  Json(vec![
    BuildingInfo {
      id: "capital_building",
      display_name: "Capital Building",
    },
    BuildingInfo {
      id: "house",
      display_name: "House",
    },
    BuildingInfo {
      id: "storage_yard",
      display_name: "Storage Yard",
    },
    BuildingInfo {
      id: "lumber_camp",
      display_name: "Lumber Camp",
    },
    BuildingInfo {
      id: "sawmill",
      display_name: "Sawmill",
    },
    BuildingInfo {
      id: "farm",
      display_name: "Farm",
    },
    BuildingInfo {
      id: "quarry",
      display_name: "Quarry",
    },
    BuildingInfo {
      id: "mine",
      display_name: "Mine",
    },
    BuildingInfo {
      id: "smelter",
      display_name: "Smelter",
    },
    BuildingInfo {
      id: "mint",
      display_name: "Mint",
    },
    BuildingInfo {
      id: "tool_workshop",
      display_name: "Tool Workshop",
    },
    BuildingInfo {
      id: "trading_post",
      display_name: "Trading Post",
    },
    BuildingInfo {
      id: "bank_vault",
      display_name: "Bank Vault",
    },
    BuildingInfo {
      id: "barracks",
      display_name: "Barracks",
    },
    BuildingInfo {
      id: "wall",
      display_name: "Wall",
    },
    BuildingInfo {
      id: "grand_monument",
      display_name: "Grand Monument",
    },
  ])
}

async fn get_pathcost(Query(params): Query<PathCostQuery>) -> Json<PathCostResult> {
  let catalog = biome_catalog();
  let movement_modifier_bp = catalog
    .iter()
    .find(|b| b.id == params.biome.as_str())
    .map(|b| b.movement_modifier_bp)
    .unwrap_or(10_000);

  let road_multiplier_bp = match params.road_tier {
    0 => 10_000,
    1 => 12_500,
    2 => 20_000,
    3 => 35_000,
    _ => 10_000,
  };

  let effective_speed_bp = movement_modifier_bp * road_multiplier_bp / 10_000;

  Json(PathCostResult {
    biome: params.biome,
    road_tier: params.road_tier,
    movement_modifier_bp,
    road_multiplier_bp,
    effective_speed_bp,
  })
}
