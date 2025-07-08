use serde::Serialize;


#[derive(Serialize, Debug, Clone)]
pub struct Edge {
    pub bottom: String,
    pub left: String,
    pub right: String,
    pub top: String,
}


