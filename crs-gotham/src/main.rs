#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate lazy_static;

use gotham::state::State;
use rbatis::rbatis::Rbatis;

pub mod errors;
pub mod routes;
pub mod service;
pub mod utils;

lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
}

/// Create a `Handler` which is invoked when responding to a `Request`.
///
/// How does a function become a `Handler`?.
/// We've simply implemented the `Handler` trait, for functions that match the signature used here,
/// within Gotham itself.
pub fn say_hello(state: State) -> (State, &'static str) {
    (state, "Hello World!")
}

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn main() {
    dotenv::dotenv().unwrap();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    println!("Init db connection(rbatis).[link]({})", database_url);

    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(say_hello))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;
    use http::status::StatusCode;

    #[test]
    fn receive_hello_world_response() {
        let test_server = TestServer::new(|| Ok(say_hello)).unwrap();
        let response = test_server
            .client()
            .get("http://localhost")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.read_body().unwrap();
        assert_eq!(&body[..], b"Hello World!");
    }
}
