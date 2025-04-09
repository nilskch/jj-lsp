use tower_lsp_server::lsp_types::Range;

#[derive(Debug)]
pub struct Conflict {
    pub _blocks: Vec<ChangeBlock>,
}

#[derive(Debug)]
pub struct ChangeBlock {
    pub _range: Range,
    pub _content: String,
}
