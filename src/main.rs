use axum::{extract::Path, http::StatusCode, routing::get, Router};

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

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(internal_server_error))
        .route("/1/*nums", get(recalibrate_multiple_packet_ids));

    Ok(router.into())
}
