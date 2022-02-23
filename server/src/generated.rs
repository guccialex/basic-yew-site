#[allow(clippy::unreadable_literal)] pub fn generate() -> ::std::collections::HashMap<&'static str, actix_web_static_files::Resource> {
use actix_web_static_files::Resource;
let mut result = ::std::collections::HashMap::new();
{
let data = include_bytes!("/home/mucci/Documents/coding/basic-yew-site/server/dist/index-e66165725551ab5e.js");
let modified = 1645602006;
let mime_type = "application/javascript";
result.insert("index-e66165725551ab5e.js", Resource { data, modified, mime_type });
}
{
let data = include_bytes!("/home/mucci/Documents/coding/basic-yew-site/server/dist/static/favicon.ico");
let modified = 1645602006;
let mime_type = "image/x-icon";
result.insert("static/favicon.ico", Resource { data, modified, mime_type });
}
{
let data = include_bytes!("/home/mucci/Documents/coding/basic-yew-site/server/dist/static/images/1.png");
let modified = 1645602006;
let mime_type = "image/png";
result.insert("static/images/1.png", Resource { data, modified, mime_type });
}
{
let data = include_bytes!("/home/mucci/Documents/coding/basic-yew-site/server/dist/index.html");
let modified = 1645602006;
let mime_type = "text/html";
result.insert("index.html", Resource { data, modified, mime_type });
}
{
let data = include_bytes!("/home/mucci/Documents/coding/basic-yew-site/server/dist/index-e66165725551ab5e_bg.wasm");
let modified = 1645602006;
let mime_type = "application/wasm";
result.insert("index-e66165725551ab5e_bg.wasm", Resource { data, modified, mime_type });
}
result
}
