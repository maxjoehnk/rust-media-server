use library::{Track, Album};

#[derive(Clone, Debug, Serialize)]
pub struct Artist {
    pub id: Option<usize>,
    pub name: String
}