

#[derive(Queryable)]
pub struct Person {
    id: i32,
    name: String,
    email: String,
}

#[derive(Queryable)]
pub struct Issue {
    id: i32,
    title: String,
}
