#![allow(non_snake_case)]

use actix_files::Files;
use actix_web::{get, App, HttpResponse, HttpServer, web};
mod api;
mod templates;
use templates::index_page_prism::{render_index_page_page, IIndexPageData};
use templates::story_page_prism::{render_story_page_page};
use templates::story_preview_component_prism::IStoryItem;
use actix_web::middleware::Logger;

#[get("/")]
async fn index_page() -> HttpResponse {
    let results = api::stories::get_best_stories();
    if results.is_err() {
        println!("{:?}", results);
        return HttpResponse::InternalServerError().finish();
    }
    let mut stories: Vec<IStoryItem> = Vec::new();
    for &story_id in &results.unwrap()[..5] {
        // TODO parallelization with futures?
        let result = api::item::get_story_preview(story_id);
        if let Ok(story) = result {
            stories.push(story);
        } else {
            println!("{:?}", result);
            return HttpResponse::InternalServerError().finish();
        }
    }
    HttpResponse::Ok()
        .content_type("text/html")
        .body(render_index_page_page(IIndexPageData { stories }))
}

#[get("/i/{storyID}")]
async fn story_page(path: web::Path<(i32,)>) -> HttpResponse {
    let result = api::story::get_story(path.into_inner().0);
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
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index_page)
            .service(story_page)
            .service(Files::new("/", "public"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
