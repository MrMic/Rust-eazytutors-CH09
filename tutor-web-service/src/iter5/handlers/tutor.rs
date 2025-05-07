// tutor-db/src/iter5/handlers/tutor.rs
use crate::dbaccess::tutor::*;
use crate::models::tutor::{NewTutor, UpdateTutor};
use crate::{errors::EzyTutorError, state::AppState};
use actix_web::{
    web::{self},
    HttpResponse,
};

// ______________________________________________________________________
pub async fn get_all_tutors(app_state: web::Data<AppState>) -> Result<HttpResponse, EzyTutorError> {
    get_all_tutors_db(&app_state.db)
        .await
        .map(|tutors| HttpResponse::Ok().json(tutors))
}

// ______________________________________________________________________
pub async fn get_tutor_details(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = path.into_inner();
    get_tutor_details_db(&app_state.db, tutor_id)
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

// ______________________________________________________________________
pub async fn post_new_tutor(
    new_tutor: web::Json<NewTutor>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, EzyTutorError> {
    post_new_tutor_db(&app_state.db, NewTutor::from(new_tutor))
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

// ______________________________________________________________________
pub async fn update_tutor_details(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
    update_tutor: web::Json<UpdateTutor>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = path.into_inner();
    update_tutor_details_db(&app_state.db, tutor_id, UpdateTutor::from(update_tutor))
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

// ______________________________________________________________________
pub async fn delete_tutor(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = path.into_inner();
    delete_tutor_db(&app_state.db, tutor_id)
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

// * INFO:                                   ┌╌╌╌╌╌╌╌┐
// * INFO:                                   ╎ TESTS ╎
// * INFO:                                   └╌╌╌╌╌╌╌┘

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::{env, sync::Mutex};

    // ______________________________________________________________________
    #[actix_rt::test]
    async fn get_all_tutors_success_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });

        let resp = get_all_tutors(app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    // ______________________________________________________________________
    #[actix_rt::test]
    async fn get_tutor_details_success_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let parameters: web::Path<i32> = web::Path::from(3);
        let resp = get_tutor_details(app_state, parameters).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    // ______________________________________________________________________
    // #[ignore]
    #[actix_rt::test]
    async fn post_tutor_success_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let new_tutor_msg = NewTutor {
            tutor_name: "Third Tutor".into(),
            tutor_pic_url: "http://tutor.s3.com/ssdfds".into(),
            tutor_profile: "Experienced tutor in Statistics".into(),
        };
        let tutor_param = web::Json(new_tutor_msg);
        let resp = post_new_tutor(tutor_param, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    // ____________________________________________________________________
    // Delete tutor
    #[actix_rt::test]
    async fn delete_tutor_success_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let parameters: web::Path<i32> = web::Path::from(2);
        let resp = delete_tutor(app_state, parameters).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
