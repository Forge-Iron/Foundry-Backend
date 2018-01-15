-- Your SQL goes here

CREATE TABLE person(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL

);

 CREATE TABLE issue (
    id INTEGER PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
   	mentor INTEGER,
    FOREIGN KEY(mentor) REFERENCES person(id)
);

 CREATE TABLE issue_resource (
    id INTEGER PRIMARY KEY NOT NULL,
    url TEXT NOT NULL,
    title TEXT NOT NULL,
   	issue INTEGER NOT NULL,
    FOREIGN KEY(issue) REFERENCES issue(id)
 );