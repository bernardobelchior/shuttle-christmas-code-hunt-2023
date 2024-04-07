use std::cmp::min;

use axum::{
    extract,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct QueryParams {
    #[serde(default = "usize::min_value")]
    offset: usize,
    #[serde(default)]
    limit: Option<usize>,
    #[serde(default)]
    split: Option<usize>,
}

async fn slice(
    extract::Query(query_params): extract::Query<QueryParams>,
    extract::Json(names): extract::Json<Vec<String>>,
) -> Response {
    let QueryParams {
        offset,
        limit: limit_opt,
        split,
    } = query_params;

    if offset >= names.len() {
        return Json(Vec::<String>::new()).into_response();
    }

    let limit = limit_opt.unwrap_or(names.len());
    let end = min(names.len(), offset + limit);

    match split {
        None => Json(names[offset..end].to_owned()).into_response(),
        Some(chunk_size) => Json(
            names[offset..end]
                .chunks(chunk_size)
                .collect::<Vec<&[String]>>(),
        )
        .into_response(),
    }
}

pub fn router() -> Router {
    Router::new().route("/", post(slice))
}
