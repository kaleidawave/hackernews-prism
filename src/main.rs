#![allow(non_snake_case)]

use actix_files::Files;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer};
mod api;
use api::stories::{get_stories, StorySorting};
mod templates;
use actix_web::middleware::Logger;
use std::time::Instant;
use templates::index_page_prism::{render_index_page_page, IIndexPageData};
use templates::story_page_prism::{render_story_page_page, render_story_page_component_content};
use templates::user_page_prism::render_user_page_page;
use templates::story_preview_component_prism::render_story_preview_component_content;

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
async fn story_page(web::Path((story_id,)): web::Path<(i32,)>) -> HttpResponse {
    let now = Instant::now();
    let result = api::items::get_story(story_id).await;
    println!(
        "Getting full story {} took {:.2} ns",
        story_id,
        now.elapsed().as_nanos()
    );
    if let Ok(story) = result {
        HttpResponse::Ok()
            .content_type("text/html")
            .body(render_story_page_page(&story))
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[get("/u/{userID}")]
async fn user_page(web::Path((user_id,)): web::Path<(String,)>) -> HttpResponse {
    let now = Instant::now();
    let result = api::items::get_user(&user_id).await;
    println!(
        "Getting full user {} took {:.2} ns",
        user_id,
        now.elapsed().as_nanos()
    );
    if result.is_err() {
        println!("{:?}", result);
        return HttpResponse::InternalServerError().finish();
    }
    return HttpResponse::Ok()
        .content_type("text/html")
        .body(render_user_page_page(&result.unwrap()));
}

// Single story preview
#[get("/story-preview/{storyID}")]
async fn single_story_component(web::Path((story_id,)): web::Path<(i32,)>) -> HttpResponse {
    let result = api::items::get_story_preview(story_id).await;
    if let Ok(data) = result {
        HttpResponse::Ok()
            .content_type("text/html")
            .body(render_story_preview_component_content(&data))
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

// Single story page
#[get("/story/{storyID}")]
async fn single_story_page(web::Path((story_id,)): web::Path<(i32,)>) -> HttpResponse {
    let result = api::items::get_story(story_id).await;
    if let Ok(data) = result {
        HttpResponse::Ok()
            .content_type("text/html")
            .body(render_story_page_component_content(&data))
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    println!("Running Hackernews on http://localhost");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::Compress::default())
            .route("/", web::get().to(best_page))
            .route("/best", web::get().to(best_page))
            .service(new_page)
            .service(top_page)
            .service(story_page)
            .service(user_page)
            .service(single_story_component)
            .service(single_story_page)
            .service(Files::new("/", "public"))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
