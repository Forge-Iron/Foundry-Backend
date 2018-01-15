

#[derive(Queryable)]
#[derive(Serialize)]
pub struct GitPerson {
    id: i32,
    name: String,
    email: String,
}

#[derive(Queryable)]
#[derive(Serialize)]
pub struct GitIssue {
    id: i32,
    title: String,
    body: String,
    mentor: i32,
}

#[derive(Queryable)]
#[derive(Serialize)]
pub struct Resource {
    id: i32,
    url: String,
    title: String,
    issue: i32,
}
