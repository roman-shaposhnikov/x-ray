use std::{ path::Path };

use oxc::span::SourceType;

use crate::project::fan_store::FanStore;
use crate::shared::file::File;

use super::ast::{ Ast, Source };

use oxc::semantic::SemanticBuilder;
use oxc::ast::ast::{ ImportDeclaration };
use oxc::ast_visit::{ Visit, walk };

pub struct Module<'a> {
    path: &'a Path,
    fan_store: &'a mut FanStore,
}

impl<'a> Module<'a> {
    pub fn new(path: &'a Path, fan_store: &'a mut FanStore) -> Self {
        Self { path, fan_store }
    }
}

impl Module<'_> {
    pub fn collect_dependencies(self) {
        let text = File::new(self.path).content();
        let kind = SourceType::from_path(self.path).unwrap();
        let source = Source { text: &text, kind };
        let ast = Ast::new(source);
        let deps = ast.accept(|| ModuleFan::new());

        self.fan_store.add_module_dependency(mod_id, dep_id);
    }
}

struct ModuleFan {
    deps: Vec<String>,
}

impl ModuleFan {
    fn new() -> Self {
        Self { deps: vec![] }
    }
}

impl<'a> Visit<'a> for ModuleFan {
    fn visit_import_declaration(&mut self, it: &ImportDeclaration<'a>) {
        dbg!(&it.source);
        walk::walk_import_declaration(self, it);
    }
}
