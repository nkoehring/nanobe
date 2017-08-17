#[get("/")]
pub fn index() -> &'static str {
    "Hello, api!"
}

