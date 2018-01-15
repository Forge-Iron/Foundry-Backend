table! {
    issue (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        mentor -> Nullable<Integer>,
    }
}

table! {
    issue_resource (id) {
        id -> Integer,
        url -> Text,
        title -> Text,
        issue -> Integer,
    }
}

table! {
    person (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
    }
}

joinable!(issue -> person (mentor));
joinable!(issue_resource -> issue (issue));
