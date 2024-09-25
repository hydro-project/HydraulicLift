#![feature(box_patterns)]

use std::{collections::HashSet, convert::identity, hash::Hash, mem::take, path::Iter};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_quote, parse_str, punctuated::Punctuated, visit::{self, Visit}, Block, Expr, ExprAwait, ExprClosure, ExprPath, ExprTuple, Ident, Local, LocalInit, Pat, PatIdent, PatPath, Path, Stmt
};

enum RawSegment {
    Await(Vec<Stmt>),
    End(Vec<Stmt>),
}

impl RawSegment {
    fn iter(&self) -> std::slice::Iter<'_, Stmt> {
        let v = match self {
            Self::Await(v) => v,
            Self::End(v) => v,
        };
        v.iter()
    }

    fn into_segment(self, inputs: Vec<Ident>, outputs: Vec<Ident>) -> Segment {
        match self {
            Self::Await(v) => Segment::Await(inputs, v, outputs),
            Self::End(v) => Segment::End(inputs, v),
        }
    }
}

enum Segment {
    Await(Vec<Ident>, Vec<Stmt>, Vec<Ident>),
    End(Vec<Ident>, Vec<Stmt>),
}

impl ToTokens for Segment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::Await(inputs, statements, outputs) => quote! {
                poll_future(
                    async |#(#inputs),*| {
                        #(#statements)*
                        (#(#outputs),*)
                    }
                ); // ; should be ->
            },
            Self::End(inputs, statements) => quote! {
                map(|#(#inputs),*| {
                    #(#statements)*
                });
            },
        });
    }
}

// match `let _ = expr.await;`
fn is_await(stmt: &Stmt) -> bool {
    matches!(
        stmt,
        &Stmt::Local(Local {
            init:
                Option::Some(LocalInit {
                    expr: box Expr::Await(_),
                    ..
                }),
            ..
        })
    )
}

fn register_statement_locals(mut locals: HashSet<Ident>, stmt: &Stmt) -> HashSet<Ident> {
    // the local defined this statement
    if let Some(defined_local) = get_defined_local(stmt) {
        locals.remove(&defined_local); // TODO: could be many!! need to visit
    }
    // the locals needed by this statement
    let needed_locals = get_needed_locals(stmt);
    locals.extend(needed_locals);

    locals
}

fn get_defined_local(stmt: &Stmt) -> Option<Ident> {
    match stmt {
        Stmt::Local(Local {
            pat: Pat::Ident(PatIdent { ident, .. }),
            ..
        }) => Some(ident.clone()),
        _ => None,
    }
}

struct LocalVisitor {
    idents: HashSet<Ident>,
}

impl LocalVisitor {
    fn new() -> LocalVisitor {
        LocalVisitor {
            idents: HashSet::new(),
        }
    }
}

impl<'ast> Visit<'ast> for LocalVisitor {
    fn visit_expr_path(&mut self, p: &'ast syn::ExprPath) {
        if let Some(ident) = p.path.get_ident() {
            self.idents.insert(ident.clone());
        }

        // visit nested?
        visit::visit_expr_path(self, p);
    }
}

fn get_needed_locals(stmt: &Stmt) -> HashSet<Ident> {
    let mut visitor = LocalVisitor::new();
    visitor.visit_stmt(stmt);
    visitor.idents
}

fn main() {
    let block: Block = parse_str(
        "
{
    let x = 1+1;
    let b = a.await;
    let y = 1+2;
    let c = b.await;
    let z = x + y;
    let d = c.await;
    if z + 1 > 3 
    {
        let unused = x + 1;
        let asdf = unused + 1;
    }

    let e = c.await;
    z
}
",
    )
    .unwrap();

    let stmts = block.stmts;

    // vector of segments
    let mut raw_segs = Vec::new();
    // each segment is a sequence of lines split by an await
    let mut cur_segment = Vec::new();

    // split stmts by await
    for stmt in stmts.into_iter() {
        let stmt_is_await = is_await(&stmt);
        cur_segment.push(stmt);
        if stmt_is_await {
            // create new empty segment
            raw_segs.push(RawSegment::Await(cur_segment));
            cur_segment = Vec::new();
        }
    }
    raw_segs.push(RawSegment::End(cur_segment));

    // for each segment, the set of local variables which need to be passed on
    let segments = raw_segs
        .into_iter()
        .rev() // walk backwards through segments
        // scan over the segments backwards, tracking the needed set of locals in each segment
        .scan(Vec::new(), |state, raw_seg| {
            let outputs = take(state);
            let init = outputs.iter().cloned().collect::<HashSet<_>>();
            let inputs = raw_seg
                .iter()
                .rev()
                .fold(init, register_statement_locals)
                .into_iter()
                .collect::<Vec<_>>();
            *state = inputs.clone();

            Some(raw_seg.into_segment(inputs, outputs))
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>();

    let segment_tokens = segments
        .into_iter()
        .map(ToTokens::into_token_stream)
        .reduce(|mut acc, x| {
            acc.extend(x);
            acc
        })
        .unwrap();
    let program_tokens = quote! {
        fn main() {
            #segment_tokens
        }
    };
    println!("{}", program_tokens);

    //problems:
    // names
    // lifetimes
    //   what if i take let b = &a and only reference b in future sections?
    // control flow
}
