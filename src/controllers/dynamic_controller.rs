use rocket::serde::{json::Json};
use rocket::http::Status;

use crate::routes_dtos::task_dto::Task;

#[get("/<name>")]
pub fn dynamic_path(name: &str) -> (Status, Json<Task>) {
    (Status::Accepted, Json(Task {
        name: name.to_string()
    }))
}