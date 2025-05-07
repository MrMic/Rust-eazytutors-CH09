use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tutor {
    pub tutor_id: i32,
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

//   ______________________________________________________________________
#[derive(Deserialize, Debug, Clone)]
pub struct NewTutor {
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

impl From<web::Json<NewTutor>> for NewTutor {
    /// Creates a `NewTutor` instance from a `web::Json<NewTutor>`.
    fn from(new_tutor: web::Json<NewTutor>) -> Self {
        NewTutor {
            tutor_name: new_tutor.tutor_name.clone(),
            tutor_pic_url: new_tutor.tutor_pic_url.clone(),
            tutor_profile: new_tutor.tutor_profile.clone(),
        }
    }
}

//   ______________________________________________________________________
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTutor {
    pub tutor_name: Option<String>,
    pub tutor_pic_url: Option<String>,
    pub tutor_profile: Option<String>,
}

impl From<web::Json<UpdateTutor>> for UpdateTutor {
    /// Creates an `UpdateTutor` instance from a `web::Json<UpdateTutor>`.
    fn from(update_tutor: web::Json<UpdateTutor>) -> Self {
        UpdateTutor {
            tutor_name: update_tutor.tutor_name.clone(),
            tutor_pic_url: update_tutor.tutor_pic_url.clone(),
            tutor_profile: update_tutor.tutor_profile.clone(),
        }
    }
}
