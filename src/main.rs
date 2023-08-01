use pillow::{
    http::*,
    templates::{Context, Template},
};

#[controller(method = "GET", path = "/")]
pub fn index() -> Response {
    Response::html("index")
}



#[tokio::main]
async fn main() {
    let mut router = MainRouter::new();

    router.public();
    router.assets();

    router.add_route(route!(index {}));

    let server = Server::default();

    server.run(router).await;
}
