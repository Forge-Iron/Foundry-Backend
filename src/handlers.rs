use rouille::Request;
use rouille::Response;

use database::get_database_connection;

use models::*;
use diesel::prelude::*;
use schema::*;


fn get_issues(request: &Request) -> Response {

    let connection = get_database_connection();
    let issue_list = issue::table.load::<GitIssue>(&connection).unwrap();

    Response::json(&issue_list)


}
