use oxc::parser::Parser;
use oxc::span::SourceType;
use oxc::allocator::Allocator;
use oxc::semantic::SemanticBuilder;
use oxc::ast::ast::{ ImportDeclaration };
use oxc::ast_visit::{ Visit, walk };

pub struct Source<'a> {
    text: &'a String,
    kind: SourceType,
}

pub struct Ast<'a> {
    source: &'a String,
    source_type: SourceType,
}

impl Ast<'_> {
    pub fn with_default_source_type<'a>(source: &'a String) -> Ast<'a> {
        Ast::new(source, SourceType::default())
    }

    pub fn new<'a>(source: &'a String, source_type: SourceType) -> Ast<'a> {
        Ast { source, source_type }
    }
}

impl Ast<'_> {
    fn iter(&self) {
        let allocator = Allocator::default();
        let ast = Parser::new(&allocator, &self.source, self.source_type).parse();

        let mut ast_pass = ModuleFan::new();
        ast_pass.visit_program(&ast.program);
    }
}

struct ModuleFan {
    out: u32,
}

impl ModuleFan {
    fn new() -> Self {
        Self { out: 0 }
    }
}

impl<'a> Visit<'a> for ModuleFan {
    fn visit_import_declaration(&mut self, it: &ImportDeclaration<'a>) {
        dbg!(&it.source);
        self.out += 1;
        walk::walk_import_declaration(self, it);
    }
}
