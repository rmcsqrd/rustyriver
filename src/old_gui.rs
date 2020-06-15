use web_view::*;
use std::fs;
use std::path::PathBuf;

pub fn draw_window(){
    
    // read from html file for web_view window
    let rel_html_path = PathBuf::from("./src/HTML/html_content.html");
    let abs_html_path = rel_html_path.canonicalize().unwrap(); // type = std::path::PathBuf
    let html_content = fs::read_to_string(abs_html_path).expect("Unable to open");
    
    // build the gui window
    let viewer = web_view::builder()  // https://docs.rs/web-view/0.4.0/web_view/struct.WebViewBuilder.html
        .title("rustyriver - River Permit Scraper")
        .content(Content::Html(html_content))
        .size(600, 800)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();

}
