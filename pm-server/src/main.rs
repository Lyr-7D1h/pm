// #![feature(decl_macro, proc_macro_hygiene)]

// use rocket::get;
// use rocket_contrib::json::Json;
// use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
// use rocket_okapi::{openapi, routes_with_openapi, JsonSchema};

// #[derive(serde::Serialize, JsonSchema)]
// struct Response {
//     reply: String,
// }

// #[openapi]
// #[get("/")]
// fn my_controller() -> Json<Response> {
//     Json(Response {
//         reply: "show me the docs!".to_string(),
//     })
// }

// fn get_docs() -> SwaggerUIConfig {
//     use rocket_okapi::swagger_ui::UrlObject;

//     SwaggerUIConfig {
//         url: "/my_resource/openapi.json".to_string(),
//         urls: vec![UrlObject::new("My Resource", "/v1/company/openapi.json")],
//         ..Default::default()
//     }
// }

// fn main() {
//     rocket::ignite()
//         .mount("/my_resource", routes_with_openapi![my_controller])
//         .mount("/swagger", make_swagger_ui(&get_docs()))
//         .launch();
// }
use actix_web::{App, HttpServer};
use paperclip::actix::{
    api_v2_operation,
    // If you prefer the macro syntax for defining routes, import the paperclip macros
    // get, post, put, delete
    // use this instead of actix_web::web
    web::{self, Json},
    Apiv2Schema,
    // extension trait for actix_web::App and proc-macro attributes
    OpenApiExt,
};
use serde::{Deserialize, Serialize};

// Mark containers (body, query, parameter, etc.) like so...
#[derive(Serialize, Deserialize, Apiv2Schema)]
struct Pet {
    name: String,
    id: Option<i64>,
}

// Mark operations like so...
#[api_v2_operation]
// Add the next line if you want to use the macro syntax
// #[post("/pets")]
async fn echo_pet(body: Json<Pet>) -> Result<Json<Pet>, ()> {
    Ok(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Record services and routes from this line.
            .wrap_api()
            // Add routes like you normally do...
            .service(web::resource("/pets").route(web::post().to(echo_pet)))
            // Or just .service(echo_pet) if you're using the macro syntax
            // Mount the v2/Swagger JSON spec at this path.
            .with_json_spec_at("/api/spec/v2")
            // If you added the "v3" feature, you can also include
            // .with_json_spec_v3_at("/api/spec/v3")
            // ... or if you wish to build the spec by yourself...
            // .with_raw_json_spec(|app, spec| {
            //     app.route("/api/spec", web::get().to(move || {
            //         actix_web::HttpResponse::Ok().json(&spec)
            //     }))
            // })
            // IMPORTANT: Build the app!
            .build()
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
