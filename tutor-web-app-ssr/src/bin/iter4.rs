use actix_files as fs;
use actix_web::{
    App, Error, HttpResponse, HttpServer, error,
    web::{self, Data},
};
use awc::Client;
use serde::{Deserialize, Serialize};
use tera::Tera;

// +--------------------------------------------------------------------+
#[derive(Serialize, Deserialize, Debug)]
pub struct Tutor {
    pub tutor_id: i32,
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

// +--------------------------------------------------------------------+
async fn handle_get_tutors(tmpl: Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let client = Client::default();

    // Create request builder and send request
    let res = client
        .get("http://localhost:3002/tutors/")
        .send()
        .await
        .unwrap()
        .body()
        .await
        .unwrap();

    let str_list = std::str::from_utf8(&res).unwrap();
    let tutor_list: Vec<Tutor> = serde_json::from_str(str_list).unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("tutors", &tutor_list);

    let rendered_html = tmpl
        .render("list.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered_html))
}

//* INFO:  ╞══════════════════════════════╡ MAIN ╞═══════════════════════════╡
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on: 127.0.0.1:8085, open browser and visit have a try!");
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter4/**/*")).unwrap();

        App::new()
            .app_data(Data::new(tera))
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/tutors").route(web::get().to(handle_get_tutors)))
    })
    .bind("127.0.0.1:8085")?
    .run()
    .await
}
