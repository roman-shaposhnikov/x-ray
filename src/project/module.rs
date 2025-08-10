use std::{ path::Path };

use oxc::span::SourceType;

use crate::project::fan_store::FanStore;
use crate::shared::file::File;

use super::ast::{ Ast, Source, Dataize };

use oxc::ast::ast::{ ImportDeclaration };
use oxc::ast_visit::{ walk, Visit };

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
    pub fn collect(self) {
        let text = File::new(self.path).content();
        let kind = SourceType::from_path(self.path).unwrap();
        let source = Source { text: &text, kind };
        let ast = Ast::new(source);
        let deps = ast.accept(|| ModuleFan::new());

        deps.iter().for_each(|id| {
            self.fan_store.add_module_dependency(mod_id, id.to_string());
        });
    }
}

type Dependency = String;

struct ModuleFan {
    deps: Vec<Dependency>,
}

impl ModuleFan {
    fn new() -> Self {
        Self { deps: vec![] }
    }
}

impl Dataize<Vec<Dependency>> for ModuleFan {
    fn dataize(self) -> Vec<Dependency> {
        return self.deps;
    }
}

impl<'a> Visit<'a> for ModuleFan {
    fn visit_import_declaration(&mut self, it: &ImportDeclaration<'a>) {
        dbg!(&it.source);
        walk::walk_import_declaration(self, it);
    }
}
