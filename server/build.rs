use std::{env, path::Path};

use actix_web_static_files::generate_resources;
use actix_web_static_files::resource_dir;

fn main() {

    let out_dir = "./src";
    let generated_filename = Path::new(&out_dir).join("generated.rs");

    generate_resources("./dist", None, generated_filename, "generate").unwrap();
}