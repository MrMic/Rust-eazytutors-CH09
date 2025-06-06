use std::str::from_utf8;

use actix_web::{Error, HttpResponse, Result, web};
use serde_json::json;

use crate::iter6::{
    model::{NewCourse, NewCourseResponse},
    state::AppState,
};

// ______________________________________________________________________
pub async fn handle_insert_course(
    _tmpl: web::Data<tera::Tera>,
    _app_state: web::Data<AppState>,
    path: web::Path<i32>,
    params: web::Json<NewCourse>,
) -> Result<HttpResponse, Error> {
    let tutor_id = path.into_inner();
    let new_course = json!({
        "tutor_id": tutor_id,
        "course_name": &params.course_name,
        "course_description": &params.course_description,
        "course_format": &params.course_format,
        "course_structure": &params.course_structure,
        "course_duration": &params.course_duration,
        "course_price": &params.course_price,
        "course_language": &params.course_language,
        "course_level": &params.course_level,
    });
    let awc_client = awc::Client::default();
    let res = awc_client
        .post("http://localhost:3002/courses/")
        .send_json(&new_course)
        .await
        .unwrap()
        .body()
        .await?;
    println!("Finished call: {:?}", res);
    let course_response: NewCourseResponse = serde_json::from_str(from_utf8(&res)?)?;
    Ok(HttpResponse::Ok().json(course_response))
}

// ______________________________________________________________________
pub async fn handle_update_course(
    _tmpl: web::Data<tera::Tera>,
    _app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Got update request"))
}

// ______________________________________________________________________
// Handler function to delete a course for a tutor_id
pub async fn handle_delete_course(
    _tmpl: web::Data<tera::Tera>,
    _app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Course deleted"))
}
