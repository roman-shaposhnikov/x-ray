use std::{ path::Path };

use oxc::span::SourceType;
use oxc::semantic::SemanticBuilder;
use oxc::ast::ast::{ Class, Function, TSImportType };
use oxc::ast_visit::{ Visit, walk };
use oxc::syntax::scope::ScopeFlags;

use crate::response::{ Fan };
use crate::shared::file::File;

use super::{ entity::Entity, ast::Ast };

pub struct Module<'a> {
    path: &'a Path,
}

impl Module<'_> {
    pub fn new(path: &Path) -> Module {
        Module { path }
    }
}

impl Entity for Module<'_> {
    fn fan(&self) -> Fan {
        let source_text = File::new(self.path).content();
        let source_type = SourceType::from_path(self.path).unwrap();
        let ast = Ast::with_source_type(&source_text, source_type);

        // let allocator = Allocator::default();
        // let ret = Parser::new(&allocator, &source_text, source_type).parse();
        // let program = ret.program;

        println!("AST:");
        println!("{program:#?}");

        let mut ast_pass = CountASTNodes::default();
        ast_pass.visit_program(&program);
        println!("{ast_pass:?}");
    }
}

#[derive(Debug, Default)]
struct CountASTNodes {
    functions: usize,
    classes: usize,
    ts_import_types: usize,
}

impl<'a> Visit<'a> for CountASTNodes {
    fn visit_function(&mut self, func: &Function<'a>, flags: ScopeFlags) {
        self.functions += 1;
        walk::walk_function(self, func, flags);
    }

    fn visit_class(&mut self, class: &Class<'a>) {
        self.classes += 1;
        dbg!(&class);
        walk::walk_class(self, class);
    }

    fn visit_ts_import_type(&mut self, ty: &TSImportType<'a>) {
        self.ts_import_types += 1;
        walk::walk_ts_import_type(self, ty);
    }
}
