pub trait StmtVisitor<R> {
    fn visit_header(&mut self, stmt: &HeaderStmt) -> R;
    fn visit_paragraph(&mut self, stmt: &ParagraphStmt) -> R;
    fn visit_list(&mut self, stmt: &ListStmt) -> R;
    fn visit_blockquote(&mut self, stmt: &BlockquoteStmt) -> R;
    fn visit_code(&mut self, stmt: &CodeStmt) -> R;
}

pub enum Stmt {
    Header(HeaderStmt),
    Paragraph(ParagraphStmt),
    List(ListStmt),
    Blockquote(BlockquoteStmt),
    Code(CodeStmt),
}

pub enum Inline {
    Text(String),
    Emphasis(Vec<Inline>),     // * / _
    Strong(Vec<Inline>),       // ** / __
    Code(String),              // ```
    Link(Vec<Inline>, String), // [link](url)
}

pub struct HeaderStmt {
    level: usize,
    content: Vec<Inline>,
}

pub struct ParagraphStmt {
    content: Vec<Inline>,
}

pub struct ListStmt {
    content: Vec<Inline>,
}

pub struct BlockquoteStmt {
    content: Vec<Inline>,
}

pub struct CodeStmt {
    content: Vec<Inline>,
}
