#![feature(box_patterns)]

mod segment;
mod visitor;

use std::mem::take;

use quote::{quote, ToTokens};
use segment::RawSegment;
use syn::{
    parse_str,
    Block, Expr, Local, LocalInit, Stmt,
};
use visitor::LocalVisitor;

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

fn main() {
    //TODO (maybe): make future directly


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
