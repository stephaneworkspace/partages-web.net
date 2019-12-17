table! {
    da01_user (id) {
        id -> Int4,
        name -> Varchar,
        mail -> Varchar,
        password -> Varchar,
        active -> Bool,
    }
}

table! {
    db01_quote (id) {
        id -> Int4,
        da01_user_id -> Int4,
        author -> Varchar,
        author_activity -> Varchar,
        quote -> Text,
        sw_published -> Bool,
    }
}

joinable!(db01_quote -> da01_user (da01_user_id));

allow_tables_to_appear_in_same_query!(
    da01_user,
    db01_quote,
);
