fn main() {
    ExprClosure {
        attrs: [],
        lifetimes: None,
        constness: None,
        movability: None,
        asyncness: None,
        capture: None,
        or1_token: Or,
        inputs: [Pat::Ident {
            attrs: [],
            by_ref: None,
            mutability: None,
            ident: Ident(a),
            subpat: None,
        }],
        or2_token: Or,
        output: ReturnType::Default,
        body: Expr::Block {
            attrs: [],
            label: None,
            block: Block {
                brace_token: Brace,
                stmts: [
                    Stmt::Local {
                        attrs: [],
                        let_token: Let,
                        pat: Pat::Ident {
                            attrs: [],
                            by_ref: None,
                            mutability: None,
                            ident: Ident(x),
                            subpat: None,
                        },
                        init: Some(LocalInit {
                            eq_token: Eq,
                            expr: Expr::Binary {
                                attrs: [],
                                left: Expr::Lit {
                                    attrs: [],
                                    lit: Lit::Int { token: 1 },
                                },
                                op: BinOp::Add(Plus),
                                right: Expr::Lit {
                                    attrs: [],
                                    lit: Lit::Int { token: 1 },
                                },
                            },
                            diverge: None,
                        }),
                        semi_token: Semi,
                    },
                    Stmt::Expr(
                        Expr::If {
                            attrs: [],
                            if_token: If,
                            cond: Expr::Paren {
                                attrs: [],
                                paren_token: Paren,
                                expr: Expr::Binary {
                                    attrs: [],
                                    left: Expr::Path {
                                        attrs: [],
                                        qself: None,
                                        path: Path {
                                            leading_colon: None,
                                            segments: [PathSegment {
                                                ident: Ident(x),
                                                arguments: PathArguments::None,
                                            }],
                                        },
                                    },
                                    op: BinOp::Lt(Lt),
                                    right: Expr::Lit {
                                        attrs: [],
                                        lit: Lit::Int { token: 5 },
                                    },
                                },
                            },
                            then_branch: Block {
                                brace_token: Brace,
                                stmts: [Stmt::Local {
                                    attrs: [],
                                    let_token: Let,
                                    pat: Pat::Ident {
                                        attrs: [],
                                        by_ref: None,
                                        mutability: None,
                                        ident: Ident(y),
                                        subpat: None,
                                    },
                                    init: Some(LocalInit {
                                        eq_token: Eq,
                                        expr: Expr::Await {
                                            attrs: [],
                                            base: Expr::Path {
                                                attrs: [],
                                                qself: None,
                                                path: Path {
                                                    leading_colon: None,
                                                    segments: [PathSegment {
                                                        ident: Ident(a),
                                                        arguments: PathArguments::None,
                                                    }],
                                                },
                                            },
                                            dot_token: Dot,
                                            await_token: Await,
                                        },
                                        diverge: None,
                                    }),
                                    semi_token: Semi,
                                }],
                            },
                            else_branch: Some((
                                Else,
                                Expr::If {
                                    attrs: [],
                                    if_token: If,
                                    cond: Expr::Paren {
                                        attrs: [],
                                        paren_token: Paren,
                                        expr: Expr::Binary {
                                            attrs: [],
                                            left: Expr::Path {
                                                attrs: [],
                                                qself: None,
                                                path: Path {
                                                    leading_colon: None,
                                                    segments: [PathSegment {
                                                        ident: Ident(x),
                                                        arguments: PathArguments::None,
                                                    }],
                                                },
                                            },
                                            op: BinOp::Gt(Gt),
                                            right: Expr::Lit {
                                                attrs: [],
                                                lit: Lit::Int { token: 10 },
                                            },
                                        },
                                    },
                                    then_branch: Block {
                                        brace_token: Brace,
                                        stmts: [Stmt::Local {
                                            attrs: [],
                                            let_token: Let,
                                            pat: Pat::Ident {
                                                attrs: [],
                                                by_ref: None,
                                                mutability: None,
                                                ident: Ident(z),
                                                subpat: None,
                                            },
                                            init: Some(LocalInit {
                                                eq_token: Eq,
                                                expr: Expr::Await {
                                                    attrs: [],
                                                    base: Expr::Path {
                                                        attrs: [],
                                                        qself: None,
                                                        path: Path {
                                                            leading_colon: None,
                                                            segments: [PathSegment {
                                                                ident: Ident(a),
                                                                arguments: PathArguments::None,
                                                            }],
                                                        },
                                                    },
                                                    dot_token: Dot,
                                                    await_token: Await,
                                                },
                                                diverge: None,
                                            }),
                                            semi_token: Semi,
                                        }],
                                    },
                                    else_branch: Some((
                                        Else,
                                        Expr::Block {
                                            attrs: [],
                                            label: None,
                                            block: Block {
                                                brace_token: Brace,
                                                stmts: [Stmt::Local {
                                                    attrs: [],
                                                    let_token: Let,
                                                    pat: Pat::Ident {
                                                        attrs: [],
                                                        by_ref: None,
                                                        mutability: None,
                                                        ident: Ident(z),
                                                        subpat: None,
                                                    },
                                                    init: Some(LocalInit {
                                                        eq_token: Eq,
                                                        expr: Expr::Await {
                                                            attrs: [],
                                                            base: Expr::Path {
                                                                attrs: [],
                                                                qself: None,
                                                                path: Path {
                                                                    leading_colon: None,
                                                                    segments: [PathSegment {
                                                                        ident: Ident(a),
                                                                        arguments:
                                                                            PathArguments::None,
                                                                    }],
                                                                },
                                                            },
                                                            dot_token: Dot,
                                                            await_token: Await,
                                                        },
                                                        diverge: None,
                                                    }),
                                                    semi_token: Semi,
                                                }],
                                            },
                                        },
                                    )),
                                },
                            )),
                        },
                        None,
                    ),
                    Stmt::Expr(
                        Expr::Binary {
                            attrs: [],
                            left: Expr::Path {
                                attrs: [],
                                qself: None,
                                path: Path {
                                    leading_colon: None,
                                    segments: [PathSegment {
                                        ident: Ident(x),
                                        arguments: PathArguments::None,
                                    }],
                                },
                            },
                            op: BinOp::Add(Plus),
                            right: Expr::Lit {
                                attrs: [],
                                lit: Lit::Int { token: 2 },
                            },
                        },
                        None,
                    ),
                ],
            },
        },
    }
}
