#![allow(non_snake_case)]

use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, middleware};
mod api;
use api::stories::{get_stories, StorySorting};
mod templates;
use actix_web::middleware::Logger;
use templates::index_page_prism::{render_index_page_page, IIndexPageData};
use templates::story_page_prism::render_story_page_page;

async fn best_page () -> HttpResponse {
    let result = get_stories(StorySorting::Best).await;
    if let Ok(stories) = result {
        HttpResponse::Ok()
            .content_type("text/html")
            .body(render_index_page_page(IIndexPageData { stories }))
    } else {
        println!("{:?} getting best stories", result);
        return HttpResponse::InternalServerError().finish();
    }
}

#[get("/new")]
async fn new_page () -> HttpResponse {
    let result = get_stories(StorySorting::New).await;
    if let Ok(stories) = result {
        HttpResponse::Ok()
            .content_type("text/html")
            .body(render_index_page_page(IIndexPageData { stories }))
    } else {
        println!("{:?} getting new stories", result);
        return HttpResponse::InternalServerError().finish();
    }
}

#[get("/top")]
async fn top_page() -> HttpResponse {
    let result = get_stories(StorySorting::Top).await;
    if let Ok(stories) = result {
        HttpResponse::Ok()
            .content_type("text/html")
            .body(render_index_page_page(IIndexPageData { stories }))
    } else {
        println!("{:?} getting top stories", result);
        return HttpResponse::InternalServerError().finish();
    }
}

#[get("/i/{storyID}")]
async fn story_page(path: web::Path<(i32,)>) -> HttpResponse {
    let result = api::story::get_story(path.into_inner().0).await;
    if result.is_err() {
        println!("{:?}", result);
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok()
        .content_type("text/html")
        .body(render_story_page_page(result.unwrap()))
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
