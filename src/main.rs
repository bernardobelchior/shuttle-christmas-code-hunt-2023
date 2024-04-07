use axum::{
    extract::{self, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn internal_server_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn recalibrate_multiple_packet_ids(Path(nums): Path<String>) -> Result<String, StatusCode> {
    let nums_iter = nums.split('/').into_iter();
    let mut xor = 0;

    for num_str in nums_iter {
        let num = num_str
            .parse::<i64>()
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        xor = xor ^ num;
    }

    Ok(xor.pow(3).to_string())
}

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

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(internal_server_error))
        .route("/1/*nums", get(recalibrate_multiple_packet_ids))
        .route("/4/strength", post(calculate_strength))
        .route("/4/contest", post(calculate_contest));

    Ok(router.into())
}
