fn raw() {
    Block(RExprBlock {
        stmt: Expr(Raw(Tagged(DebugStr { inner: "()" }, ()))),
        return_expr: Block(RExprBlock {
            stmt: Let(Tagged(
                RStmtLet {
                    ident: Ident { sym: x },
                    value: Raw(Tagged(DebugStr { inner: "1 + 1" }, ())),
                },
                (),
            )),
            return_expr: Block(RExprBlock {
                stmt: Expr(If(RExprIf {
                    cond_expr: Raw(Tagged(DebugStr { inner: "(x < 5)" }, ())),
                    then_expr: Block(RExprBlock {
                        stmt: Expr(Raw(Tagged(DebugStr { inner: "()" }, ()))),
                        return_expr: Block(RExprBlock {
                            stmt: Let(Tagged(
                                RStmtLet {
                                    ident: Ident { sym: z },
                                    value: Raw(Tagged(DebugStr { inner: "1 + 1" }, ())),
                                },
                                (),
                            )),
                            return_expr: Block(RExprBlock {
                                stmt: Let(Tagged(
                                    RStmtLet {
                                        ident: Ident { sym: y },
                                        value: Raw(Tagged(DebugStr { inner: "a . await" }, ())),
                                    },
                                    (),
                                )),
                                return_expr: Raw(Tagged(DebugStr { inner: "y + z" }, ())),
                            }),
                        }),
                    }),
                    else_expr: If(RExprIf {
                        cond_expr: Raw(Tagged(DebugStr { inner: "(x > 10)" }, ())),
                        then_expr: Block(RExprBlock {
                            stmt: Expr(Raw(Tagged(DebugStr { inner: "()" }, ()))),
                            return_expr: Block(RExprBlock {
                                stmt: Let(Tagged(
                                    RStmtLet {
                                        ident: Ident { sym: z },
                                        value: Raw(Tagged(DebugStr { inner: "a . await" }, ())),
                                    },
                                    (),
                                )),
                                return_expr: Raw(Tagged(DebugStr { inner: "z + x" }, ())),
                            }),
                        }),
                        else_expr: Block(RExprBlock {
                            stmt: Expr(Raw(Tagged(DebugStr { inner: "()" }, ()))),
                            return_expr: Block(RExprBlock {
                                stmt: Let(Tagged(
                                    RStmtLet {
                                        ident: Ident { sym: z },
                                        value: Raw(Tagged(DebugStr { inner: "a . await" }, ())),
                                    },
                                    (),
                                )),
                                return_expr: Block(RExprBlock {
                                    stmt: Return(RStmtReturn {
                                        value: Raw(Tagged(DebugStr { inner: "z" }, ())),
                                    }),
                                    return_expr: Raw(Tagged(DebugStr { inner: "()" }, ())),
                                }),
                            }),
                        }),
                    }),
                })),
                return_expr: Raw(Tagged(DebugStr { inner: "x + 2" }, ())),
            }),
        }),
    })
}
fn tagged() {
    Block(RExprBlock {
        stmt: Expr(Raw(Tagged(
            DebugStr { inner: "()" },
            IO {
                input_scope: [Ident { sym: testvar }],
                output_scope: [Ident { sym: testvar }],
            },
        ))),
        return_expr: Block(RExprBlock {
            stmt: Let(Tagged(
                RStmtLet {
                    ident: Ident { sym: x },
                    value: Raw(Tagged(
                        DebugStr { inner: "1 + 1" },
                        IO {
                            input_scope: [Ident { sym: testvar }],
                            output_scope: [Ident { sym: testvar }],
                        },
                    )),
                },
                IO {
                    input_scope: [],
                    output_scope: [Ident { sym: x }],
                },
            )),
            return_expr: Block(RExprBlock {
                stmt: Expr(If(RExprIf {
                    cond_expr: Raw(Tagged(
                        DebugStr { inner: "(x < 5)" },
                        IO {
                            input_scope: [Ident { sym: testvar }],
                            output_scope: [Ident { sym: testvar }],
                        },
                    )),
                    then_expr: Block(RExprBlock {
                        stmt: Expr(Raw(Tagged(
                            DebugStr { inner: "()" },
                            IO {
                                input_scope: [Ident { sym: testvar }],
                                output_scope: [Ident { sym: testvar }],
                            },
                        ))),
                        return_expr: Block(RExprBlock {
                            stmt: Let(Tagged(
                                RStmtLet {
                                    ident: Ident { sym: z },
                                    value: Raw(Tagged(
                                        DebugStr { inner: "1 + 1" },
                                        IO {
                                            input_scope: [Ident { sym: testvar }],
                                            output_scope: [Ident { sym: testvar }],
                                        },
                                    )),
                                },
                                IO {
                                    input_scope: [],
                                    output_scope: [Ident { sym: z }],
                                },
                            )),
                            return_expr: Block(RExprBlock {
                                stmt: Let(Tagged(
                                    RStmtLet {
                                        ident: Ident { sym: y },
                                        value: Raw(Tagged(
                                            DebugStr { inner: "a . await" },
                                            IO {
                                                input_scope: [Ident { sym: testvar }],
                                                output_scope: [Ident { sym: testvar }],
                                            },
                                        )),
                                    },
                                    IO {
                                        input_scope: [],
                                        output_scope: [Ident { sym: y }],
                                    },
                                )),
                                return_expr: Raw(Tagged(
                                    DebugStr { inner: "y + z" },
                                    IO {
                                        input_scope: [Ident { sym: testvar }],
                                        output_scope: [Ident { sym: testvar }],
                                    },
                                )),
                            }),
                        }),
                    }),
                    else_expr: If(RExprIf {
                        cond_expr: Raw(Tagged(
                            DebugStr { inner: "(x > 10)" },
                            IO {
                                input_scope: [Ident { sym: testvar }],
                                output_scope: [Ident { sym: testvar }],
                            },
                        )),
                        then_expr: Block(RExprBlock {
                            stmt: Expr(Raw(Tagged(
                                DebugStr { inner: "()" },
                                IO {
                                    input_scope: [Ident { sym: testvar }],
                                    output_scope: [Ident { sym: testvar }],
                                },
                            ))),
                            return_expr: Block(RExprBlock {
                                stmt: Let(Tagged(
                                    RStmtLet {
                                        ident: Ident { sym: z },
                                        value: Raw(Tagged(
                                            DebugStr { inner: "a . await" },
                                            IO {
                                                input_scope: [Ident { sym: testvar }],
                                                output_scope: [Ident { sym: testvar }],
                                            },
                                        )),
                                    },
                                    IO {
                                        input_scope: [],
                                        output_scope: [Ident { sym: z }],
                                    },
                                )),
                                return_expr: Raw(Tagged(
                                    DebugStr { inner: "z + x" },
                                    IO {
                                        input_scope: [Ident { sym: testvar }],
                                        output_scope: [Ident { sym: testvar }],
                                    },
                                )),
                            }),
                        }),
                        else_expr: Block(RExprBlock {
                            stmt: Expr(Raw(Tagged(
                                DebugStr { inner: "()" },
                                IO {
                                    input_scope: [Ident { sym: testvar }],
                                    output_scope: [Ident { sym: testvar }],
                                },
                            ))),
                            return_expr: Block(RExprBlock {
                                stmt: Let(Tagged(
                                    RStmtLet {
                                        ident: Ident { sym: z },
                                        value: Raw(Tagged(
                                            DebugStr { inner: "a . await" },
                                            IO {
                                                input_scope: [Ident { sym: testvar }],
                                                output_scope: [Ident { sym: testvar }],
                                            },
                                        )),
                                    },
                                    IO {
                                        input_scope: [],
                                        output_scope: [Ident { sym: z }],
                                    },
                                )),
                                return_expr: Block(RExprBlock {
                                    stmt: Return(RStmtReturn {
                                        value: Raw(Tagged(
                                            DebugStr { inner: "z" },
                                            IO {
                                                input_scope: [Ident { sym: testvar }],
                                                output_scope: [Ident { sym: testvar }],
                                            },
                                        )),
                                    }),
                                    return_expr: Raw(Tagged(
                                        DebugStr { inner: "()" },
                                        IO {
                                            input_scope: [Ident { sym: testvar }],
                                            output_scope: [Ident { sym: testvar }],
                                        },
                                    )),
                                }),
                            }),
                        }),
                    }),
                })),
                return_expr: Raw(Tagged(
                    DebugStr { inner: "x + 2" },
                    IO {
                        input_scope: [Ident { sym: testvar }],
                        output_scope: [Ident { sym: testvar }],
                    },
                )),
            }),
        }),
    })
}
fn hydroflow() {
    Map {
        f: |testvar| (x + 2, (testvar,)),
        input: Union(
            Map {
                f: |testvar| (y + z, (testvar,)),
                input: Map {
                    f: |(y, ())| (y,),
                    input: Map {
                        f: |testvar| (a.await, (testvar,)),
                        input: Map {
                            f: |(z, ())| (z,),
                            input: Map {
                                f: |testvar| (1 + 1, (testvar,)),
                                input: Map {
                                    f: |testvar| ((), (testvar,)),
                                    input: Filter {
                                        f: |(condition, scope)| condition.then_some(scope),
                                        input: Tee {
                                            inner: RefCell {
                                                value: Map {
                                                    f: |testvar| ((x < 5), (testvar,)),
                                                    input: Map {
                                                        f: |(x, ())| (x,),
                                                        input: Map {
                                                            f: |testvar| (1 + 1, (testvar,)),
                                                            input: Map {
                                                                f: |testvar| ((), (testvar,)),
                                                                input: Placeholder,
                                                            },
                                                        },
                                                    },
                                                },
                                            },
                                        },
                                    },
                                },
                            },
                        },
                    },
                },
            },
            Union(
                Map {
                    f: |testvar| (z + x, (testvar,)),
                    input: Map {
                        f: |(z, ())| (z,),
                        input: Map {
                            f: |testvar| (a.await, (testvar,)),
                            input: Map {
                                f: |testvar| ((), (testvar,)),
                                input: Filter {
                                    f: |(condition, scope)| condition.then_some(scope),
                                    input: Tee {
                                        inner: RefCell {
                                            value: Map {
                                                f: |testvar| ((x > 10), (testvar,)),
                                                input: Filter {
                                                    f: |(condition, scope)| {
                                                        condition.then_some(scope)
                                                    },
                                                    input: Map {
                                                        f: |(input_value, scope)| {
                                                            (
                                                                (::std::ops::Not::not)(input_value),
                                                                scope,
                                                            )
                                                        },
                                                        input: Tee {
                                                            inner: RefCell {
                                                                value: Map {
                                                                    f: |testvar| {
                                                                        ((x < 5), (testvar,))
                                                                    },
                                                                    input: Map {
                                                                        f: |(x, ())| (x,),
                                                                        input: Map {
                                                                            f: |testvar| {
                                                                                (1 + 1, (testvar,))
                                                                            },
                                                                            input: Map {
                                                                                f: |testvar| {
                                                                                    ((), (testvar,))
                                                                                },
                                                                                input: Placeholder,
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    },
                                                },
                                            },
                                        },
                                    },
                                },
                            },
                        },
                    },
                },
                Map {
                    f: |testvar| ((), (testvar,)),
                    input: Map {
                        f: |(z, ())| (z,),
                        input: Map {
                            f: |testvar| (a.await, (testvar,)),
                            input: Map {
                                f: |testvar| ((), (testvar,)),
                                input: Filter {
                                    f: |(condition, scope)| condition.then_some(scope),
                                    input: Map {
                                        f: |(input_value, scope)| {
                                            ((::std::ops::Not::not)(input_value), scope)
                                        },
                                        input: Tee {
                                            inner: RefCell {
                                                value: Map {
                                                    f: |testvar| ((x > 10), (testvar,)),
                                                    input: Filter {
                                                        f: |(condition, scope)| {
                                                            condition.then_some(scope)
                                                        },
                                                        input: Map {
                                                            f: |(input_value, scope)| {
                                                                (
                                                                    (::std::ops::Not::not)(
                                                                        input_value,
                                                                    ),
                                                                    scope,
                                                                )
                                                            },
                                                            input: Tee {
                                                                inner: RefCell {
                                                                    value: Map {
                                                                        f: |testvar| {
                                                                            ((x < 5), (testvar,))
                                                                        },
                                                                        input: Map {
                                                                            f: |(x, ())| (x,),
                                                                            input: Map {
                                                                                f: |testvar| {
                                                                                    (
                                                                                        1 + 1,
                                                                                        (testvar,),
                                                                                    )
                                                                                },
                                                                                input: Map {
                                                                                    f: |testvar| {
                                                                                        (() , (testvar ,))
                                                                                    },
                                                                                    input:
                                                                                        Placeholder,
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    },
                                                },
                                            },
                                        },
                                    },
                                },
                            },
                        },
                    },
                },
            ),
        ),
    }
}
