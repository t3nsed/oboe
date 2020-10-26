use crate::postgres::{OP, Thread, Post};
use crate::metainfo;

// These functions take a reference of a string and add HTML5-compliant tags around them.

pub fn to_xml_paragraph(string: &String) -> String {
    format!("<p>{}</p>", string)
}

pub fn to_xml_header1(string: &String) -> String {
    format!("<h1>{}</h1>", string)
}

pub fn to_xml_header3(string: &String) -> String {
    format!("<h3>{}</h3>", string)
}

pub fn to_xml_link(url: &String, text: &String) -> String {
    format!("<a class=\"underline--hover\" href=\"{}\">{}</a>", url, text)
}

// creates a clickable link to a thread from its thread id
pub fn clickable_thread(ops: &OP) -> String {
    format!("/thread/{}",ops.threadid)
}

// content is what should be in the div, class are the css classes applied
pub fn to_xml_div_w_class(content: &String, class: &str) -> String {
    format!("<div class=\"{}\">{}</div>", class, content)
}

pub fn to_xml_div_w_id(content: &String, id: &str) -> String {
    format!("<div id=\"{}\">{}</div>", id, content)
}

pub fn to_xml_div_noclass_no_id(content: &String) -> String {
    format!("<div>{}</div>", content)
}

// content is what should be in the body, class are the css classes applied
pub fn to_xml_body(content: &String, class: &str) -> String {
    format!("<body class=\"{}\">{}</body>", class, content)
}

// create <img> tags. 'handle' refers to where the images are mounted.
pub fn to_xml_image(content: &String, handle: &'static str) -> String {
    format!("<img src=\"{}{}\" alt=\"image not found\" class=\"imgThread\">", handle, content)
}

pub fn to_xml_image_gallery(content: &String, handle: &'static str) -> String {
    format!("<img src=\"{}{}\" alt=\"image not found\" class=\"imgGallery\">", handle, content)
}

// The header is always the same, with the exception of the title, which depends on the page currently viewed.
pub fn retrieve_header(title: &str) -> String {
    format!("<head><title>{}</title>
	<meta charset=\"utf-8\">
	<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
	<link rel=\"stylesheet\" type=\"text/css\" href=\"/static/stylesheet.css\">
	</head>", title)
}

// The navigation bar is a hardcoded string since it is always the same.
pub fn retrieve_navigation_bar() -> String {
    "<div class=\"background-white\"><div class=\"bar\"><nav class=\"nav\"><span class=\"nav_span\"><a href=\"/\"><img id=\"logo\" class=\"logo\" width=\"32\" height=\"32\" src=\"/static/oboe.png\" alt=\"logo\"></a><a class=\"nav_entity underline--hover bold blue\" href=\"/\">Home</a><span class=\"nav_separator\">/</span><a class=\"nav_entity underline--hover\" href=\"/gallery\">Gallery</a><span class=\"nav_separator\">/</span><a class=\"nav_entity underline--hover\" href=\"/makethread\">New Thread</a></span></nav></div></div>".to_owned()
}

// imports the script to fetch new comments.
pub fn retrieve_js_import() -> &'static str {
    "<script src=\"/static/main.js\"></script>"
}

// a commented line with the last 'postid'. This is used by the script, else it would have to parse
// the whole html to find out what the last post is.
pub fn retrieve_post_counter(threadid: &i32) -> String {
    format!("<!--{}-->", metainfo::get_postid(threadid, false).unwrap())
}

// Takes an array of 'OP', builds a full HTML page from it. This makes up the index/titlepage.
pub fn xmlify_for_index(openings: Vec<OP>) -> String {
    let mut thread_content = String::new();

    // creation of "tiles" which contain user content
    for op in openings.iter() {
        let title = to_xml_link(&clickable_thread(op), &op.title);
        let title = to_xml_div_w_class(&title, "title");

        let poster = to_xml_div_w_class(&op.poster, "user");
        let threadid = to_xml_div_w_class(&format!("ID: {}", &op.threadid), "id");
        let time = to_xml_div_w_class(&op.time, "time");
        let date = to_xml_div_w_class(&op.date, "date");
        let thread_info = to_xml_div_noclass_no_id(&format!("{}{}{}{}", poster, threadid, time, date));

        let image = to_xml_image(&op.img, "/");
        let body = to_xml_paragraph(&op.body);
        let content = to_xml_div_w_class(&format!("{}{}", image, body), "content");

        let tile = to_xml_div_w_class(&format!("{}{}{}", title, thread_info, content), "tile");

        thread_content.push_str(tile.as_str());
    }

    //wrap all tiles into one division, "main__"
    thread_content = to_xml_div_w_class(&thread_content, "main__");

    format!("
        <!DOCTYPE html><html lang=\"en\" xml:lang=\"en\">{}<body class=\"background keep_space\">{}{}</body></html>", retrieve_header("Home"), retrieve_navigation_bar(), thread_content
    )
}

// Builds the full page of the gallery view of the site. Needs both all 'OP' and all 'Post' since
// this focuses on portraying all images currently in use.
pub fn xmlify_for_gallery(from_openings: Vec<OP>, from_posts: Vec<Post>) -> String {
    let mut images = Vec::new();
    for op in from_openings.iter() {
        images.push(&op.img);
    }
    for post in from_posts.iter() {
        images.push(&post.img);
    }

    // xmlified images
    let mut content = String::new();
    for image in images.iter() {
        content.push_str(to_xml_image_gallery(*image, "/").as_str());
    }

    let heading = to_xml_div_w_class(&"Image Gallery".to_owned(), "title");
    let spacer = to_xml_header3(&"".to_owned());

    let gallery = to_xml_div_w_class(&format!("{}{}{}", heading, spacer, content), "gallery");
    let gallery_in_tile = to_xml_div_w_class(&gallery, "main__");
    format!("
        <!DOCTYPE html><html lang=\"en\" xml:lang=\"en\">
        {}
        <body class=\"background keep_space\">
        {}
        {}
        </body>
        </html>", retrieve_header("Gallery"), retrieve_navigation_bar(), gallery_in_tile
    )
}

// Builds a HTML page for a full thread, with a opening post ('OP') and all its comments ('Post').
pub fn xmlify_for_indvthread(thread: Thread) -> String {

    let opening = &thread.op;
    let posts = thread.posts;
    // the opening is on a single tile
    let title = to_xml_link(&clickable_thread(&opening), &opening.title);
    let title = to_xml_div_w_class(&title, "title");

    let poster = to_xml_div_w_class(&opening.poster, "user");
    let threadid = to_xml_div_w_class(&format!("ID: {}", &opening.threadid), "id");
    let time = to_xml_div_w_class(&opening.time, "time");
    let date = to_xml_div_w_class(&opening.date, "date");
    let thread_info = to_xml_div_noclass_no_id(&format!("{}{}{}{}", poster, threadid, time, date));

    let image = to_xml_image(&opening.img, "/");
    let body = to_xml_paragraph(&opening.body);
    let content = to_xml_div_w_class(&format!("{}{}", image, body), "content");

    let opening_tile = to_xml_div_w_class(&format!("{}{}{}", title, thread_info, content), "tile");

    // all comments are put on a single tile, if there are any
    match posts {
        // if no comments are present, only the reply field make_post is shown
        None => {
            //empty division for comments added, so that AJAX may fill up with future comments
            let make_post = {
                to_xml_div_w_class(
                    &"<div class=\"title\">Comments</div><div id=\"commentSection\"></div><div class=\"comment\"><div class=\"title\">Make a Comment</div><form enctype=\"multipart/form-data\" method=\"post\" autocomplete=\"off\"><div class=\"form_space\"><input type=\"text\" name=\"poster\" placeholder=\"Identifier\"></div><textarea class=\"textarea--v\" name=\"body\" rows=\"5\" cols=\"50\" placeholder=\"Thread Content\"></textarea><div class=\"form_space\"><input type=\"file\" name=\"image\" accept=\"image/*\"></div><div class=\"button_space\"><input type=\"submit\" class=\"button button--blue\" value=\"make comment\"></div></form></div>".to_owned(),
                    "tile"
                )
            };
            let wrapper = to_xml_div_w_class(&format!("{}{}", opening_tile, make_post), "main__");
            return format!("
            <!DOCTYPE html><html lang=\"en\" xml:lang=\"en\">
            {}
            <body class=\"background keep_space\">
            {}
            {}
            {}
            {}
            </body>
            </html>", retrieve_header(&opening.title), retrieve_post_counter(&opening.threadid), retrieve_navigation_bar(), wrapper, retrieve_js_import()
            );
        },
        // else all comments are shown, plus the reply field
        Some(posts) => {

            // everything wrapped in the second division class "tile" that will contain all comments and the POST-form
            let mut comments_tile = String::from("<div class=\"title\">Comments</div>");
            // only the comments
            let mut comment_section = String::new();

            for post in posts.iter() {
                let poster = to_xml_div_w_class(&post.poster, "user");
                let postid = to_xml_div_w_class(&format!("ID: {}", &post.postid), "id");
                let time = to_xml_div_w_class(&post.time, "time");
                let date = to_xml_div_w_class(&post.date, "date");
                let thread_info = to_xml_div_noclass_no_id(&format!("{}{}{}{}", poster, postid, time, date));

                let image = to_xml_image(&post.img, "/");
                let body = to_xml_paragraph(&post.body);
                let content = to_xml_div_w_class(&format!("{}{}", image, body), "content");

                let single_comment = to_xml_div_w_class(&format!("{}{}", thread_info, content), "comment");

                &comment_section.push_str(single_comment.as_str());
            }
            // wrap final list of xml-compliant comments in div "commentSection" that is used by AJAX
            // to parse and add new comments
            comment_section = to_xml_div_w_id(&comment_section, "commentSection");

            comments_tile.push_str(&comment_section.as_str());
            // at the bottom, users can make posts. This is the form to make these.
            let make_post = {
                "<div class=\"comment\"><div class=\"title\">Make a Comment</div><form enctype=\"multipart/form-data\" method=\"post\" autocomplete=\"off\"><div class=\"form_space\"><input type=\"text\" name=\"poster\" placeholder=\"Identifier\"></div><textarea class=\"textarea--v\" name=\"body\" rows=\"5\" cols=\"50\" placeholder=\"Thread Content\"></textarea><div class=\"form_space\"><input type=\"file\" name=\"image\" accept=\"image/*\"></div><div class=\"button_space\"><input type=\"submit\" class=\"button button--blue\" value=\"make comment\"></div></form></div>"
            };
            &comments_tile.push_str(make_post);
            let comments_tile = to_xml_div_w_class(&comments_tile, "tile");

            let wrapper = to_xml_div_w_class(&format!("{}{}", opening_tile, comments_tile), "main__");

            return format!("
            <!DOCTYPE html><html lang=\"en\" xml:lang=\"en\">
            {}
            <body class=\"background keep_space\">
            {}
            {}
            {}
            {}
            </body>
            </html>", retrieve_header(&opening.title), retrieve_post_counter(&opening.threadid), retrieve_navigation_bar(), wrapper, retrieve_js_import()
            );
        }
    }
}