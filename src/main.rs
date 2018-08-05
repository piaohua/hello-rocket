#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;

mod files;

use rocket::Request;
//use rocket::response::Redirect;
use rocket_contrib::{Template};

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

//#[get("/")]
//fn index() -> Redirect {
//    Redirect::to("/hello/Unknown")
//}

//#[get("/hello/<name>")]
//fn hello(name: String) -> String {
//    name
//}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("error/404", &map)
}

#[get("/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![
               files::files, files::delete,
               files::download, files::upload,
               index, static_files])
        .attach(Template::fairing())
        .catch(catchers![not_found])
}

fn main() {
    rocket().launch();
}
