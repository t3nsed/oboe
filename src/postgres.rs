extern crate postgres;

use postgres::{Connection, TlsMode};
use std::{env, process};
use serde::{Deserialize, Serialize};

// setting env var 'SQL_URL' is necessary for the program to know where the DB is.
// default was postgresql://rocket:testcase@localhost:11375/site_content
pub fn get_psql_entry() -> String {
    env::var("SQL_URL")
    .unwrap_or_else(|err| {
        println!("You must set the \"SQL_URL\" to a readable and writable DB. \n Detailed Error: \n\n {}", err);
        // if the DB does not exist or is not named, the user must fix this issue.
        process::exit(1);
    })
}
// A Opening Post ('OP') is a struct that contains identifiable information of the user, in addition to threadid,
// which should be generated
#[derive(Serialize, Deserialize)]
pub struct OP {
    pub threadid: i32,
    // must be UTF-8 or ASCII, Postgres Driver panics with others due to conversion
    // to internal Postgres data formats
    pub poster: String,
    pub title: String,
    pub body: String,
    pub img: String,
    pub time: String,
    pub date: String
}

impl OP {
    // Adds a filled-in 'OP' struct to the DB.
    // Either returns the thread id if the insertion was successful, else returns error message given by the DB driver
    // TODO - currently this panics when something is wrong with the DB instead of returning the error.
    pub fn add_thread(&self) -> Result<i32, &'static str> {
        let connection = Connection::connect(get_psql_entry(), TlsMode::None).unwrap();
        connection.execute("INSERT INTO threads VALUES ($1, $2, $3, $4, $5, $6, $7);",
                           &[&self.threadid, &self.poster, &self.title, &self.body, &self.img, &self.time, &self.date]).unwrap();
        Ok(self.threadid)
    }
}

// A 'Post' is a comment on a thread.
#[derive(Serialize, Deserialize)]
pub struct Post {
    // to which 'OP' it belongs
    pub threadid: i32,
    pub poster: String,
    pub body: String,
    pub img: String,
    pub time: String,
    pub date: String,
    pub postid: i32
}

impl Post {
    // Adds a filled-in 'Post' struct to the DB.
    // Either returns nothing if the insertion was successful, else returns error message given by the DB driver.
    // TODO - currently this panics when something is wrong with the DB instead of returning the error.
    pub fn add_post(&self) -> Result<(), &'static str> {
        let connection = Connection::connect(get_psql_entry(), TlsMode::None).unwrap();
        connection.execute("INSERT INTO posts VALUES ($1, $2, $3, $4, $5, $6, $7);",
                           &[&self.threadid, &self.poster, &self.body, &self.img, &self.time, &self.date, &self.postid]).unwrap();
        Ok(())
    }
}

// A 'Thread' contains one opening post 'OP', and any number of replies 'Post' or none.
#[derive(Serialize, Deserialize)]
pub struct Thread {
    pub op: OP,
    pub posts: Option<Vec<Post>>
}

// The following are functions relating to retrieving DB instead of inserting it.

// Retrieval of a 'Thread', or a Error message if it does not exist
pub fn retrieve_thread(threadid: &i32) -> Result<Thread, &'static str> {
    match retrieve_op(threadid) {
        None => { return Err("the specified thread does not exist.")},
        Some(op) => {
            // iff there exists a thread with said threadid, then posts may also exist.
            let posts: Option<Vec<Post>> = retrieve_posts(threadid);
            return Ok( Thread { op, posts } );
        },
    };
}

// retrieve a opening post ('OP').
pub fn retrieve_op(threadid: &i32) -> Option<OP> {
    let connection = Connection::connect(get_psql_entry(), TlsMode::None).unwrap();
    for row in &connection.query("SELECT * FROM threads WHERE threadid=$1", &[threadid]).unwrap() {
        return Some(
            OP {
            threadid: row.get(0),
            poster: row.get(1),
            title: row.get(2),
            body: row.get(3),
            img: row.get(4),
            time: row.get(5),
            date: row.get(6)
            }
        );
    }
    None
}

// retrieve all opening posts ('OP') as an array.
pub fn retrieve_all_op() -> Vec<OP> {
    let mut returnable = Vec::new();
    let connection = Connection::connect(get_psql_entry(), TlsMode::None).unwrap();
    for row in &connection.query("SELECT * FROM threads", &[]).unwrap() {
        returnable.push(
            OP {
                threadid: row.get(0),
                poster: row.get(1),
                title: row.get(2),
                body: row.get(3),
                img: row.get(4),
                time: row.get(5),
                date: row.get(6)
            }
        );
    }
    returnable
}

// retrieve all posts ('Post') belonging to a opening post ('OP') in an array if there exist some.
pub fn retrieve_posts(threadid: &i32) -> Option<Vec<Post>> {
    let mut posts: Vec<Post> = Vec::new();
    let connection = Connection::connect(get_psql_entry(), TlsMode::None).unwrap();
    for row in &connection.query("SELECT * FROM posts WHERE threadid=$1", &[threadid]).unwrap() {
        posts.push(
            Post {
                threadid: row.get(0),
                poster: row.get(1),
                body: row.get(2),
                img: row.get(3),
                time: row.get(4),
                date: row.get(5),
                postid: row.get(6)
            }
        );
    }
    if posts.is_empty() {
        return None;
    }
    Some(posts)
}

// retrieve all posts ('Post').
pub fn retrieve_all_posts() -> Vec<Post> {
    let mut posts: Vec<Post> = Vec::new();
    let connection = Connection::connect(get_psql_entry(), TlsMode::None).unwrap();
    for row in &connection.query("SELECT * FROM posts", &[]).unwrap() {
        posts.push(
            Post {
                threadid: row.get(0),
                poster: row.get(1),
                body: row.get(2),
                img: row.get(3),
                time: row.get(4),
                date: row.get(5),
                postid: row.get(6)
            }
        );
    }
    posts
}

// IMPORTANT: these will only succeed if the system has a valid SQL database connected
#[cfg(test)]
mod tests {
    use crate::postgres::{OP, Post, retrieve_op, retrieve_posts};
    use rand::{random, Rng};

    #[test]
    fn check_thread_creation() {
        let mut generator = rand::thread_rng();
        let threadid = generator.gen::<u32>() as i32;
        let dummy_thread = OP {
            threadid,
            poster: "Anonymous".to_owned(),
            title: "this is a thread".to_owned(),
            body: "this is a thread body".to_owned(),
            img: "this is the image URL".to_owned(),
            time: "13:37".to_owned(),
            date: "01.01.2019".to_owned()
        };
        &dummy_thread.add_thread();

        assert_eq!(retrieve_op(&threadid).unwrap().threadid, *&threadid);
        //check DB manually for correctness of tuples
    }

    #[test]
    fn check_post_creation() {
        let mut generator = rand::thread_rng();
        let threadid = generator.gen::<u32>() as i32;
        let dummy_post = Post {
            threadid,
            poster: "Anonymous".to_owned(),
            body: "this is a post body".to_owned(),
            img: "this is the image URL".to_owned(),
            time: "13:38".to_string(),
            date: "01.02.2019".to_string(),
            postid: generator.gen::<u32>() as i32
        };
        &dummy_post.add_post();

        assert!(!retrieve_posts(&threadid).unwrap().is_empty());
        //check DB manually for correctness of tuples
    }
}

