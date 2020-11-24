use crate::api::request;
use crate::templates::story_page_prism::IStoryPageData;
use crate::templates::comment_component_prism::IComment;
use crate::templates::story_preview_component_prism::IStoryItem;
use crate::templates::user_page_prism::IUserData;
use futures::future::try_join_all;
use async_recursion::async_recursion;
use lru::LruCache;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref STORY_CACHE: Mutex<LruCache<i32, IStoryPageData>> = Mutex::new(
        LruCache::new(1000)
    );
    static ref STORY_PREVIEW_CACHE: Mutex<LruCache<i32, IStoryItem>> = Mutex::new(
        LruCache::new(1000)
    );
}

pub async fn get_story(
    id: i32, 
) -> Result<IStoryPageData, request::GetError> {
    if let Some(cached_story) = STORY_CACHE.lock().unwrap().get(&id) {
        return Ok(cached_story.clone());
    }
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let result = request::make_json_get_request::<IStoryPageData>(&url).await;
    if result.is_err() {
        return result;
    } else {
        let mut story = result.unwrap();
        let comment_ids = &story.kids[..story.kids.len().min(3)];
        let comments = try_join_all(
            comment_ids.iter()
                .map(|id| get_comment(*id as i32, 3))
        ).await;
        if comments.is_err() {
            println!("{:?}", comments);
            // Ignore error at this point in time
        } else {
            story.comments = comments.unwrap();
        }
        // TODO could spawn thread as to not block while adding to cache ...?
        STORY_CACHE.lock().unwrap().put(id, story.clone());
        return Ok(story);
    }
} 

// Same as get_story but does not add comments
pub async fn get_story_preview(
    id: i32, 
) -> Result<IStoryItem, request::GetError> {
    if let Some(cached_story) = STORY_PREVIEW_CACHE.lock().unwrap().get(&id) {
        return Ok(cached_story.clone());
    }
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let story_preview = request::make_json_get_request::<IStoryItem>(&url).await;
    // TODO could spawn thread as to not block while adding to cache ...?
    if let Ok(valid_story) = &story_preview {
        STORY_PREVIEW_CACHE.lock().unwrap().put(id, valid_story.clone());
    }
    return story_preview;  
} 

#[async_recursion]
pub async fn get_comment(id: i32, depth: i32) -> Result<IComment, request::GetError> { 
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let result = request::make_json_get_request::<IComment>(&url).await;
    if result.is_err() {
        return result;
    } else {
        let mut comment = result.unwrap();
        if depth > 0 {
            let sub_comment_ids = &comment.kids[..comment.kids.len().min(3)];
            let sub_comments = try_join_all(
                sub_comment_ids.iter()
                    .map(|id| get_comment(*id as i32, depth - 1))
            ).await;
            if sub_comments.is_err() {
                println!("{:?}", sub_comments);
                // Ignore error at this point in time
            } else {
                comment.subComments = sub_comments.unwrap();
            }
        }
        return Ok(comment);
    }
}

pub async fn get_user(
    user_id: &String, 
) -> Result<IUserData, request::GetError> {
    let url = format!("https://hacker-news.firebaseio.com/v0/user/{}.json", user_id);
    let result = request::make_json_get_request::<IUserData>(&url).await;
    if let Err(e) = result {
        return Err(e);
    }
    let mut user = result.unwrap();
    let first_story_ids = &user.kids[..user.kids.len().min(10)];
    let story_futures = first_story_ids.iter().map(|id| get_story_preview(*id as i32));
    // TODO stories do not appear ???
    match try_join_all(story_futures).await {
        Ok(stories) => {user.stories = stories},
        Err(err) => {println!("Getting user posts {:?}", err)}
    }
    return Ok(user);
}