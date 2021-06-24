table! {
    users (id) {
        id -> Int4,
        username -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
