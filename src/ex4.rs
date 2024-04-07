use axum::{extract, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Reindeer {
    name: String,
    strength: u64,
}

#[derive(Deserialize, Debug)]
struct ContestReindeer {
    name: String,
    strength: u64,
    speed: f64,
    height: u64,
    antler_width: u64,
    snow_magic_power: u64,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: u64,
}

#[derive(Debug, Serialize)]
struct ContestResult {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

async fn calculate_strength(
    extract::Json(reindeers): extract::Json<Vec<Reindeer>>,
) -> impl IntoResponse {
    reindeers
        .iter()
        .fold(0u64, |sum, reindeer| sum + reindeer.strength)
        .to_string()
}

async fn calculate_contest(
    extract::Json(reindeers): extract::Json<Vec<ContestReindeer>>,
) -> Result<Json<ContestResult>, StatusCode> {
    if reindeers.len() < 1 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut fastest = &reindeers[0];
    let mut tallest = &reindeers[0];
    let mut magician = &reindeers[0];
    let mut consumer = &reindeers[0];

    for reindeer in reindeers.iter() {
        if reindeer.speed > fastest.speed {
            fastest = &reindeer;
        }

        if reindeer.height > tallest.height {
            tallest = &reindeer;
        }

        if reindeer.snow_magic_power > magician.snow_magic_power {
            magician = &reindeer;
        }

        if reindeer.candies_eaten_yesterday > consumer.candies_eaten_yesterday {
            consumer = &reindeer;
        }
    }

    Ok(Json(ContestResult {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food
        ),
    }))
}

pub fn router() -> Router {
    Router::new()
        .route("/strength", post(calculate_strength))
        .route("/contest", post(calculate_contest))
}
