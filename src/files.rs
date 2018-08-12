extern crate walkdir;

use self::walkdir::{DirEntry, WalkDir};
use std::io::Error;
//use std::env;
use std::path::Path;

use rocket_contrib::{Template};

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    //name: Option<String>,
    items: Vec<&'static str>,
    upload: &'static str,
    delete: &'static str,
    download: &'static str,
}

#[get("/files")]
fn files() -> Template {
    let _ = file_list();
    let _ = walk_count();
    //format!("file list")
    Template::render("files", &TemplateContext{
        title: "File List",
        //name: Some(name),
        items: vec!["One", "Two", "Three"],
        upload: "upload",
        delete: "delete",
        download: "download",
    })
}

fn file_list() -> Result<(), Error> {
    //for entry in WalkDir::new("static/files/") {
    //    println!("{}", entry?.path().display());
    //}
    //
    //for entry in WalkDir::new("static/files/").into_iter().filter_map(|e| e.ok()) {
    //    println!("{}", entry.path().display());
    //}
    //
    let walker = WalkDir::new("static/files/").into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        println!("{}", entry?.path().display());
    }
    Ok(())
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

#[get("/delete")]
fn delete() -> &'static str {
    "delete file"
}

#[get("/download")]
fn download() -> &'static str {
    "download file"
}

#[post("/upload")]
fn upload() -> &'static str {
    "upload file"
}

fn walk<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let mut count = 0;
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.file_type().is_dir() {
            continue;
        }
        let name = entry.file_name();
        if let Some(s) = name.to_str() {
            if s.to_ascii_lowercase().ends_with(".jpg") {
                count += 1;
            }
        }
    }
    println!("{}", count);
    Ok(())
}

fn walk_count() -> Result<(), Error> {
    //walk(env::args().nth(1).unwrap())
    walk("static/files/")
}
