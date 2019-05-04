#![feature(proc_macro_hygiene, decl_macro)]

use std::io;
use std::fs;
use std::path;

use std::fmt;
use std::borrow::Cow;

use rand::{self, Rng};

/// Table to retrieve base62 values from.
const BASE62: &'static [u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// A _probably_ unique paste ID.
pub struct FileID<'a>(Cow<'a, str>);

impl<'a> FileID<'a> {
    /// Generate a _probably_ unique ID with `size` characters. For readability,
    /// the characters used are from the sets [0-9], [A-Z], [a-z]. The
    /// probability of a collision depends on the value of `size` and the number
    /// of IDs generated thus far.
    pub fn new(size: usize) -> FileID<'static> {
        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }

        FileID(Cow::Owned(id))
    }
}

impl<'a> fmt::Display for FileID<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[macro_use]
extern crate rocket;
extern crate backend;

use backend::ImageCompare;

#[post("/", data = "<image>")]
fn upload(image: Data) -> io::Result<String> {
    let upload_dir = Path::new("./uploads");

    if !upload_dir.exists() {
        fs::create_dir_all(upload_dir);
    }

    image::
}

fn diff_images(repo: String, branch: String, commit: String) {
    let ic = ImageCompare::compare_images("./test-assets/test-1.png", "./test-assets/test-2.png")
        .unwrap();

    ic.save("./test-output/").unwrap();
}

fn main() {}
