const TOP_STORIES: &str = "https://hacker-news.firebaseio.com/v0/topstories.json?print=pretty";
const NEW_STORIES: &str = "https://hacker-news.firebaseio.com/v0/newstories.json?print=pretty";
const BEST_STORIES: &str = "https://hacker-news.firebaseio.com/v0/beststories.json?print=pretty";
use crate::api::request;
use crate::api::item;
use futures::future::{try_join_all};
use crate::templates::story_preview_component_prism::IStoryItem;

pub enum StorySorting {
    Top, New, Best
}

pub async fn get_stories(sort: StorySorting) -> Result<Vec<IStoryItem>, request::GetError> {
    let url = match sort {
        StorySorting::Best => BEST_STORIES,
        StorySorting::Top => TOP_STORIES,
        StorySorting::New => NEW_STORIES
    };

    let result = request::make_json_get_request::<Vec<i32>>(url).await;
    if let Err(e) = result {
        return Err(e);
    }
    let story_ids = result.unwrap();
    let first_five_story_ids = &story_ids[..story_ids.len().min(10)];
    return try_join_all(
        first_five_story_ids.iter()
            .map(|id| item::get_story_preview(*id as i32))
    ).await;
}
