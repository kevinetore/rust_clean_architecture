diesel::table! {
    books {
        id -> Integer,
        title -> Text,
        author -> Text,
        pages -> Integer,
    }
}
