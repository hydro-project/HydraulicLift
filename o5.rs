fn R() {
    Block(RExprBlock {
        stmt: Let(Tagged(
            RStmtLet {
                id: Ident { sym: _ },
                value: Raw(Tagged(RExprRaw(DebugStr { inner: "()" }), ())),
            },
            (),
        )),
        expr: Block(RExprBlock {
            stmt: Let(Tagged(
                RStmtLet {
                    id: Ident { sym: x },
                    value: Raw(Tagged(RExprRaw(DebugStr { inner: "1 + 1" }), ())),
                },
                (),
            )),
            expr: Block(RExprBlock {
                stmt: Let(Tagged(
                    RStmtLet {
                        id: Ident { sym: _ },
                        value: If(RExprIf {
                            cond_expr: Raw(Tagged(RExprRaw(DebugStr { inner: "(x < 5)" }), ())),
                            then_expr: Block(RExprBlock {
                                stmt: Let(Tagged(
                                    RStmtLet {
                                        id: Ident { sym: _ },
                                        value: Raw(Tagged(RExprRaw(DebugStr { inner: "()" }), ())),
                                    },
                                    (),
                                )),
                                expr: Block(RExprBlock {
                                    stmt: Let(Tagged(
                                        RStmtLet {
                                            id: Ident { sym: z },
                                            value: Raw(Tagged(
                                                RExprRaw(DebugStr { inner: "1 + 1" }),
                                                (),
                                            )),
                                        },
                                        (),
                                    )),
                                    expr: Block(RExprBlock {
                                        stmt: Let(Tagged(
                                            RStmtLet {
                                                id: Ident { sym: y },
                                                value: Raw(Tagged(
                                                    RExprRaw(DebugStr { inner: "a . await" }),
                                                    (),
                                                )),
                                            },
                                            (),
                                        )),
                                        expr: Raw(Tagged(
                                            RExprRaw(DebugStr { inner: "y + z" }),
                                            (),
                                        )),
                                    }),
                                }),
                            }),
                            else_expr: If(RExprIf {
                                cond_expr: Raw(Tagged(
                                    RExprRaw(DebugStr { inner: "(x > 10)" }),
                                    (),
                                )),
                                then_expr: Block(RExprBlock {
                                    stmt: Let(Tagged(
                                        RStmtLet {
                                            id: Ident { sym: _ },
                                            value: Raw(Tagged(
                                                RExprRaw(DebugStr { inner: "()" }),
                                                (),
                                            )),
                                        },
                                        (),
                                    )),
                                    expr: Block(RExprBlock {
                                        stmt: Let(Tagged(
                                            RStmtLet {
                                                id: Ident { sym: z },
                                                value: Raw(Tagged(
                                                    RExprRaw(DebugStr { inner: "a . await" }),
                                                    (),
                                                )),
                                            },
                                            (),
                                        )),
                                        expr: Raw(Tagged(
                                            RExprRaw(DebugStr { inner: "z + x" }),
                                            (),
                                        )),
                                    }),
                                }),
                                else_expr: Block(RExprBlock {
                                    stmt: Let(Tagged(
                                        RStmtLet {
                                            id: Ident { sym: _ },
                                            value: Raw(Tagged(
                                                RExprRaw(DebugStr { inner: "()" }),
                                                (),
                                            )),
                                        },
                                        (),
                                    )),
                                    expr: Block(RExprBlock {
                                        stmt: Let(Tagged(
                                            RStmtLet {
                                                id: Ident { sym: z },
                                                value: Raw(Tagged(
                                                    RExprRaw(DebugStr { inner: "a . await" }),
                                                    (),
                                                )),
                                            },
                                            (),
                                        )),
                                        expr: Block(RExprBlock {
                                            stmt: Return(RStmtReturn {
                                                value: Raw(Tagged(
                                                    RExprRaw(DebugStr { inner: "z" }),
                                                    (),
                                                )),
                                            }),
                                            expr: Raw(Tagged(
                                                RExprRaw(DebugStr { inner: "()" }),
                                                (),
                                            )),
                                        }),
                                    }),
                                }),
                            }),
                        }),
                    },
                    (),
                )),
                expr: Raw(Tagged(RExprRaw(DebugStr { inner: "x + 2" }), ())),
            }),
        }),
    })
}

fn R_tag() {
    Block(RExprBlock {
        stmt: Let(Tagged(
            RStmtLet {
                id: Ident { sym: _ },
                value: Raw(Tagged(
                    RExprRaw(DebugStr { inner: "()" }),
                    IO {
                        ins: Scope({}),
                        outs: Scope({}),
                    },
                )),
            },
            IO {
                ins: Scope({}),
                outs: Scope({ Ident { sym: _ } }),
            },
        )),
        expr: Block(RExprBlock {
            stmt: Let(Tagged(
                RStmtLet {
                    id: Ident { sym: x },
                    value: Raw(Tagged(
                        RExprRaw(DebugStr { inner: "1 + 1" }),
                        IO {
                            ins: Scope({}),
                            outs: Scope({}),
                        },
                    )),
                },
                IO {
                    ins: Scope({}),
                    outs: Scope({ Ident { sym: x } }),
                },
            )),
            expr: Block(RExprBlock {
                stmt: Let(Tagged(
                    RStmtLet {
                        id: Ident { sym: _ },
                        value: If(RExprIf {
                            cond_expr: Raw(Tagged(
                                RExprRaw(DebugStr { inner: "(x < 5)" }),
                                IO {
                                    ins: Scope({}),
                                    outs: Scope({}),
                                },
                            )),
                            then_expr: Block(RExprBlock {
                                stmt: Let(Tagged(
                                    RStmtLet {
                                        id: Ident { sym: _ },
                                        value: Raw(Tagged(
                                            RExprRaw(DebugStr { inner: "()" }),
                                            IO {
                                                ins: Scope({}),
                                                outs: Scope({}),
                                            },
                                        )),
                                    },
                                    IO {
                                        ins: Scope({}),
                                        outs: Scope({ Ident { sym: _ } }),
                                    },
                                )),
                                expr: Block(RExprBlock {
                                    stmt: Let(Tagged(
                                        RStmtLet {
                                            id: Ident { sym: z },
                                            value: Raw(Tagged(
                                                RExprRaw(DebugStr { inner: "1 + 1" }),
                                                IO {
                                                    ins: Scope({}),
                                                    outs: Scope({}),
                                                },
                                            )),
                                        },
                                        IO {
                                            ins: Scope({}),
                                            outs: Scope({ Ident { sym: z } }),
                                        },
                                    )),
                                    expr: Block(RExprBlock {
                                        stmt: Let(Tagged(
                                            RStmtLet {
                                                id: Ident { sym: y },
                                                value: Raw(Tagged(
                                                    RExprRaw(DebugStr { inner: "a . await" }),
                                                    IO {
                                                        ins: Scope({}),
                                                        outs: Scope({}),
                                                    },
                                                )),
                                            },
                                            IO {
                                                ins: Scope({}),
                                                outs: Scope({ Ident { sym: y } }),
                                            },
                                        )),
                                        expr: Raw(Tagged(
                                            RExprRaw(DebugStr { inner: "y + z" }),
                                            IO {
                                                ins: Scope({}),
                                                outs: Scope({}),
                                            },
                                        )),
                                    }),
                                }),
                            }),
                            else_expr: If(RExprIf {
                                cond_expr: Raw(Tagged(
                                    RExprRaw(DebugStr { inner: "(x > 10)" }),
                                    IO {
                                        ins: Scope({}),
                                        outs: Scope({}),
                                    },
                                )),
                                then_expr: Block(RExprBlock {
                                    stmt: Let(Tagged(
                                        RStmtLet {
                                            id: Ident { sym: _ },
                                            value: Raw(Tagged(
                                                RExprRaw(DebugStr { inner: "()" }),
                                                IO {
                                                    ins: Scope({}),
                                                    outs: Scope({}),
                                                },
                                            )),
                                        },
                                        IO {
                                            ins: Scope({}),
                                            outs: Scope({ Ident { sym: _ } }),
                                        },
                                    )),
                                    expr: Block(RExprBlock {
                                        stmt: Let(Tagged(
                                            RStmtLet {
                                                id: Ident { sym: z },
                                                value: Raw(Tagged(
                                                    RExprRaw(DebugStr { inner: "a . await" }),
                                                    IO {
                                                        ins: Scope({}),
                                                        outs: Scope({}),
                                                    },
                                                )),
                                            },
                                            IO {
                                                ins: Scope({}),
                                                outs: Scope({ Ident { sym: z } }),
                                            },
                                        )),
                                        expr: Raw(Tagged(
                                            RExprRaw(DebugStr { inner: "z + x" }),
                                            IO {
                                                ins: Scope({}),
                                                outs: Scope({}),
                                            },
                                        )),
                                    }),
                                }),
                                else_expr: Block(RExprBlock {
                                    stmt: Let(Tagged(
                                        RStmtLet {
                                            id: Ident { sym: _ },
                                            value: Raw(Tagged(
                                                RExprRaw(DebugStr { inner: "()" }),
                                                IO {
                                                    ins: Scope({}),
                                                    outs: Scope({}),
                                                },
                                            )),
                                        },
                                        IO {
                                            ins: Scope({}),
                                            outs: Scope({ Ident { sym: _ } }),
                                        },
                                    )),
                                    expr: Block(RExprBlock {
                                        stmt: Let(Tagged(
                                            RStmtLet {
                                                id: Ident { sym: z },
                                                value: Raw(Tagged(
                                                    RExprRaw(DebugStr { inner: "a . await" }),
                                                    IO {
                                                        ins: Scope({}),
                                                        outs: Scope({}),
                                                    },
                                                )),
                                            },
                                            IO {
                                                ins: Scope({}),
                                                outs: Scope({ Ident { sym: z } }),
                                            },
                                        )),
                                        expr: Block(RExprBlock {
                                            stmt: Return(RStmtReturn {
                                                value: Raw(Tagged(
                                                    RExprRaw(DebugStr { inner: "z" }),
                                                    IO {
                                                        ins: Scope({}),
                                                        outs: Scope({}),
                                                    },
                                                )),
                                            }),
                                            expr: Raw(Tagged(
                                                RExprRaw(DebugStr { inner: "()" }),
                                                IO {
                                                    ins: Scope({}),
                                                    outs: Scope({}),
                                                },
                                            )),
                                        }),
                                    }),
                                }),
                            }),
                        }),
                    },
                    IO {
                        ins: Scope({}),
                        outs: Scope({ Ident { sym: _ } }),
                    },
                )),
                expr: Raw(Tagged(
                    RExprRaw(DebugStr { inner: "x + 2" }),
                    IO {
                        ins: Scope({}),
                        outs: Scope({}),
                    },
                )),
            }),
        }),
    })
}

fn H() {
    HOutput { input: HReturn { input: Raw(Tagged(HExprRaw { input: Bind(Tagged(HBind { input: Union(HExprUnion(Raw(Tagged(HExprRaw { input: Bind(Tagged(HBind { input: Raw(Tagged(HExprRaw { input: Bind(Tagged(HBind { input: Raw(Tagged(HExprRaw { input: Bind(Tagged(HBind { input: Raw(Tagged(HExprRaw { input: Filter(HFilter { cond: Shared(HExprShared(Raw(Tagged(HExprRaw { input: Bind(Tagged(HBind { input: Raw(Tagged(HExprRaw { input: Bind(Tagged(HBind { input: Raw(Tagged(HExprRaw { input: Input(HInput), expr: Expr::Tuple { attrs: [], paren_token: Paren, elems: [] } }, IO { ins: Scope({}), outs: Scope({}) })), id: Ident { sym: _ } }, IO { ins: Scope({}), outs: Scope({Ident { sym: _ }}) })), expr: Expr::Binary { attrs: [], left: Expr::Lit { attrs: [], lit: Lit::Int { token: 1 } }, op: BinOp::Add(Plus), right: Expr::Lit { attrs: [], lit: Lit::Int { token: 1 } } } }, IO { ins: Scope({}), outs: Scope({}) })), id: Ident { sym: x } }, IO { ins: Scope({}), outs: Scope({Ident { sym: x }}) })), expr: Expr::Paren { attrs: [], paren_token: Paren, expr: Expr::Binary { attrs: [], left: Expr::Path { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { sym: x }, arguments: PathArguments::None }] } }, op: BinOp::Lt(Lt), right: Expr::Lit { attrs: [], lit: Lit::Int { token: 5 } } } } }, IO { ins: Scope({}), outs: Scope({}) })))), expr: Expr::Lit { attrs: [], lit: Lit::Bool { value: true } } }), expr: Expr::Tuple { attrs: [], paren_token: Paren, elems: [] } }, IO { ins: Scope({}), outs: Scope({}) })), id: Ident { sym: _ } }, IO { ins: Scope({}), outs: Scope({Ident { sym: _ }}) })), expr: Expr::Binary { attrs: [], left: Expr::Lit { attrs: [], lit: Lit::Int { token: 1 } }, op: BinOp::Add(Plus), right: Expr::Lit { attrs: [], lit: Lit::Int { token: 1 } } } }, IO { ins: Scope({}), outs: Scope({}) })), id: Ident { sym: z } }, IO { ins: Scope({}), outs: Scope({Ident { sym: z }}) })), expr: Expr::Await { attrs: [], base: Expr::Path { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { sym: a }, arguments: PathArguments::None }] } }, dot_token: Dot, await_token: Await } }, IO { ins: Scope({}), outs: Scope({}) })), id: Ident { sym: y } }, IO { ins: Scope({}), outs: Scope({Ident { sym: y }}) })), expr: Expr::Binary { attrs: [], left: Expr::Path { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { sym: y }, arguments: PathArguments::None }] } }, op: BinOp::Add(Plus), right: Expr::Path { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { sym: z }, arguments: PathArguments::None }] } } } }, IO { ins: Scope({}), outs: Scope({}) })), Raw(Tagged(HExprRaw { input: Bind(Tagged(HBind { input: Raw(Tagged(HExprRaw { input: Bind(Tagged(HBind { input: Raw(Tagged(HExprRaw { input: Filter(HFilter { cond: Shared(HExprShared(Raw(Tagged(HExprRaw { input: Filter(HFilter { cond: Shared(HExprShared(Raw(Tagged(HExprRaw { input: Bind(Tagged(HBind { input: Raw(Tagged(HExprRaw { input: Bind(Tagged(HBind { input: Raw(Tagged(HExprRaw { input: Input(HInput), expr: Expr::Tuple { attrs: [], paren_token: Paren, elems: [] } }, IO { ins: Scope({}), outs: Scope({}) })), id: Ident { sym: _ } }, IO { ins: Scope({}), outs: Scope({Ident { sym: _ }}) })), expr: Expr::Binary { attrs: [], left: Expr::Lit { attrs: [], lit: Lit::Int { token: 1 } }, op: BinOp::Add(Plus), right: Expr::Lit { attrs: [], lit: Lit::Int { token: 1 } } } }, IO { ins: Scope({}), outs: Scope({}) })), id: Ident { sym: x } }, IO { ins: Scope({}), outs: Scope({Ident { sym: x }}) })), expr: Expr::Paren { attrs: [], paren_token: Paren, expr: Expr::Binary { attrs: [], left: Expr::Path { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { sym: x }, arguments: PathArguments::None }] } }, op: BinOp::Lt(Lt), right: Expr::Lit { attrs: [], lit: Lit::Int { token: 5 } } } } }, IO { ins: Scope({}), outs: Scope({}) })))), expr: Expr::Lit { attrs: [], lit: Lit::Bool { value: false } } }), expr: Expr::Paren { attrs: [], paren_token: Paren, expr: Expr::Binary { attrs: [], left: Expr::Path { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { sym: x }, arguments: PathArguments::None }] } }, op: BinOp::Gt(Gt), right: Expr::Lit { attrs: [], lit: Lit::Int { token: 10 } } } } }, IO { ins: Scope({}), outs: Scope({}) })))), expr: Expr::Lit { attrs: [], lit: Lit::Bool { value: true } } }), expr: Expr::Tuple { attrs: [], paren_token: Paren, elems: [] } }, IO { ins: Scope({}), outs: Scope({}) })), id: Ident { sym: _ } }, IO { ins: Scope({}), outs: Scope({Ident { sym: _ }}) })), expr: Expr::Await { attrs: [], base: Expr::Path { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { sym: a }, arguments: PathArguments::None }] } }, dot_token: Dot, await_token: Await } }, IO { ins: Scope({}), outs: Scope({}) })), id: Ident { sym: z } }, IO { ins: Scope({}), outs: Scope({Ident { sym: z }}) })), expr: Expr::Binary { attrs: [], left: Expr::Path { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { sym: z }, arguments: PathArguments::None }] } }, op: BinOp::Add(Plus), right: Expr::Path { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { sym: x }, arguments: PathArguments::None }] } } } }, IO { ins: Scope({}), outs: Scope({}) })))), id: Ident { sym: _ } }, IO { ins: Scope({}), outs: Scope({Ident { sym: _ }}) })), expr: Expr::Binary { attrs: [], left: Expr::Path { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { sym: x }, arguments: PathArguments::None }] } }, op: BinOp::Add(Plus), right: Expr::Lit { attrs: [], lit: Lit::Int { token: 2 } } } }, IO { ins: Scope({}), outs: Scope({}) })) }, other: Some(HOutput { input: HReturn { input: Raw(Tagged(HExprRaw { input: Bind(Tagged(HBind { input: Raw(Tagged(HExprRaw { input: Bind(Tagged(HBind { input: Raw(Tagged(HExprRaw { input: Filter(HFilter { cond: Shared(HExprShared(Raw(Tagged(HExprRaw { input: Filter(HFilter { cond: Shared(HExprShared(Raw(Tagged(HExprRaw { input: Bind(Tagged(HBind { input: Raw(Tagged(HExprRaw { input: Bind(Tagged(HBind { input: Raw(Tagged(HExprRaw { input: Input(HInput), expr: Expr::Tuple { attrs: [], paren_token: Paren, elems: [] } }, IO { ins: Scope({}), outs: Scope({}) })), id: Ident { sym: _ } }, IO { ins: Scope({}), outs: Scope({Ident { sym: _ }}) })), expr: Expr::Binary { attrs: [], left: Expr::Lit { attrs: [], lit: Lit::Int { token: 1 } }, op: BinOp::Add(Plus), right: Expr::Lit { attrs: [], lit: Lit::Int { token: 1 } } } }, IO { ins: Scope({}), outs: Scope({}) })), id: Ident { sym: x } }, IO { ins: Scope({}), outs: Scope({Ident { sym: x }}) })), expr: Expr::Paren { attrs: [], paren_token: Paren, expr: Expr::Binary { attrs: [], left: Expr::Path { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { sym: x }, arguments: PathArguments::None }] } }, op: BinOp::Lt(Lt), right: Expr::Lit { attrs: [], lit: Lit::Int { token: 5 } } } } }, IO { ins: Scope({}), outs: Scope({}) })))), expr: Expr::Lit { attrs: [], lit: Lit::Bool { value: false } } }), expr: Expr::Paren { attrs: [], paren_token: Paren, expr: Expr::Binary { attrs: [], left: Expr::Path { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { sym: x }, arguments: PathArguments::None }] } }, op: BinOp::Gt(Gt), right: Expr::Lit { attrs: [], lit: Lit::Int { token: 10 } } } } }, IO { ins: Scope({}), outs: Scope({}) })))), expr: Expr::Lit { attrs: [], lit: Lit::Bool { value: false } } }), expr: Expr::Tuple { attrs: [], paren_token: Paren, elems: [] } }, IO { ins: Scope({}), outs: Scope({}) })), id: Ident { sym: _ } }, IO { ins: Scope({}), outs: Scope({Ident { sym: _ }}) })), expr: Expr::Await { attrs: [], base: Expr::Path { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { sym: a }, arguments: PathArguments::None }] } }, dot_token: Dot, await_token: Await } }, IO { ins: Scope({}), outs: Scope({}) })), id: Ident { sym: z } }, IO { ins: Scope({}), outs: Scope({Ident { sym: z }}) })), expr: Expr::Path { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { sym: z }, arguments: PathArguments::None }] } } }, IO { ins: Scope({}), outs: Scope({}) })) }, other: None }) }
}

fn HFPlus() {
    Union(Map { f: | (value , _) | { value }, input: Map { f: | () | { let value = x + 2 ; (value , ()) }, input: Map { f: | (_ , ()) | { (_ ,) }, input: Union(Map { f: | () | { let value = y + z ; (value , ()) }, input: Map { f: | (y , ()) | { (y ,) }, input: Map { f: | () | { let value = a . await ; (value , ()) }, input: Map { f: | (z , ()) | { (z ,) }, input: Map { f: | () | { let value = 1 + 1 ; (value , ()) }, input: Map { f: | (_ , ()) | { (_ ,) }, input: Map { f: | () | { let value = () ; (value , ()) }, input: Map { f: | (cond , scope) | { if cond != true { return None } Some (scope) }, input: Tee { inner: RefCell { value: Map { f: | () | { let value = (x < 5) ; (value , ()) }, input: Map { f: | (x , ()) | { (x ,) }, input: Map { f: | () | { let value = 1 + 1 ; (value , ()) }, input: Map { f: | (_ , ()) | { (_ ,) }, input: Map { f: | () | { let value = () ; (value , ()) }, input: Tee { inner: RefCell { value: Placeholder } } } } } } } } } } } } } } } } }, Map { f: | () | { let value = z + x ; (value , ()) }, input: Map { f: | (z , ()) | { (z ,) }, input: Map { f: | () | { let value = a . await ; (value , ()) }, input: Map { f: | (_ , ()) | { (_ ,) }, input: Map { f: | () | { let value = () ; (value , ()) }, input: Map { f: | (cond , scope) | { if cond != true { return None } Some (scope) }, input: Tee { inner: RefCell { value: Map { f: | () | { let value = (x > 10) ; (value , ()) }, input: Map { f: | (cond , scope) | { if cond != false { return None } Some (scope) }, input: Tee { inner: RefCell { value: Map { f: | () | { let value = (x < 5) ; (value , ()) }, input: Map { f: | (x , ()) | { (x ,) }, input: Map { f: | () | { let value = 1 + 1 ; (value , ()) }, input: Map { f: | (_ , ()) | { (_ ,) }, input: Map { f: | () | { let value = () ; (value , ()) }, input: Tee { inner: RefCell { value: Placeholder } } } } } } } } } } } } } } } } } } }) } } }, Map { f: | (value , _) | { value }, input: Map { f: | () | { let value = z ; (value , ()) }, input: Map { f: | (z , ()) | { (z ,) }, input: Map { f: | () | { let value = a . await ; (value , ()) }, input: Map { f: | (_ , ()) | { (_ ,) }, input: Map { f: | () | { let value = () ; (value , ()) }, input: Map { f: | (cond , scope) | { if cond != false { return None } Some (scope) }, input: Tee { inner: RefCell { value: Map { f: | () | { let value = (x > 10) ; (value , ()) }, input: Map { f: | (cond , scope) | { if cond != false { return None } Some (scope) }, input: Tee { inner: RefCell { value: Map { f: | () | { let value = (x < 5) ; (value , ()) }, input: Map { f: | (x , ()) | { (x ,) }, input: Map { f: | () | { let value = 1 + 1 ; (value , ()) }, input: Map { f: | (_ , ()) | { (_ ,) }, input: Map { f: | () | { let value = () ; (value , ()) }, input: Tee { inner: RefCell { value: Placeholder } } } } } } } } } } } } } } } } } } } })
}
