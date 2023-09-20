use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct  CreateUserReq {
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct LoginUserReq {
    pub user_email: String,
    pub user_password: String
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserResp {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: NaiveDateTime
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUserResp {
    pub jwt_token: String
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct SaveDocumentReq {
    pub document_data: String,
    pub document_type: String,
    pub lender_id: i32
}


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct CreateLenderReq {
    pub org_name: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct CreateLenderResp {
    pub org_name: String,
    pub email: String,
    pub created_at: NaiveDateTime
}

#[derive(Serialize, Deserialize)]
pub struct LoginLenderReq {
    pub lender_email: String,
    pub lender_password: String
}

#[derive(Serialize, Deserialize)]
pub struct LoginLenderResp {
    pub jwt_token: String
}