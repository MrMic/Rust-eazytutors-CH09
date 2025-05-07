use actix_web::{
    App, Error, HttpResponse, HttpServer, error,
    web::{self, Data},
};
use serde::{Deserialize, Serialize};
use tera::Tera;

// Store Tera template in Application State
async fn index(tmpl: Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let s = tmpl
        .render("form.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

//----------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
pub struct Tutor {
    name: String,
}

//----------------------------------------------------------------------
// curl -X POST localhost:8080/tutors -d "name=Terry"
async fn handle_post_tutor(
    tmpl: Data<tera::Tera>,
    params: web::Form<Tutor>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", &params.name);
    ctx.insert("text", "Welcome!");
    let s = tmpl
        .render("user.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

// * INFO: ╞╡ MAIN ╞══════════════════════════════════════════════════════════╡
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on: 127.0.0.1:8085, open browser and visit have a try!");
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter2/**/*")).unwrap();

        App::new().app_data(Data::new(tera)).configure(app_config)
    })
    .bind("127.0.0.1:8085")?
    .run()
    .await
}

//----------------------------------------------------------------------
fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/tutor").route(web::post().to(handle_post_tutor))),
    );
}

//*INFO:          ╓─────────────────────────────────────────────────────────╖
//*INFO:          ║                          TESTS                          ║
//*INFO:          ╙─────────────────────────────────────────────────────────╜
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        dev::{Service, ServiceResponse},
        http::{
            StatusCode,
            header::{CONTENT_TYPE, HeaderValue},
        },
        test,
        web::Form,
    };

    #[actix_rt::test]
    async fn handle_post_1_unit_test() {
        let params = Form(Tutor {
            name: "Terry".to_string(),
        });
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter2/**/*")).unwrap();
        let webdata_tera = Data::new(tera);
        let resp = handle_post_tutor(webdata_tera, params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/html")
        );
    }

    // Integration test case
    #[actix_rt::test]
    async fn handle_post_1_integration_test() {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter2/**/*")).unwrap();

        let app =
            test::init_service(App::new().app_data(Data::new(tera)).configure(app_config)).await;

        let req = test::TestRequest::post()
            .uri("/tutor")
            .set_form(&Tutor {
                name: "Terry".to_string(),
            })
            .to_request();

        let resp: ServiceResponse = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/html")
        );
    }
}
