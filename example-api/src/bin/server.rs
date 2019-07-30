use example_api::{Api, server};

// Serve the API.
//
// Navigate your browser to http://localhost:8000/ui.html to see
// the API as rendered by [Swagger UI](https://github.com/swagger-api/swagger-ui)
fn main() -> Result<(), std::io::Error> {
    env_logger::init();
    let uri = "127.0.0.1:8000".parse().unwrap();
    server::serve::<Api>(uri)
}
