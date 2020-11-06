use crate::api::request;
use crate::templates::story_page_prism::IStoryPageData;
use crate::templates::comment_component_prism::IComment;

pub fn get_story(id: i32) -> Result<IStoryPageData, request::GetError> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let result = request::make_json_get_request::<IStoryPageData>(&url);
    if result.is_err() {
        return result;
    } else {
        let mut story = result.unwrap();
        let mut comments = Vec::new();
        for &comment_id in &story.kids[..story.kids.len().min(3)] {
            if let Ok(comment) = get_comment(comment_id as i32, 4) {
                comments.push(comment);
            }
        }
        story.comments = comments;
        return Ok(story);
    }
} 

pub fn get_comment(id: i32, depth: i32) -> Result<IComment, request::GetError> { 
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let result = request::make_json_get_request::<IComment>(&url);
    if result.is_err() {
        return result;
    } else {
        let mut comment = result.unwrap();
        if depth > 0 {
            let mut subComments = Vec::new();
            for &sub_comment_id in &comment.kids[..comment.kids.len().min(3)] {
                if let Ok(comment) = get_comment(sub_comment_id as i32, depth - 1) {
                    subComments.push(comment);
                }
            }
            comment.subComments = subComments;
        }
        return Ok(comment);
    }
}