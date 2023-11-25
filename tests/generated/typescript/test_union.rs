#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comments_ts_format_1_9906b5bd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Foo = (\n  | \"thing1\" // Comment1\n  | \"thing2\" // Comment2\n)[]; // Final comment1\n\ntype Foo = (\n  | \"thing1\" // Comment1\n  | \"thing2\" // Comment2\n) & Bar; // Final comment2") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Foo = (\n  | \"thing1\" // Comment1\n  | \"thing2\" // Comment2\n)[]; // Final comment1\n\ntype Foo = (\n  | \"thing1\" // Comment1\n  | \"thing2\" // Comment2\n) &\n  Bar; // Final comment2");
}
#[test]
fn test_inlining_ts_format_1_497b703f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface RelayProps {\n  articles: a | null,\n}\ninterface RelayProps {\n  articles: Array<{\n    __id: string,\n  } | null> | null | void,\n}\n\ninterface RelayProps {\n  articles: Array<{\n    __id: string,\n  } | null>\n  | null // articles type may be null\n  | void, // articles type may be void\n}\n\ntype FooBar = null // null\n| { /** x **/\n  y: number;\n  z: string;\n} // this documents the first option\n  | void // this documents the second option\n  ;\n\ntype FooBarWithoutComment = null\n  | {\n  y: number;\n  z: string;\n}\n  | void\n  ;\n\ntype FooBar2 =\n  | Number // this documents the first option\n  | void // this documents the second option\n  ;\n\ntype UploadState<E, EM, D>\n  // The upload hasnt begun yet\n  = {type: \"Not_begun\"}\n  // The upload timed out\n  | {type: \"Timed_out\"}\n  // Failed somewhere on the line\n  | {type: \"Failed\", error: E, errorMsg: EM}\n  // Uploading to aws3 and CreatePostMutation succeeded\n  | {type: \"Success\", data: D};\n\ntype UploadState2<E, EM, D>\n  // The upload hasnt begun yet\n  = A\n  // The upload timed out\n  | B\n  // Failed somewhere on the line\n  | C\n  // Uploading to aws3 and CreatePostMutation succeeded\n  | D;\n\ntype window = Window & {\n  __REDUX_DEVTOOLS_EXTENSION_COMPOSE__: Function;\n};\n\ntype T1 = (number | string)[\"toString\"];\ntype T2 = ((number | string))[\"toString\"];\ntype T3 = (((number | string)))[\"toString\"];\ntype T4 = ((((number | string))))[\"toString\"];\ntype T5 = number | ((arg: any) => void);\ntype T6 = number | (((arg: any) => void));\ntype T7 = number | ((((arg: any) => void)));\ntype T8 = number | (((((arg: any) => void))));") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "interface RelayProps {\n  articles: a | null;\n}\ninterface RelayProps {\n  articles: Array<{\n    __id: string;\n  } | null> | null | void;\n}\n\ninterface RelayProps {\n  articles:\n    | Array<{\n        __id: string;\n      } | null>\n    | null // articles type may be null\n    | void; // articles type may be void\n}\n\ntype FooBar =\n  | null // null\n  | {\n      /** x **/ y: number;\n      z: string;\n    } // this documents the first option\n  | void; // this documents the second option\n\ntype FooBarWithoutComment = null | {\n  y: number;\n  z: string;\n} | void;\n\ntype FooBar2 =\n  | Number // this documents the first option\n  | void; // this documents the second option\n\ntype UploadState<E, EM, D> =\n  // The upload hasnt begun yet\n  | { type: \"Not_begun\" }\n  // The upload timed out\n  | { type: \"Timed_out\" }\n  // Failed somewhere on the line\n  | { type: \"Failed\"; error: E; errorMsg: EM }\n  // Uploading to aws3 and CreatePostMutation succeeded\n  | { type: \"Success\"; data: D };\n\ntype UploadState2<E, EM, D> =\n  // The upload hasnt begun yet\n  | A\n  // The upload timed out\n  | B\n  // Failed somewhere on the line\n  | C\n  // Uploading to aws3 and CreatePostMutation succeeded\n  | D;\n\ntype window = Window & {\n  __REDUX_DEVTOOLS_EXTENSION_COMPOSE__: Function;\n};\n\ntype T1 = (number | string)[\"toString\"];\ntype T2 = (number | string)[\"toString\"];\ntype T3 = (number | string)[\"toString\"];\ntype T4 = (number | string)[\"toString\"];\ntype T5 = number | ((arg: any) => void);\ntype T6 = number | ((arg: any) => void);\ntype T7 = number | ((arg: any) => void);\ntype T8 = number | ((arg: any) => void);");
}
#[test]
fn test_union_parens_ts_format_1_39a0459c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\nexport type A = (\n  | aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n  | bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\n);\n\nexport type B = (\n  aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa |\n  bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\n);\n\nexport type C =\n  | aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n  | bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;\n\nexport type D =\n  aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa |\n  bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;\n\nexport type Multi = (string | number)[];\n\nfunction f(): (string | number) {}\n\nvar x: (string | number);\nvar y: ((string | number));\n\nclass Foo<T extends (string | number)> {}\n\ninterface Interface {\n    i: (X | Y) & Z;\n    j: Partial<(X | Y)>;\n}\n\ntype State = {\n  sharedProperty: any;\n} & (\n  | { discriminant: \"FOO\"; foo: any }\n  | { discriminant: \"BAR\"; bar: any }\n  | { discriminant: \"BAZ\"; baz: any }\n);\n\nconst foo1 = [abc, def, ghi, jkl, mno, pqr, stu, vwx, yz] as (\n  | string\n  | undefined\n)[];\n\nconst foo2: (\n  | AAAAAAAAAAAAAAAAAAAAAA\n  | BBBBBBBBBBBBBBBBBBBBBB\n  | CCCCCCCCCCCCCCCCCCCCCC\n  | DDDDDDDDDDDDDDDDDDDDDD\n)[] = [];\n\nconst foo3: keyof (\n  | AAAAAAAAAAAAAAAAAAAAAA\n  | BBBBBBBBBBBBBBBBBBBBBB\n  | CCCCCCCCCCCCCCCCCCCCCC\n  | DDDDDDDDDDDDDDDDDDDDDD\n) = bar;\n\nconst foo4:\n  | foo\n  | (\n      | AAAAAAAAAAAAAAAAAAAAAA\n      | BBBBBBBBBBBBBBBBBBBBBB\n      | CCCCCCCCCCCCCCCCCCCCCC\n      | DDDDDDDDDDDDDDDDDDDDDD\n    ) = bar;\n\nlet a1 : C;\nlet a2 : | C;\nlet a3 : (| C);\nlet a4 : | (C);\nlet a5 : (| (C));\nlet a6 : /*1*/ | C;\nlet a7 : /*1*/ | (C);\nlet a8 : /*1*/ (| C);\nlet a9 : (/*1*/ | C);\nlet a10: /*1*/ | /*2*/ C;\nlet a11: /*1*/ (| /*2*/ C);\n\nlet aa1: /*1*/ | /*2*/ C | D;\nlet aa2: /*1*/ | /*2*/ C | /*3*/ D;\nlet aa3: /*1*/ | /*2*/ C | /*3*/ D /*4*/;\n\ntype A1  = C;\ntype A2  = | C;\ntype A3  = (| C);\ntype A4  = | (C);\ntype A5  = (| (C));\ntype A6  = /*1*/ | C;\ntype A7  = /*1*/ | (C);\ntype A8  = /*1*/ (| C);\ntype A9  = (/*1*/ | C);\ntype A10 = /*1*/ | /*2*/ C;\ntype A11 = /*1*/ (| /*2*/ C);\ntype A12 = /*1*/ | ( (C));\ntype A13 = /*1*/ ( (C));\n\ntype Aa1 = /*1*/ | /*2*/ C | D;\ntype Aa2 = /*1*/ | /*2*/ C | /*3*/ D;\ntype Aa3 = /*1*/ | /*2*/ C | /*3*/ D /*4*/;\n\ntype C1 = /*1*/ & a | b;\ntype C2 = /*1*/ & a | (b);\ntype C3 = /*1*/ & a | (& b);\ntype C4 = /*1*/ & (a | b);\ntype C5 = /*1*/ (& a | b);\ntype C6 /*0*/ = /*1*/ (& a | b);\n\ntype Ctor = (new () => X) | Y;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export type A =\n  | aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n  | bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;\n\nexport type B =\n  | aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n  | bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;\n\nexport type C =\n  | aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n  | bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;\n\nexport type D =\n  | aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n  | bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;\n\nexport type Multi = (string | number)[];\n\nfunction f(): string | number {}\n\nvar x: string | number;\nvar y: string | number;\n\nclass Foo<T extends string | number> {}\n\ninterface Interface {\n  i: (X | Y) & Z;\n  j: Partial<X | Y>;\n}\n\ntype State = {\n  sharedProperty: any;\n} & (\n  | { discriminant: \"FOO\"; foo: any }\n  | { discriminant: \"BAR\"; bar: any }\n  | { discriminant: \"BAZ\"; baz: any }\n);\n\nconst foo1 = [abc, def, ghi, jkl, mno, pqr, stu, vwx, yz] as (\n  | string\n  | undefined\n)[];\n\nconst foo2: (\n  | AAAAAAAAAAAAAAAAAAAAAA\n  | BBBBBBBBBBBBBBBBBBBBBB\n  | CCCCCCCCCCCCCCCCCCCCCC\n  | DDDDDDDDDDDDDDDDDDDDDD\n)[] = [];\n\nconst foo3: keyof (\n  | AAAAAAAAAAAAAAAAAAAAAA\n  | BBBBBBBBBBBBBBBBBBBBBB\n  | CCCCCCCCCCCCCCCCCCCCCC\n  | DDDDDDDDDDDDDDDDDDDDDD\n) = bar;\n\nconst foo4:\n  | foo\n  | (\n      | AAAAAAAAAAAAAAAAAAAAAA\n      | BBBBBBBBBBBBBBBBBBBBBB\n      | CCCCCCCCCCCCCCCCCCCCCC\n      | DDDDDDDDDDDDDDDDDDDDDD\n    ) = bar;\n\nlet a1: C;\nlet a2: C;\nlet a3: C;\nlet a4: C;\nlet a5: C;\nlet a6: /*1*/ C;\nlet a7: /*1*/ C;\nlet a8: /*1*/ C;\nlet a9: /*1*/ C;\nlet a10: /*1*/ /*2*/ C;\nlet a11: /*1*/ /*2*/ C;\n\nlet aa1: /*1*/ /*2*/ C | D;\nlet aa2: /*1*/ /*2*/ C | /*3*/ D;\nlet aa3: /*1*/ /*2*/ C | /*3*/ D /*4*/;\n\ntype A1 = C;\ntype A2 = C;\ntype A3 = C;\ntype A4 = C;\ntype A5 = C;\ntype A6 /*1*/ = C;\ntype A7 /*1*/ = C;\ntype A8 /*1*/ = C;\ntype A9 /*1*/ = C;\ntype A10 /*1*/ = /*2*/ C;\ntype A11 /*1*/ = /*2*/ C;\ntype A12 /*1*/ = C;\ntype A13 = /*1*/ C;\n\ntype Aa1 = /*1*/ /*2*/ C | D;\ntype Aa2 = /*1*/ /*2*/ C | /*3*/ D;\ntype Aa3 = /*1*/ /*2*/ C | /*3*/ D /*4*/;\n\ntype C1 = /*1*/ a | b;\ntype C2 = /*1*/ a | b;\ntype C3 = /*1*/ a | b;\ntype C4 /*1*/ = a | b;\ntype C5 = /*1*/ a | b;\ntype C6 /*0*/ = /*1*/ a | b;\n\ntype Ctor = (new () => X) | Y;");
}
#[test]
fn test_with_type_params_ts_format_1_d166ebb6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type GetChatsSagaEffects =\n  | CallEffect\n  | PutEffect<\n      | GetUsersRequestedAction\n      | GetChatsSucceededAction\n      | GetChatsFailedAction\n      | GetChatsStartedAction\n    >\n  | SelectEffect") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type GetChatsSagaEffects =\n  | CallEffect\n  | PutEffect<\n      | GetUsersRequestedAction\n      | GetChatsSucceededAction\n      | GetChatsFailedAction\n      | GetChatsStartedAction\n    >\n  | SelectEffect;");
}
