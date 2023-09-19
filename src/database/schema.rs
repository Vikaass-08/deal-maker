// @generated automatically by Diesel CLI.

diesel::table! {
    deal (id) {
        id -> Int4,
        lender_id -> Int4,
        user_id -> Int4,
        document_id -> Int4,
        #[max_length = 50]
        status -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    document (id) {
        id -> Int4,
        lender_id -> Int4,
        #[max_length = 50]
        document_type -> Varchar,
        document_data -> Text,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    lender (id) {
        id -> Int4,
        #[max_length = 255]
        org_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(deal -> document (document_id));
diesel::joinable!(deal -> lender (lender_id));
diesel::joinable!(deal -> users (user_id));
diesel::joinable!(document -> lender (lender_id));

diesel::allow_tables_to_appear_in_same_query!(
    deal,
    document,
    lender,
    users,
);
