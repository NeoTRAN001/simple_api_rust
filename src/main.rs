#[macro_use] extern crate rocket;

#[path= "dto/routes_dtos.rs"]
mod routes_dtos;

#[path= "controllers/routes_controllers.rs"]
mod routes_controllers;

use routes_controllers::dynamic_controller as dynamic;
use routes_controllers::hello_controller as hello;
use routes_controllers::forwarding_controller as forwarding;


#[launch()]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello::index, hello::hello_world])
        .mount("/dynamic", routes![dynamic::dynamic_path])
        .mount("/forwarding", routes![forwarding::index])
}
