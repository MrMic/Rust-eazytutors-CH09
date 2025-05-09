use std::str::from_utf8;

use actix_web::{Error, HttpResponse, web};
use argon2::Config;
use serde_json::json;

use super::dbaccess::{get_user_record, post_new_user};
use super::errors::EzyTutorError;
use super::model::{TutorResponse, User};
use super::{model::TutorRegisterForm, state::AppState};

// __ * INFO: Handler function to show the registration form to the user __
pub async fn show_register_form(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_username", "");
    ctx.insert("current_password", "");
    ctx.insert("current_confirmation", "");
    ctx.insert("current_name", "");
    ctx.insert("current_imageurl", "");
    ctx.insert("current_profile", "");
    let s = tmpl
        .render("register.html", &ctx)
        .map_err(|_| EzyTutorError::TeraError("Template Error".to_string()))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

/// Handles the registration process for a tutor.
///
/// This asynchronous function processes the tutor registration form,
/// performs necessary validations, interacts with the application state,
/// and renders the appropriate response.
///
/// # Parameters
/// - `tmpl`: A shared reference to the Tera template engine used for rendering views.
/// - `app_state`: A shared reference to the application state containing configuration and resources.
/// - `params`: The form data submitted by the tutor, containing the registration details.
///
/// # Returns
/// - `Ok(HttpResponse)`: An HTTP response indicating success or failure of the registration process.
/// - `Err(Error)`: An error response if the registration process encounters an issue.
///
/// # Errors
/// This function can return an error in the following cases:
/// - Template rendering errors from the Tera engine.
/// - Validation errors for the form input.
/// - Database-related errors when interacting with the application state.
///
/// # Examples
/// ```
/// use actix_web::{web, HttpResponse};
/// use tera::Tera;
/// use my_app::{AppState, TutorRegisterForm};
///
/// async fn example_usage() -> Result<HttpResponse, Error> {
///     let tmpl = web::Data::new(Tera::new("templates/**/*").unwrap());
///     let app_state = web::Data::new(AppState::new());
///     let params = web::Form(TutorRegisterForm::default());
///
///     handle_register(tmpl, app_state, params).await
/// }
/// ```
// ______________________________________________________________________
pub async fn handle_register(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
    params: web::Form<TutorRegisterForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let s;
    let username = params.username.clone();
    let user = get_user_record(&app_state.db, username.to_string()).await;
    let user_not_found = user.is_err();
    // ! WARN: If user not found in database, proceed to verification of passwords
    if user_not_found {
        if params.password != params.confirmation {
            ctx.insert("error", "Passwords do not match");
            ctx.insert("current_username", &params.username);
            ctx.insert("current_password", "");
            ctx.insert("current_confirmation", "");
            ctx.insert("current_name", &params.name);
            ctx.insert("current_imageurl", &params.imageurl);
            ctx.insert("current_profile", &params.profile);
            s = tmpl
                .render("register.html", &ctx)
                .map_err(|_| EzyTutorError::TeraError("Template Error".to_string()))?;
        } else {
            let new_tutor = json!({
                "tutor_name": &params.name,
                "tutor_pic_url": &params.imageurl,
                "tutor_profile": &params.profile
            });
            let awc_client = awc::Client::default();
            let res = awc_client
                .post("http://localhost:3002/tutors/")
                .send_json(&new_tutor)
                .await
                .unwrap()
                .body()
                .await?;
            let tutor_response: TutorResponse = serde_json::from_str(from_utf8(&res)?)?;
            s = format!(
                "Congratulations. You have been successfully registered with EzyTutor and your tutor id is: {}. To start using EzyTutor, please login with your credentials.",
                tutor_response.tutor_id
            );

            // Hash the password
            let salt = b"somerandomsalt";
            let config = Config::default();
            let hash =
                argon2::hash_encoded(params.password.clone().as_bytes(), salt, &config).unwrap();
            let user = User {
                username,
                tutor_id: Some(tutor_response.tutor_id),
                user_password: hash,
            };
            let _tutor_created = post_new_user(&app_state.db, user).await?;
        }
    } else {
        ctx.insert("error", "User Id already exists");
        ctx.insert("current_username", &params.username);
        ctx.insert("current_password", "");
        ctx.insert("current_confirmation", "");
        ctx.insert("current_name", &params.name);
        ctx.insert("current_imageurl", &params.imageurl);
        ctx.insert("current_profile", &params.profile);
        s = tmpl
            .render("register.html", &ctx)
            .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
    }

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

// ______________________________________________________________________
pub async  show_signup_form() -> Result<HttpResponse, Error> {
todo!()
}


// ______________________________________________________________________
pub async  handle_signin() -> Result<HttpResponse, Error> {
todo!()
}
