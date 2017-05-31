extern crate pencil;
use std::collections::BTreeMap;
use pencil::{Pencil, Request, Response, PencilResult, jsonify};

fn app_info(_: &mut Request) -> PencilResult {
    let mut d = BTreeMap::new();
    d.insert("name", "nanobe");
    d.insert("version", "0.1.0");
    return jsonify(&d);
}

fn hello(_: &mut Request) -> PencilResult {
    Ok(Response::from("Hello World!"))
}

fn main() {
    let mut app = Pencil::new("/nanobe");
    app.get("/info", "app_info", app_info);
    app.get("/", "hello", hello);
    app.run("127.0.0.1:8000");
}
