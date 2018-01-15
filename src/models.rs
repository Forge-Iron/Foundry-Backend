

use schema::{issue_resource, issue, person};

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
    mentor: Option<i32>,
}

#[derive(Queryable)]
#[derive(Serialize)]
pub struct Resource {
    id: i32,
    url: String,
    title: String,
    issue: i32,
}


/*
 * Insertable Types
 */

#[derive(Insertable)]
#[derive(Deserialize)]
#[table_name = "person"]
pub struct NewPerson<'a> {
    name: &'a str,
    email: &'a str,
}


#[derive(Insertable)]
#[derive(Deserialize)]
#[table_name = "issue"]
pub struct NewIssue<'a> {
    title: &'a str,
    body: &'a str,
    mentor: Option<i32>,
}


#[derive(Insertable)]
#[derive(Deserialize)]
#[table_name = "issue_resource"]
pub struct NewResource<'a> {
    url: &'a str,
    title: &'a str,
    issue: i32,
}
