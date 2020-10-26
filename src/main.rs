#![feature(proc_macro_hygiene, decl_macro)]
#![feature(register_attr)]
#![feature(plugin)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
extern crate serde_json;
extern crate serde;
extern crate rocket_multipart_form_data;
extern crate regex;

// defines tools needed to control data inward/outward flow of the DBMS
mod postgres;
// defines tools needed for parsing multipart POST requests
mod multipart;
// XML (de)serializer of content recieved from the DBMS
mod xmlify;
// writing metadata to disk
mod metainfo;
// accessing data in JSON form through a api
mod api;

use std::io;
use std::path::{PathBuf, Path};

use rocket::response::NamedFile;
use rocket::Data;
use rocket::http::ContentType;
use rocket::response::Redirect;
use rocket::response::content::Html;

// GET requests

#[get("/makethread")]
fn makethread() -> io::Result<NamedFile> {
    NamedFile::open("static/makethread.html")
}

#[get("/")]
fn getindex() -> Html<String> {
    Html(xmlify::xmlify_for_index(postgres::retrieve_all_op()))
}

#[get("/gallery")]
fn getgallery() -> Html<String> {
    Html(xmlify::xmlify_for_gallery(postgres::retrieve_all_op(), postgres::retrieve_all_posts()))
}

#[get("/thread/<threadid>")]
fn getindvidualthread(threadid: i32) -> Result<Html<String>, Redirect> {
    match postgres::retrieve_thread(&threadid) {
        Ok(thread) => {
            return Ok(
                Html(
                    xmlify::xmlify_for_indvthread(thread)
                )
            );
        },
        Err(_err) => {
            return Err(
                Redirect::to("/404")
            );
        },
    }
}

// GET API requests

// gets all new posts after a certain post, if the thread exists
#[get("/thread/<threadid>/<after_postid>")]
fn api_new_posts(threadid: i32, after_postid: i32) -> Result<String, Redirect> {
    match api::api_posts_after(&threadid, &after_postid) {
        Ok(json) => {
            Ok(
                json
            )
        },
        Err(_err) => {
            return Err(
                Redirect::to("/500")
            )
        },
    }
}

// POST requests

// creation of a thread
#[post("/makethread",data = "<data>")]
fn makethread_post(content_type: &ContentType, data: Data) -> Redirect {
    match multipart::eval_multipart_thread(content_type, data) {
        None => {
            return Redirect::to("/form");
        },
        Some(thread) => {
            match thread.add_thread() {
                Ok(threadid) => {
                    return Redirect::to(format!("/thread/{}", threadid))
                },
                Err(_) => {
                    return Redirect::to("/500")
                },
            }
        },
    }
}

// creation of a post on a specif   ic thread
#[post("/thread/<threadid>", data = "<data>")]
fn threadid_post(threadid: i32, content_type: &ContentType, data: Data) -> Redirect {
    match multipart::eval_multipart_post(threadid, content_type, data) {
        None => {
            return Redirect::to("/form");
        },
        Some(post) => {
            if post.add_post().is_err() {
                return Redirect::to("/500");
            }
        }
    }
    Redirect::to(format!("/thread/{}", threadid))
}

// Static links to content

#[get("/static/stylesheet.css")]
fn get_css() -> io::Result<NamedFile> {
    NamedFile::open("static/stylesheet.css")
}

#[get("/static/fonts/Inter-Regular.woff2")]
fn inter_regular() -> io::Result<NamedFile> {
    NamedFile::open("static/fonts/Inter-Regular.woff2")
}

#[get("/static/fonts/Inter-Bold.woff2")]
fn inter_bold() -> io::Result<NamedFile> {
    NamedFile::open("static/fonts/Inter-Bold.woff2")
}

#[get("/static/oboe.png")]
fn logo() -> io::Result<NamedFile> {
    NamedFile::open("static/oboe.png")
}

#[get("/static/main.js")]
fn get_js() -> io::Result<NamedFile> {
    NamedFile::open("static/main.js")
}

#[get("/404")]
fn fourofour() -> io::Result<NamedFile> {
    NamedFile::open("static/404.html")
}

#[get("/500")]
fn fivehundred() -> io::Result<NamedFile> {
    NamedFile::open("static/500.html")
}

#[get("/form")]
fn form_err() -> io::Result<NamedFile> {
    NamedFile::open("static/form.html")
}


// let unauthorized users get all files in Pictures/ , iff they are either png or jpg/jpeg
#[get("/Pictures/<file..>")]
fn pictures(file: PathBuf) -> Option<NamedFile> {
    match file.extension() {
        Some(fileending) if (!(fileending == "png" || fileending == "jpeg" || fileending == "jpg" || fileending == "gif")) => return None,
        _ => {}
    }
    NamedFile::open(Path::new("Pictures/").join(file)).ok()
}

// launchable
// defines all routes that should be made available
fn main() {
    let rocket = rocket::ignite()
        .mount("/", routes![form_err,
            fivehundred,
            fourofour,
            pictures,
            logo,
            get_js,
            makethread,
            getindvidualthread,
            getgallery,
            getindex,
            api_new_posts,
            threadid_post,
            makethread_post,
            get_css,
            inter_regular,
            inter_bold]
        );
    rocket.launch();
}
