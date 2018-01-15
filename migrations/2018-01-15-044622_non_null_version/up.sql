-- Your SQL goes here

CREATE TABLE Person(

        id INTEGER PRIMARY KEY,
       name TEXT NOT NULL,
        email TEXT NOT NULL

);


 CREATE TABLE Issue (
        id INTEGER PRIMARY KEY,
        title TEXT NOT NULL,
        body TEXT NOT NULL,
   		mentor INTEGER,
        FOREIGN KEY(mentor) REFERENCES Person(id)
);

 CREATE TABLE IssueResource (
        id INTEGER PRIMARY KEY,
         url TEXT NOT NULL,
         title TEXT NOT NULL,
   		 issue INTEGER NOT NULL,
         FOREIGN KEY(issue) REFERENCES Issue(id)
 );
