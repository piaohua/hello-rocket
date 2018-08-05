
#[get("/files")]
fn files() -> String {
    format!("file list")
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
