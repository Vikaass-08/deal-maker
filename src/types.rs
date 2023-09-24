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
    pub lender_email: String,
    pub user_email: String
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


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct DocumentRequestUsersAPIReq {
    pub lender_email: String,
    pub borrower_email: String,
}

#[derive(Serialize, Deserialize)]
pub struct DocumentRequestAPIResp {
    pub lender_email: String,
    pub borrower_email: String,
    pub request_status: String,
    pub created_at: NaiveDateTime
}



pub enum DocStatusCode {
    INITITATED,
    REJECTED,
    ACCEPTED
}

impl ToString for DocStatusCode {
    fn to_string(&self) -> String {
        match self {
            DocStatusCode::INITITATED => String::from("INITITATED"),
            DocStatusCode::REJECTED => String::from("REJECTED"),
            DocStatusCode::ACCEPTED => String::from("ACCEPTED"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DocumentRequestLenderAPIReq {
    pub lender_email: String,
    pub borrower_email: String,
    pub status: String
}

#[derive(Serialize, Deserialize)]
pub struct DocumentRequestLenderAPIGetReq {
    pub lender_email: String,
    pub borrower_email: String,
}


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct CreateDealReq {
    pub lender_email: String,
    pub borrower_email: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateDealResp {
    pub document_id: i32,
    pub status: String,
    pub updated_at: NaiveDateTime
}

pub enum DealStatusCode {
    CREATED,
    REJECT,
    DONE
}

impl ToString for DealStatusCode {
    fn to_string(&self) -> String {
        match self {
            DealStatusCode::REJECT => String::from("REJECT"),
            DealStatusCode::DONE => String::from("DONE"),
            DealStatusCode::CREATED => String::from("CREATED"),
        }
    }
}



#[derive(Serialize, Deserialize)]
pub struct UpdateDealReq {
    pub lender_email: String,
    pub borrower_email: String,
    pub status: String
}

#[derive(Serialize, Deserialize)]
pub struct UpdateDealResp {
    pub document_id: i32,
    pub status: String,
    pub updated_at: NaiveDateTime
}