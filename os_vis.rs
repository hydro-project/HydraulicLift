fn expr() {
    Expr::Block {
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
                        ident: Ident { sym: x },
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
                                        ident: Ident { sym: hf_in },
                                        arguments: PathArguments::None,
                                    }],
                                },
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
                    Expr::Return {
                        attrs: [],
                        return_token: Return,
                        expr: Some(Expr::Path {
                            attrs: [],
                            qself: None,
                            path: Path {
                                leading_colon: None,
                                segments: [PathSegment {
                                    ident: Ident { sym: x },
                                    arguments: PathArguments::None,
                                }],
                            },
                        }),
                    },
                    Some(Semi),
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
                                    ident: Ident { sym: x },
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
    }
}
fn r_expr() {
    Block(RExprBlock {
        stmt: Let(Tagged(
            RStmtLet {
                id: Ident { sym: x },
                value: Raw(Tagged(RExprRaw(hf_in + 1), ())),
            },
            (),
        )),
        expr: Block(RExprBlock {
            stmt: Return(RStmtReturn {
                value: Raw(Tagged(RExprRaw(x), ())),
            }),
            expr: Raw(Tagged(RExprRaw(x + 2), ())),
        }),
    })
}
fn r_expr_tagged() {
    Block(RExprBlock {
        stmt: Let(Tagged(
            RStmtLet {
                id: Ident { sym: x },
                value: Raw(Tagged(
                    RExprRaw(hf_in + 1),
                    IO {
                        ins: Scope({ Ident { sym: hf_in } }),
                        outs: Scope({}),
                    },
                )),
            },
            IO {
                ins: Scope({ Ident { sym: hf_in } }),
                outs: Scope({ Ident { sym: x } }),
            },
        )),
        expr: Block(RExprBlock {
            stmt: Return(RStmtReturn {
                value: Raw(Tagged(
                    RExprRaw(x),
                    IO {
                        ins: Scope({ Ident { sym: x } }),
                        outs: Scope({}),
                    },
                )),
            }),
            expr: Raw(Tagged(
                RExprRaw(x + 2),
                IO {
                    ins: Scope({ Ident { sym: x } }),
                    outs: Scope({}),
                },
            )),
        }),
    })
}
fn h_expr() {
    HOutput {
        input: HReturn {
            value: Raw(Tagged(
                HExprRaw {
                    expr: x,
                    input: Bind(Tagged(
                        HBind {
                            id: Ident { sym: x },
                            value: Raw(Tagged(
                                HExprRaw {
                                    expr: hf_in + 1,
                                    input: Input(HInput),
                                },
                                IO {
                                    ins: Scope({ Ident { sym: hf_in } }),
                                    outs: Scope({}),
                                },
                            )),
                        },
                        IO {
                            ins: Scope({ Ident { sym: hf_in } }),
                            outs: Scope({ Ident { sym: x } }),
                        },
                    )),
                },
                IO {
                    ins: Scope({ Ident { sym: x } }),
                    outs: Scope({}),
                },
            )),
        },
        other: None,
    }
}
fn hf() {
    Map {
        f: |(value, _)| value,
        input: Map {
            f: |(x)| {
                let value = x;
                (value, ())
            },
            input: Map {
                f: |(x, (hf_in))| (x),
                input: Map {
                    f: |(hf_in)| {
                        let value = hf_in + 1;
                        (value, ())
                    },
                    input: Tee {
                        inner: RefCell { value: Placeholder },
                    },
                },
            },
        },
    }
}
fn HFPlus() {
    node0 = Placeholder;

    node0
        .MAP(|(hf_in)| {
            let value = hf_in + 1;
            (value, ())
        })
        .MAP(|(x, (hf_in))| (x))
        .MAP(|(x)| {
            let value = x;
            (value, ())
        })
        .MAP(|(value, _)| value)
}
