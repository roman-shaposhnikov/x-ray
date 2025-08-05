use oxc::parser::Parser;
use oxc::span::SourceType;
use oxc::allocator::Allocator;
use oxc::ast_visit::Visit;

pub struct Ast<'a> {
    source: Source<'a>,
}

pub struct Source<'a> {
    pub text: &'a String,
    pub kind: SourceType,
}

impl Ast<'_> {
    pub fn new(source: Source) -> Ast {
        Ast { source }
    }
}

pub type VisitorCtor<'a, Return> = fn() -> (impl SuperV<'a, Return>);

impl Ast<'_> {
    pub fn accept<'a, Return>(&self, visitor: VisitorCtor<'a, Return>) -> Return {
        let allocator = Allocator::default();
        let ast = Parser::new(&allocator, &self.source.text, self.source.kind).parse();

        let mut visitor = visitor();
        visitor.visit_program(&ast.program);
        visitor.dataize()
    }
}

trait SuperV<'a, Return>: Visit<'a> {
    fn dataize() -> Return;
}
