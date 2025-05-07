use actix_files as fs;
use actix_web::web::Data;
use actix_web::{App, Error, HttpResponse, HttpServer, Result, error, web};
use tera::{Context, Tera};

// ______________________________________________________________________
async fn index(tmpl: Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    ctx.insert("name", "Bob");
    let s = tmpl
        .render("index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

// ______________________________________________________________________
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on: 127.0.0.1:8085, open browser and visit have a try!");
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter1/**/*")).unwrap();

        App::new()
            .app_data(Data::new(tera))
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind("127.0.0.1:8085")?
    .run()
    .await
}
