//TODO: Implement own parser
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Response {
    pub query: Query,
}

#[derive(Deserialize)]
pub struct Query {
    pub pages: Vec<Page>,
}

#[derive(Deserialize)]
pub struct Page {
    pub title: String,
    pub revisions: Option<Vec<Revision>>,
}

#[derive(Deserialize)]
pub struct Revision {
    pub slots: Slots,
}

#[derive(Deserialize)]
pub struct Slots {
    pub main: Main,
}

#[derive(Deserialize)]
pub struct Main {
    pub content: String,
}
