#[allow(clippy::unreadable_literal)] pub fn generate() -> ::std::collections::HashMap<&'static str, actix_web_static_files::Resource> {
use actix_web_static_files::Resource;
let mut result = ::std::collections::HashMap::new();
{
let data = include_bytes!("/home/mucci/Documents/coding/basic-yew-site/server/dist/index-cd1cef1443c4550f_bg.wasm");
let modified = 1644123181;
let mime_type = "application/wasm";
result.insert("index-cd1cef1443c4550f_bg.wasm", Resource { data, modified, mime_type });
}
{
let data = include_bytes!("/home/mucci/Documents/coding/basic-yew-site/server/dist/index-cd1cef1443c4550f.js");
let modified = 1644123181;
let mime_type = "application/javascript";
result.insert("index-cd1cef1443c4550f.js", Resource { data, modified, mime_type });
}
{
let data = include_bytes!("/home/mucci/Documents/coding/basic-yew-site/server/dist/static/favicon.ico");
let modified = 1644123181;
let mime_type = "image/x-icon";
result.insert("static/favicon.ico", Resource { data, modified, mime_type });
}
{
let data = include_bytes!("/home/mucci/Documents/coding/basic-yew-site/server/dist/index.html");
let modified = 1644123181;
let mime_type = "text/html";
result.insert("index.html", Resource { data, modified, mime_type });
}
result
}
