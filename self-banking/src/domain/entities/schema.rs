// @generated automatically by Diesel CLI.

diesel::table! {
    Asset (id) {
        id -> Nullable<Integer>,
        name -> Text,
        asset_type_id -> Integer,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    AssetType (id) {
        id -> Nullable<Integer>,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    Contact (id) {
        id -> Nullable<Integer>,
        name -> Text,
        business_name -> Text,
        phone -> Text,
        description -> Nullable<Text>,
        contact_type_id -> Integer,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    ContactType (id) {
        id -> Nullable<Integer>,
        name -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    CurrentSheet (id) {
        id -> Nullable<Integer>,
        asset_id -> Integer,
        balance -> Double,
        updated_at -> Text,
    }
}

diesel::table! {
    Expense (id) {
        id -> Nullable<Integer>,
        description -> Text,
        expense_type_id -> Integer,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    ExpenseType (id) {
        id -> Nullable<Integer>,
        name -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    TransactionRecord (id) {
        id -> Nullable<Integer>,
        transaction_type -> Text,
        amount -> Double,
        asset_id -> Integer,
        destination_asset_id -> Nullable<Integer>,
        expense_id -> Nullable<Integer>,
        contact_id -> Nullable<Integer>,
        note -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::joinable!(Asset -> AssetType (asset_type_id));
diesel::joinable!(Contact -> ContactType (contact_type_id));
diesel::joinable!(Expense -> ExpenseType (expense_type_id));

diesel::allow_tables_to_appear_in_same_query!(
    Asset,
    AssetType,
    Contact,
    ContactType,
    CurrentSheet,
    Expense,
    ExpenseType,
    TransactionRecord,
);
