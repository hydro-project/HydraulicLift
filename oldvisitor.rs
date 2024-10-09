use std::collections::HashSet;

use syn::{visit::{self, Visit}, Ident, Pat, PatIdent};

use crate::oldsegment::RawSegment;

pub struct LocalVisitor {
    idents: HashSet<Ident>,
}

impl LocalVisitor {
    pub fn visit_raw_segment(idents: Vec<Ident>, raw_segment: RawSegment) -> Vec<Ident> {
        let mut visitor = LocalVisitor {
            idents: idents.into_iter().collect(),
        };
        visitor.visit_block(&raw_segment.into_block());
        visitor.idents.into_iter().collect()
    }
}

impl<'ast> Visit<'ast> for LocalVisitor {
    // visit statements in reverse
    fn visit_block(&mut self, block: &'ast syn::Block) {
        for stmt in block.stmts.iter().rev() {
            self.visit_stmt(stmt);
        }
    }

    // Add visited locals
    fn visit_expr_path(&mut self, exprpath: &'ast syn::ExprPath) {
        if let Some(ident) = exprpath.path.get_ident() {
            self.idents.insert(ident.clone());
        }

        // visit nested?
        visit::visit_expr_path(self, exprpath);
    }

    fn visit_local(&mut self, local: &'ast syn::Local) {
        if let Pat::Ident(PatIdent { ident, .. }) = &local.pat {
            self.idents.remove(ident);
        }

        // visit nested?
        visit::visit_local(self, local);
    }
}
