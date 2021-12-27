table! {
    likes (first, second, cookie) {
        first -> Varchar,
        second -> Varchar,
        cookie -> Varchar,
        timestamp -> Nullable<Timestamp>,
    }
}

table! {
    likes_count (first, second, count) {
        first -> Varchar,
        second -> Varchar,
        count -> BigInt,
    }
}
