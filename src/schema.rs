table! {
    products (id) {
        id -> Int4,
        productname -> Text,
        productdescription -> Text,
        shortdescription -> Text,
        category -> Text,
        price -> Text,
        active -> Text,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    products,
    users,
);
