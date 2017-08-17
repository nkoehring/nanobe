#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

mod nanobe;
use nanobe::frontend;
use nanobe::api;

fn main() {
  rocket::ignite()
  .mount("/", routes![frontend::index])
  .mount("/api", routes![api::index])
  .launch();
}
