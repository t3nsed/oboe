use serde_json::Result;
use crate::postgres::*;

// very simple api that returns JSON data if requested. Currently only used for AJAX
// to fetch new comments.

pub fn api_posts_after(threadid: &i32, after_postid: &i32) -> Result<(String)> {
    let posts = match retrieve_posts(threadid) {
        Some(posts) => {
            posts
        },
        _ => {
            return Ok("nil".to_owned());
        }
    };
    let mut json_buffer = String::from("[");

    for post in posts.iter() {
        if post.postid > *after_postid {
            &json_buffer.push_str(serde_json::to_string(post)?.as_str());
            &json_buffer.push(',');
        }
    }
    &json_buffer.pop();
    &json_buffer.push(']');
    Ok(json_buffer)
} 