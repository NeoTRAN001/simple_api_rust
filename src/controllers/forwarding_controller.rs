use rocket::http::Status;

#[get("/<name>/<age>")]
pub fn index(name: &str, age: u8) -> Status{
    if name == "" {
        return Status::NotFound
    }

    if age <= 0 {
        return Status::NotFound
    }

    return Status::Ok
}