use axum::{extract::Path, http::StatusCode, routing::get, Router};

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

pub fn router() -> Router {
    Router::new().route("/*nums", get(recalibrate_multiple_packet_ids))
}
