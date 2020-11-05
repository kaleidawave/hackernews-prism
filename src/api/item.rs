use crate::api::request;
use crate::templates::story_preview_component_prism::IStoryItem;

pub fn get_story_preview(id: i32) -> Result<IStoryItem, request::GetError> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    return request::make_json_get_request::<IStoryItem>(&url);  
} 