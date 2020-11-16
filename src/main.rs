#![allow(non_snake_case)]

use actix_files::Files;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer};
mod api;
use api::stories::{get_stories, StorySorting};
mod templates;
use actix_web::middleware::Logger;
use std::time::Instant;
use templates::index_page_prism::{render_index_page_page, IIndexPageData};
use templates::story_page_prism::render_story_page_page;

async fn best_page() -> HttpResponse {
    let now = Instant::now();
    let result = get_stories(StorySorting::Best).await;
    println!(
        "Getting best stories took {:.2} ns",
        now.elapsed().as_nanos()
    );
    if let Ok(stories) = result {
        let page = render_index_page_page(&IIndexPageData { stories });
        return HttpResponse::Ok().content_type("text/html").body(page);
    } else {
        println!("{:?} getting best stories", result);
        return HttpResponse::InternalServerError().finish();
    }
}

#[get("/new")]
async fn new_page() -> HttpResponse {
    let result = get_stories(StorySorting::New).await;
    if let Ok(stories) = result {
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(render_index_page_page(&IIndexPageData { stories }));
    } else {
        println!("{:?} getting new stories", result);
        return HttpResponse::InternalServerError().finish();
    }
}

#[get("/top")]
async fn top_page() -> HttpResponse {
    let result = get_stories(StorySorting::Top).await;
    if let Ok(stories) = result {
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(render_index_page_page(&IIndexPageData { stories }));
    } else {
        println!("{:?} getting top stories", result);
        return HttpResponse::InternalServerError().finish();
    }
}

#[get("/i/{storyID}")]
async fn story_page(path: web::Path<(i32,)>) -> HttpResponse {
    let now = Instant::now();
    let id = path.into_inner().0;
    let result = api::items::get_story(id).await;
    println!(
        "Getting full story {} took {:.2} ns",
        id,
        now.elapsed().as_nanos()
    );
    if result.is_err() {
        println!("{:?}", result);
        return HttpResponse::InternalServerError().finish();
    }
    return HttpResponse::Ok()
        .content_type("text/html")
        .body(render_story_page_page(&result.unwrap()));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    println!("Running Hackernews on http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::Compress::default())
            .route("/", web::get().to(best_page))
            .route("/best", web::get().to(best_page))
            .service(new_page)
            .service(top_page)
            .service(story_page)
            .service(Files::new("/", "public"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
