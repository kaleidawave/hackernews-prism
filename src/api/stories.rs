const TOP_STORIES: &str = "https://hacker-news.firebaseio.com/v0/topstories.json?print=pretty";
const NEW_STORIES: &str = "https://hacker-news.firebaseio.com/v0/newstories.json?print=pretty";
const BEST_STORIES: &str = "https://hacker-news.firebaseio.com/v0/beststories.json?print=pretty";
use crate::api::request;
// TODO do the join in this mod

pub fn get_best_stories() -> Result<Vec<i32>, request::GetError> {
    return request::make_json_get_request::<Vec<i32>>(BEST_STORIES);  
} 

pub fn get_top_stories() -> Result<Vec<i32>, request::GetError> {
    return request::make_json_get_request::<Vec<i32>>(TOP_STORIES);  
} 

pub fn get_new_stories() -> Result<Vec<i32>, request::GetError> {
    return request::make_json_get_request::<Vec<i32>>(NEW_STORIES);  
} 