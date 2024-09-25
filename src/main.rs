#![feature(box_patterns)]

use std::{collections::HashSet, convert::identity, hash::Hash, mem::take, path::Iter};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_quote, parse_str, punctuated::Punctuated, token::Brace, visit::{self, Visit}, Block, Expr, ExprAwait, ExprClosure, ExprPath, ExprTuple, Ident, Local, LocalInit, Pat, PatIdent, PatPath, Path, Stmt
};

#[derive(Clone)]
enum RawSegment {
    Await(Vec<Stmt>),
    End(Vec<Stmt>),
}


impl RawSegment {
    fn into_block(self) -> Block {
        Block {
            brace_token: Default::default(),
            stmts: match self {
                Self::Await(v) => v,
                Self::End(v) => v,
            }
        }
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

// fn register_statement_locals(mut locals: HashSet<Ident>, stmt: &Stmt) -> HashSet<Ident> {
//     // the local defined this statement
//     if let Some(defined_local) = get_defined_local(stmt) {
//         locals.remove(&defined_local); // TODO: could be many!! need to visit
//     }
//     // the locals needed by this statement
//     let needed_locals = get_needed_locals(stmt);
//     locals.extend(needed_locals);

//     locals
// }

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
    fn visit_raw_segment(idents: Vec<Ident>, raw_segment: RawSegment) -> Vec<Ident> {
        let mut visitor = LocalVisitor {
            idents: idents.into_iter().collect()
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
        if let Pat::Ident(PatIdent { ident, ..}) = &local.pat {
            self.idents.remove(ident);
        }
        
        // visit nested?
        visit::visit_local(self, local);
    }
}

// fn get_needed_locals(stmt: &Stmt) -> HashSet<Ident> {
//     let mut visitor = LocalVisitor::new();
//     visitor.visit_stmt(stmt);
//     visitor.idents
// }

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

    let e = d.await;
    let out = z + 2;
    out
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

    // parse the raw segments into segments (adding local variable input/output information)
    let segments = raw_segs
        .into_iter()
        .rev() // walk backwards through segments
        // scan over the segments backwards, tracking the needed set of locals in each segment
        .scan(Vec::new(), |state, raw_seg| {
            let outputs = take(state);
            let inputs = LocalVisitor::visit_raw_segment(outputs.clone(), raw_seg.clone());
            *state = inputs.clone();

            Some(raw_seg.into_segment(inputs, outputs))
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev() //undo the reverse at the beginning
        .collect::<Vec<_>>();

    // concatenate all segments
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
