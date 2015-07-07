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
    mount.mount("/", Static::new(Path::new("html/")));
    
    // Serve the static file docs at /doc/
    //mount.mount("/doc/", Static::new(Path::new("target/doc/staticfile/")));
    
    // Serve the source code at /src/
    //mount.mount("/src/", Static::new(Path::new("target/doc/src/staticfile/lib.rs.html")));

    println!("Doc server running on http://localhost:3000/");

    Iron::new(mount).http("127.0.0.1:3000").unwrap();
}

