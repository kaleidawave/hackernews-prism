use isahc::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct IPost {
    pub by: String,
    pub id: i32
}

#[derive(Debug)]
enum GetPostError {
    Isahc(isahc::Error),
    SerdeJson(serde_json::Error),
}

impl From<isahc::Error> for GetPostError {
    fn from(error: isahc::Error) -> Self {
        GetPostError::Isahc(error)
    }
}

impl From<serde_json::Error> for GetPostError {
    fn from(error: serde_json::Error) -> Self {
        GetPostError::SerdeJson(error)
    }
}

fn get_post(post: i32) -> Result<IPost, GetPostError> {
    let mut response = isahc::get(format!("https://hacker-news.firebaseio.com/v0/item/{}.json", post))?;
    return Ok(response.json::<IPost>()?);
}

fn main() -> Result<(), GetPostError> {
    // Read the response body as text into a string and print it.
    print!("{:?}", get_post(121003)?);
    Ok(())
}