use oxc::parser::Parser;
use oxc::span::SourceType;
use oxc::allocator::Allocator;
use oxc::ast_visit::Visit;

pub trait Dataize<Return> {
    fn dataize(self) -> Return;
}

pub struct Ast<'a> {
    source: Source<'a>,
}

pub struct Source<'a> {
    pub text: &'a String,
    pub kind: SourceType,
}

impl<'a> Ast<'a> {
    pub fn new(source: Source<'a>) -> Self {
        Self { source }
    }
}

impl Ast<'_> {
    pub fn accept<F, T, Return>(&self, visitor_ctor: F) -> Return
        where F: Fn() -> T, T: for<'a> Visit<'a> + Dataize<Return>
    {
        let allocator = Allocator::default();
        let ast = Parser::new(&allocator, &self.source.text, self.source.kind).parse();

        let mut visitor = visitor_ctor();
        visitor.visit_program(&ast.program);
        visitor.dataize()
    }
}
