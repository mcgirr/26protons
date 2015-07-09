extern crate iron;
extern crate staticfile;
extern crate mount;

use std::path::Path;

use iron::Iron;
use staticfile::Static;
use mount::Mount;

fn main() {
    let mut mount = Mount::new();

    // Serve the files from the html dir at /
    mount.mount("/", Static::new(Path::new("www/")));
    
    // Serve the static blog files at /blog/
    mount.mount("/blog/", Static::new(Path::new("www/html/blog.html")));

    println!("Server running on http://localhost:3000/");

    Iron::new(mount).http("127.0.0.1:3000").unwrap();
}

