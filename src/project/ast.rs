use oxc::parser::Parser;
use oxc::span::SourceType;
use oxc::allocator::Allocator;

pub struct Ast<'a> {
    source: &'a String,
    source_type: SourceType,
}

impl Ast<'_> {
    pub fn new<'a>(source: &'a String) -> Ast<'a> {
        Ast { source, source_type: SourceType::default() }
    }

    pub fn with_source_type<'a>(source: &'a String, source_type: SourceType) -> Ast<'a> {
        Ast { source, source_type }
    }
}

impl Ast<'_> {
    fn parse(&self) {
        let allocator = Allocator::default();
        let ret = Parser::new(&allocator, &self.source, self.source_type).parse();
        let program = ret.program;
    }
}
