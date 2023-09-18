// @generated automatically by Diesel CLI.

diesel::table! {
    agreement (id) {
        id -> Int4,
        agreement_data -> Varchar,
        data_type -> Text,
    }
}
