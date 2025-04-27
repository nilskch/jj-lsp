use tower_lsp_server::lsp_types::Range;

#[derive(Debug, Clone)]
pub struct Conflict {
    pub range: Range,
    pub title_range: Range,
    pub blocks: Vec<ChangeBlock>,
}

#[derive(Debug, Clone)]
pub struct ChangeBlock {
    pub title_range: Range,
    pub content: String,
}
