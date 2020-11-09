use crate::api::request;
use crate::templates::story_page_prism::IStoryPageData;
use crate::templates::comment_component_prism::IComment;
use futures::future::{try_join_all};
use async_recursion::async_recursion;

pub async fn get_story(id: i32) -> Result<IStoryPageData, request::GetError> {
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
        return Ok(story);
    }
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