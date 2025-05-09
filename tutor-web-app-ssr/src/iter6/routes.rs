use actix_files as fs;
use actix_web::web;

use super::handler::{handle_register, handle_signin, show_register_form, show_signup_form};

pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/").route(web::get().to(show_register_form)))
            .service(web::resource("/signinform").route(web::get().to(show_signup_form)))
            .service(web::resource("/signin").route(web::post().to(handle_signin)))
            .service(web::resource("/register").route(web::post().to(handle_register))),
    );
}
