#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use rocket::{get, Build, Rocket};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi, openapi_get_routes, JsonSchema};
use serde_json::Value;

#[derive(serde::Serialize, JsonSchema)]
struct ErrorResponse {
    code: Option<i32>,
    description: String,
}

#[derive(serde::Serialize, JsonSchema)]
struct Response {
    /// The reply.
    reply: String,
    error: Option<ErrorResponse>,
    // TODO how to deserialize an object into either a typed structure or a Value?
    arbitrary: Value,
}

#[openapi]
#[get("/")]
async fn route() -> Json<Response> {
    Json(Response {
        reply: "show me the docs!".to_string(),
        error: Some(ErrorResponse {
            code: Some(0),
            description: "not enough cowbell".to_string(),
        }),
        arbitrary: Value::Null,
    })
}

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/api/openapi.json".to_string(),
        ..Default::default()
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    panic!("oop");
    rocket::build()
        .mount("/api", openapi_get_routes![route])
        .mount("/swagger", make_swagger_ui(&get_docs()))
}
