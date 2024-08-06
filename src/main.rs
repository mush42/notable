#![windows_subsystem = "windows"]

use directories::UserDirs;
use open;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use warp::{Buf, Filter};

const WIKI_HTML: &str = include_str!("../html/wiki.html");

fn extract_html_to_home() -> std::io::Result<PathBuf> {
    let user_dirs = UserDirs::new().expect("Unable to get user directories");
    let documents_dir = user_dirs
        .document_dir()
        .expect("Unable to get Documents directory");
    let html_path = documents_dir.join("wiki.html");

    if !html_path.exists() {
        let mut file = fs::File::create(&html_path)?;
        file.write_all(WIKI_HTML.as_bytes())?;
    }

    Ok(html_path)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    let html_path = extract_html_to_home()?;

    // Start the server in a separate task
    let server = tokio::spawn(async move {
        let wiki_html_route = warp::filters::method::get().and(warp::fs::file(html_path.clone()));

        let dav_header_route = warp::filters::method::options()
            .map(warp::reply)
            .map(|reply| warp::reply::with_header(reply, "dav", "1"));

        let save_wiki_route =
            warp::filters::method::put()
                .and(warp::body::bytes())
                .map(move |body: bytes::Bytes| {
                    let mut file = std::fs::File::create(html_path.clone()).unwrap();
                    std::io::copy(&mut body.reader(), &mut file).unwrap();
                    warp::reply::with_status("File updated", warp::http::StatusCode::OK)
                });

        let routes = dav_header_route.or(save_wiki_route).or(wiki_html_route);

        warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    });

    // Open the file in the default browser
    open::that("http://127.0.0.1:3030").expect("Failed to open browser");

    server.await.expect("Server failed");

    Ok(())
}
