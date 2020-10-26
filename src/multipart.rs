use std::fs;
use rand::Rng;
use std::error::Error;

// for UNIX time formats
extern crate chrono;
// for creating a tripcode
extern crate md5;

use chrono::{DateTime, Utc};
use regex::Regex;

use crate::postgres::{OP, Post};
use crate::metainfo;

use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, FileField, TextField};
use rocket::Data;
use rocket::http::ContentType;
use self::chrono::{Timelike, Datelike};

// retrieves raw data ('data') of the POST request and content types ('ContentType') found in the data.
// Parses this info, and returns a Opening Post struct ('OP') or nothing if the input is faulty.
pub fn eval_multipart_thread(content_type: &ContentType, data: Data) -> Option<OP> {

    // configuration
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields.push(MultipartFormDataField::text("poster"));
    options.allowed_fields.push(MultipartFormDataField::text("title"));
    options.allowed_fields.push(MultipartFormDataField::text("body"));
    options.allowed_fields.push(MultipartFormDataField::file("image").content_type_by_string(Some(mime::IMAGE_STAR)).unwrap());

    // parsing
    // may fail if the user submits a invalid form, or does not fill out fields correctly.
    let multipart_form_data = match MultipartFormData::parse(content_type, data, options) {
        Ok(form) => form,
        Err(_err) => return None,
    };
    // extracting of content of fields
    let poster = multipart_form_data.texts.get(&"poster".to_string());
    let title = multipart_form_data.texts.get(&"title".to_string());
    let body = multipart_form_data.texts.get(&"body".to_string());
    let image = multipart_form_data.files.get(&"image".to_string());

    // evaluation of fields
    let mut poster= extract_text(poster);
    // if the user didn't enter a user name, he/she wants to stay anonymous
    if poster.is_empty() {
        poster = "Anonymous".to_owned();
    }
    // if the user submits a username in the form 'something#something' a identifier gets computed
    // and replaced with the actual username
    if poster.contains("#") {
        poster = get_tripcode(poster)
    }
    let title = extract_text(title);
    let body = extract_text(body);

    //do not allow DB entry if these two are empty
    if body.is_empty() || title.is_empty() {
        return None
    }

    let image = extract_image(image);

    let mut generator = rand::thread_rng();
    let threadid = generator.gen::<u32>() as i32;

    let (time, date) = get_utc_current();

    // see metainfo.rs for more information
    metainfo::create_info_file(&threadid).unwrap();

    Some(OP {
        threadid,
        poster,
        title,
        body,
        img: image,
        time,
        date
    })
}

// Essentially the same as above, but without the 'title' field which is needed in a Opening Post, but not in a 'Post'.
// Additionally, a 'threadid' needs to be specified to signal to which 'Thread' a 'Post' belongs.
pub fn eval_multipart_post(threadid: i32, content_type: &ContentType, data: Data) -> Option<Post> {

    // configuration
    let mut options = MultipartFormDataOptions::new();

    options.allowed_fields.push(MultipartFormDataField::text("poster"));
    options.allowed_fields.push(MultipartFormDataField::text("body"));
    options.allowed_fields.push(MultipartFormDataField::file("image").content_type_by_string(Some(mime::IMAGE_STAR)).unwrap());

    // parsing
    // may fail if the user submits a invalid form, or does not fill out fields correctly.
    let multipart_form_data = match MultipartFormData::parse(content_type, data, options) {
        Ok(form) => form,
        Err(_err) => return None,
    };

    // extracting of content of fields
    let poster = multipart_form_data.texts.get(&"poster".to_string());
    let body = multipart_form_data.texts.get(&"body".to_string());
    let img = multipart_form_data.files.get(&"image".to_string());

    //evaluation of fields
    let mut poster= extract_text(poster);
    if poster.is_empty() {
        poster = "Anonymous".to_owned();
    }
    // if the user submits a username in the form 'something#something' a identifier gets computed
    if poster.contains("#") {
        poster = get_tripcode(poster)
    }
    let body = extract_text(body);

    //do not allow DB entry if body is empty
    if body.is_empty() {
        return None
    }

    let img = extract_image(img);

    let (time, date) = get_utc_current();

    // see metainfo.rs for more information
    let postid = metainfo::get_postid(&threadid, true).unwrap();

    Some(Post {
        threadid,
        poster,
        body,
        img,
        time,
        date,
        postid
    })
}



// gets current time and date, formatted as a tuple of strings.
fn get_utc_current() -> (String, String) {
    let now: DateTime<Utc> = Utc::now();
    let time = format!("{}:{}:{}", now.hour() + 2, now.minute(), now.second());
    let date = format!("{}.{}.{}", now.day(), now.month(), now.year());
    (time, date)
}

// computes a tripcode. Everything after the first '#' is hashed with md5
fn get_tripcode(poster: String) -> String {
    let regex = Regex::new(r"^(.+)#(.+)$").unwrap();
    for group in regex.captures_iter(&poster) {
        let md5 = md5::compute(&group[2]);
        return format!("{}#{:x}", &group[1], &md5);
    }
    "".to_owned()
}

// helper functions for the eval_* functions above. These are needed since the multipart libary used
// uses structs that are far to abstract for our need.
fn extract_text(text_field: Option<&TextField>) -> String {
    if let Some(text_field) = text_field {
        match text_field {
            TextField::Single(text) => {
                let _content_type = &text.content_type;
                let _file_name = &text.file_name;
                let _text = &text.text;
                return _text.to_owned()
            }
            TextField::Multiple(_texts) => {
            	// each textfield has its own name/identifier, so this is not used
            }
        }
    }
    "error parsing this textfield".to_owned()
}

fn extract_image(image_field: Option<&FileField>) -> String {
    if let Some(image_field) = image_field {
        match image_field {
            FileField::Single(file) => {
                let _content_type = &file.content_type;
                let _file_name = file.file_name.clone();
                let _path = &file.path;
                let mut pathbuilder: String = String::from("Pictures/");
                pathbuilder.push_str(_file_name.unwrap().as_str());
                println!(
                    "Saving to accessable directory {} ...", &pathbuilder
                );
                match fs::copy(_path, &pathbuilder) {
                    Ok(_) => { /* Ok */},
                    Err(err) => {
                        println!(
                            "Error: {}", err.description()
                        );

                    },
                }
                let pathbuilder = pathbuilder.clone();
                return pathbuilder
            }
            FileField::Multiple(_file) => {
                //unused, but needs to be specified in this interface
                //could be used for multiple file uploads if multiple html forms are present
            }
        }
    }
    "error parsing this image".to_owned()
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use super::chrono::{Timelike, Datelike};

    #[test]
    fn timeformats() {
        let now: DateTime<Utc> = Utc::now();
        //UNIX, not UTC or GMT, but has been adjusted for GMT+1
        let time = format!("{}:{}:{}", now.hour() + 2, now.minute(), now.second());
        let date = format!("{}/{}/{}", now.day(), now.month(), now.year());
        //check manually against your system clock
        assert_eq!(time, time);
        assert_eq!(date, date);
    }
}