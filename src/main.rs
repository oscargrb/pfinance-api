use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

#[macro_use]
extern crate rocket;

mod db;
mod model;
mod read_query;
mod routes;

#[launch]
async fn rocket() -> _ {
    let client = db::conection_db().await.expect("Db conn fail");

    rocket::build()
        .manage(db::DbClient(client))
        .attach(CORS)
        .mount(
            "/",
            routes![
                routes::index,
                routes::get_budget,
                routes::submit_budget,
                routes::submit_detail,
                routes::get_detail,
                routes::get_data_dashboard,
                routes::get_data_form_budget,
                routes::get_data_form_detail,
                routes::submit_tasa,
                all_options
            ],
        )
}

struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, OPTIONS",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        ));
    }
}

#[options("/<_..>")]
fn all_options() -> &'static str {
    ""
}
