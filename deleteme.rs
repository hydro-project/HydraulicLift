fn main() {
    let x = ExprClosure {
        attrs: [],
        lifetimes: None,
        constness: None,
        movability: None,
        asyncness: Some(Async),
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
                    Stmt::Local {
                        attrs: [],
                        let_token: Let,
                        pat: Pat::Ident {
                            attrs: [],
                            by_ref: None,
                            mutability: None,
                            ident: Ident(b),
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
                    },
                    Stmt::Local {
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
                            expr: Expr::Binary {
                                attrs: [],
                                left: Expr::Lit {
                                    attrs: [],
                                    lit: Lit::Int { token: 1 },
                                },
                                op: BinOp::Add(Plus),
                                right: Expr::Path {
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
                            },
                            diverge: None,
                        }),
                        semi_token: Semi,
                    },
                    Stmt::Local {
                        attrs: [],
                        let_token: Let,
                        pat: Pat::Ident {
                            attrs: [],
                            by_ref: None,
                            mutability: None,
                            ident: Ident(c),
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
                                            ident: Ident(b),
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
                    },
                    Stmt::Local {
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
                                op: BinOp::Add(Plus),
                                right: Expr::Path {
                                    attrs: [],
                                    qself: None,
                                    path: Path {
                                        leading_colon: None,
                                        segments: [PathSegment {
                                            ident: Ident(y),
                                            arguments: PathArguments::None,
                                        }],
                                    },
                                },
                            },
                            diverge: None,
                        }),
                        semi_token: Semi,
                    },
                    Stmt::Expr(
                        Expr::Return {
                            attrs: [],
                            return_token: Return,
                            expr: Some(Expr::Path {
                                attrs: [],
                                qself: None,
                                path: Path {
                                    leading_colon: None,
                                    segments: [PathSegment {
                                        ident: Ident(c),
                                        arguments: PathArguments::None,
                                    }],
                                },
                            }),
                        },
                        Some(Semi),
                    ),
                ],
            },
        },
    };
}
