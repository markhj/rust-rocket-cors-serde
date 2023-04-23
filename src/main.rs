#[macro_use] extern crate rocket;

use rocket::{
    Request,
    Response,
    response,
    http::{Header, ContentType},
    response::{Builder, content, Responder},
    futures::io::Cursor,
};

use serde::Deserialize;
use serde::Serialize;
use serde_json::Result;

struct JsonResponse {
    body: String,
}

impl<'r> Responder<'r, 'static> for JsonResponse {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let string = self.body;
        Response::build_from(string.respond_to(req)?)
            .raw_header("Access-Control-Allow-Origin", "example.com")
            .raw_header("Access-Control-Allow-Headers", "X-Custom-Header")
            .raw_header("Access-Control-Allow-Methods", "GET, POST, PATCH, PUT, DELETE, OPTION")
            .header(ContentType::new("application", "json"))
            .ok()
    }
}

#[derive(Serialize, Deserialize)]
struct IndexResponse {
    foo: String,
    bar: i32,
}

#[get("/")]
fn index() -> JsonResponse {
    let result = IndexResponse {
        foo: String::from("bar"),
        bar: 70,
    };

    JsonResponse {
        body: serde_json::to_string(&result).unwrap(),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
