use tower_lsp_server::lsp_types::Range;

#[derive(Debug)]
pub struct Conflict {
    pub range: Range,
    pub title_range: Range,
    pub blocks: Vec<ChangeBlock>,
}

#[derive(Debug)]
pub struct ChangeBlock {
    pub title_range: Range,
    pub content: String,
}
