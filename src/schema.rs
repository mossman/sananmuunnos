use diesel::table;

table! {
    likes (first, second, cookie) {
        first -> Varchar,
        second -> Varchar,
        cookie -> Varchar,
        timestamp -> Nullable<Timestamp>,
    }
}
