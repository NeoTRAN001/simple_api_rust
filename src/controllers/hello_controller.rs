use rocket::http::Status;

#[get("/")]
pub fn index() -> Status {
    Status::Ok
}

#[get("/hello_world")]
pub fn hello_world() -> &'static str {
    "Hello world"
}