use std::error::Error;
use std::fs;
use std::path::PathBuf;
use actix_files::NamedFile;
use actix_web::{get, HttpRequest};

//todo: get this to work without having access to folders outside of /static
// #[get("/{filename:.*}")]
// async fn index(req: HttpRequest) -> actix_web::Result<NamedFile> {
//     let path: PathBuf = req.match_info().query("filename").parse().unwrap();
//     Ok(NamedFile::open(path)?)
// }