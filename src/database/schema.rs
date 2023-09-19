// @generated automatically by Diesel CLI.

diesel::table! {
    agreement (agreement_id) {
        agreement_id -> Int4,
        agreement_data -> Text,
        #[max_length = 20]
        agreement_type -> Varchar,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
        #[max_length = 20]
        user_type -> Varchar,
        #[max_length = 50]
        user_email -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    agreement,
    users,
);
