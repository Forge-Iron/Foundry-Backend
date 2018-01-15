table! {
    Issue (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        mentor -> Nullable<Integer>,
    }
}

table! {
    IssueResource (id) {
        id -> Integer,
        url -> Text,
        title -> Text,
        issue -> Integer,
    }
}

table! {
    Person (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
    }
}

joinable!(Issue -> Person (mentor));
joinable!(IssueResource -> Issue (issue));
