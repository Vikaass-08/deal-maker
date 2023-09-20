use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct  CreateUserReq {
  pub first_name: String,
  pub last_name: String,
  pub email: String
}



#[derive(Serialize, Deserialize)]
pub struct SaveDocumentReq {
    pub document_data: String,
    pub document_type: String,
    pub lender_id: i32
}