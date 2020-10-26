use std::fs::{File, OpenOptions};
use std::io::{prelude::*, Seek, SeekFrom};

use regex::Regex;

// each transaction between user and server is using an additional thread in rocket.rs,
// making it difficult to share current states. Instead of using shared memory structs,
// which may lead to race conditions, we opted for the safer option of writing to disk.

// in the current state it only saves the number of posts, a number used to name
// consecutive posts.

// creates a file containing meta-information for the specified 'threadid', and initiates the
// number of posts at 0
pub fn create_info_file(threadid: &i32) -> std::io::Result<()> {
    let mut file = File::create(format!("metainfo/{}",threadid))?;
    file.write_all(b"posts=0")?;
    Ok(())
}

// returns the next 'postid' for a specified 'threadid', and increments the value in the file if specified
pub fn get_postid(threadid: &i32, update: bool) -> std::io::Result<(i32)> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(format!("metainfo/{}",threadid))
        .unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // compiles the regex only the first time, then uses cashed version
    lazy_static!{
        static ref CACHED_REGEX: Regex = Regex::new("posts=(.*)").unwrap();
    }
    // essentially there is only one 'posts=<value>' entry, but the regex crate
    // only provides a iterator over all groups so we have to use a loop
    // of length 1
    for group in CACHED_REGEX.captures_iter(&mut content) {
        if update {
            let postid : i32 = group[1].parse::<i32>().unwrap() + 1;
            file.seek(SeekFrom::Start(0)).unwrap();
            file.write_all(format!("posts={}", postid).as_bytes())?;
            return Ok(postid);
        }
        else {
            return Ok(group[1].parse::<i32>().unwrap());
        }
    }
    // since the actual postid is not outside of the for clause, the compiler refuses
    // to compile unless something is directly returned, but this clause will never
    // occur
    Ok(-1)
}