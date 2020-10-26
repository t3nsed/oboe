<p align="center"><img src="/static/oboe.png" width="150"></a></p> 

# Oboe

A simple imageboard software project using rust's rocket.rs and PostgreSQL.

## Notice

This is a project done for a web-development course. It is meant to show skills  obtained in the course, and is not made primarily with production use in mind. All the common issues with SQL injection and XSS will work because there is very little verification/validation done.


## Building

- make sure you have rust installed:
    - `curl https://sh.rustup.rs -sSf | sh`
    - if you already have rust installed, make sure it is up to date: `rustup update && cargo update`
    - add `~/.cargo/bin/` to `$PATH`
- make sure to restart your terminal, since `$PATH` won't get reloaded instantly
- change the toolchain to nightly:

    - `rustup default nightly`

- set up a PostgreSQL database. You must change the environment variable of `SQL_URL` and make sure the the user/role is able to read/write all tables. The variable should be in the form of `postgresql://username:password@ip:port/database` 
    - we only need two tables `threads` and `posts`. Look at `SQL.md` for SQL commands.
    - make sure `pg_hba.conf` lets the user access the DB. The postgres driver is of type `host`, (`local` is default)
- compile the project (must be in root directory of `webserver`):
    - `cargo run --release` (there might be some warnings due to non-standard formatting practices)
    - or `cargo build` if you just want to see if it compiles, and do not need a optimized binary
- by default, rocket serves <a href=localhost:8000> localhost on port 8000</a>.

## Examples

<p align="center"><img src="/.promo/HelloWorld.png" width="1000"></a></p>
<p align="center"><img src="/.promo/Gallery.png" width="1000"></a></p> 
<p align="center"><img src="/.promo/NewThread.png" width="1000"></a></p> 


## Dependencies

All dependencies are automatically downloaded if you run `cargo <build/test/run` instead if `rustc <file>`. A list of dependencies and their versions can be found in `Cargo.toml`, all licensed under MIT. The top-level dependencies are:

* rocket - Sergio Benitez
* rocket-multipart-form-data - Magic Len
* postgres - Steven Fackler
* rand - Rust maintainers
* chrono - Brandon W Maister, Kang Seonghoon
* regex - Rust maintainers 
* md5 - Ivan Ukhov et al.
* lazy_static - Marvin Löbel
* serde, serde_json - David Tolnay, Erick Tryzelaar
