use dada_ir::{filename::Filename, item::Item};
use dada_parse::prelude::*;

#[salsa::memoized(in crate::Jar)]
pub fn check_filename(db: &dyn crate::Db, filename: Filename) {
    let items = filename.items(db);

    for &item in items {
        match item {
            Item::Function(function) => {
                function.parameters(db);
                function.syntax_tree(db);
            }
            Item::Class(class) => {
                class.fields(db);
            }
        }
    }
}