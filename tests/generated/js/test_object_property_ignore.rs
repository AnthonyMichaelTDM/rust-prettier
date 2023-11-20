#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_ignore_js_trailing_commaall_format_1_407d03fd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("foo = {\n  // prettier-ignore\n  bar:            1,\n}\n\nfoo = {\n  _: '',\n  // prettier-ignore\n  bar:            1,\n}\n\n/* comments */\nfoo = {\n  _: '',\n  // prettier-ignore\n  bar:            1,         // comment\n}\n\nfoo = {\n  _: '',\n  // prettier-ignore\n  bar:            1,         /* comment */\n}\n\nfoo = {\n  _: '',\n  // prettier-ignore\n  bar:            /* comment */          1,\n}\n\n/* SpreadElement */\nfoo = {\n  _: '',\n  // prettier-ignore\n  ...bar,\n}\n\n// Nested\nfoo = {\n  baz: {\n  // prettier-ignore\n  foo: [1, 2,    3]\n},\n  // prettier-ignore\n  bar:            1,\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "foo = {\n  // prettier-ignore\n  bar:            1,\n};\n\nfoo = {\n  _: \"\",\n  // prettier-ignore\n  bar:            1,\n};\n\n/* comments */\nfoo = {\n  _: \"\",\n  // prettier-ignore\n  bar:            1, // comment\n};\n\nfoo = {\n  _: \"\",\n  // prettier-ignore\n  bar:            1 /* comment */,\n};\n\nfoo = {\n  _: \"\",\n  // prettier-ignore\n  bar:            /* comment */          1,\n};\n\n/* SpreadElement */\nfoo = {\n  _: \"\",\n  // prettier-ignore\n  ...bar,\n};\n\n// Nested\nfoo = {\n  baz: {\n    // prettier-ignore\n    foo: [1, 2,    3],\n  },\n  // prettier-ignore\n  bar:            1,\n};");
}
#[test]
fn test_ignore_js_trailing_commaes_5_format_1_407d03fd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .trailing_comma("es5")
        .parsers(vec!["babel", "flow", "typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("foo = {\n  // prettier-ignore\n  bar:            1,\n}\n\nfoo = {\n  _: '',\n  // prettier-ignore\n  bar:            1,\n}\n\n/* comments */\nfoo = {\n  _: '',\n  // prettier-ignore\n  bar:            1,         // comment\n}\n\nfoo = {\n  _: '',\n  // prettier-ignore\n  bar:            1,         /* comment */\n}\n\nfoo = {\n  _: '',\n  // prettier-ignore\n  bar:            /* comment */          1,\n}\n\n/* SpreadElement */\nfoo = {\n  _: '',\n  // prettier-ignore\n  ...bar,\n}\n\n// Nested\nfoo = {\n  baz: {\n  // prettier-ignore\n  foo: [1, 2,    3]\n},\n  // prettier-ignore\n  bar:            1,\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "foo = {\n  // prettier-ignore\n  bar:            1,\n};\n\nfoo = {\n  _: \"\",\n  // prettier-ignore\n  bar:            1,\n};\n\n/* comments */\nfoo = {\n  _: \"\",\n  // prettier-ignore\n  bar:            1, // comment\n};\n\nfoo = {\n  _: \"\",\n  // prettier-ignore\n  bar:            1 /* comment */,\n};\n\nfoo = {\n  _: \"\",\n  // prettier-ignore\n  bar:            /* comment */          1,\n};\n\n/* SpreadElement */\nfoo = {\n  _: \"\",\n  // prettier-ignore\n  ...bar,\n};\n\n// Nested\nfoo = {\n  baz: {\n    // prettier-ignore\n    foo: [1, 2,    3],\n  },\n  // prettier-ignore\n  bar:            1,\n};");
}
#[test]
fn test_ignore_js_trailing_commanone_format_1_407d03fd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .trailing_comma("none")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("foo = {\n  // prettier-ignore\n  bar:            1,\n}\n\nfoo = {\n  _: '',\n  // prettier-ignore\n  bar:            1,\n}\n\n/* comments */\nfoo = {\n  _: '',\n  // prettier-ignore\n  bar:            1,         // comment\n}\n\nfoo = {\n  _: '',\n  // prettier-ignore\n  bar:            1,         /* comment */\n}\n\nfoo = {\n  _: '',\n  // prettier-ignore\n  bar:            /* comment */          1,\n}\n\n/* SpreadElement */\nfoo = {\n  _: '',\n  // prettier-ignore\n  ...bar,\n}\n\n// Nested\nfoo = {\n  baz: {\n  // prettier-ignore\n  foo: [1, 2,    3]\n},\n  // prettier-ignore\n  bar:            1,\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "foo = {\n  // prettier-ignore\n  bar:            1\n};\n\nfoo = {\n  _: \"\",\n  // prettier-ignore\n  bar:            1\n};\n\n/* comments */\nfoo = {\n  _: \"\",\n  // prettier-ignore\n  bar:            1 // comment\n};\n\nfoo = {\n  _: \"\",\n  // prettier-ignore\n  bar:            1 /* comment */\n};\n\nfoo = {\n  _: \"\",\n  // prettier-ignore\n  bar:            /* comment */          1\n};\n\n/* SpreadElement */\nfoo = {\n  _: \"\",\n  // prettier-ignore\n  ...bar\n};\n\n// Nested\nfoo = {\n  baz: {\n    // prettier-ignore\n    foo: [1, 2,    3]\n  },\n  // prettier-ignore\n  bar:            1\n};");
}
#[test]
fn test_issue_5678_js_trailing_commaall_format_1_f2f00d3f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("all")
        .print_width(80)
        .parsers(vec!["babel", "flow", "typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// #5678\nconst refreshTokenPayload = {\n    type: 'refreshToken',\n    sub: this._id,\n    role: this.role,\n  \t// prettier-ignore\n    exp: now + (60 * 60 * 24 * 90), // (90 days)\n  };\n\nexport default {\n  // prettier-ignore\n  protagonist: \"  0\\\\r\\\\n\" +\n               \"0 00\\\\r\\\\n\" +\n               \"00000\\\\r\\\\n\" +\n               \"0 0\\\\r\\\\n\" +\n               \"0 0\",\n\n  // prettier-ignore\n  wall: \"00000\\\\r\\\\n\" +\n        \"00000\\\\r\\\\n\" +\n        \"00000\\\\r\\\\n\" +\n        \"00000\\\\r\\\\n\" +\n        \"00000\",\n\n  // prettier-ignore\n  cheese: \"0\\\\r\\\\n\" +\n          \" 0\\\\r\\\\n\" +\n          \"000\\\\r\\\\n\" +\n          \"00 0\\\\r\\\\n\" +\n          \"00000\",\n\n  // prettier-ignore\n  enemy: \"0   0\\\\r\\\\n\" +\n         \"00 00\\\\r\\\\n\" +\n         \"00000\\\\r\\\\n\" +\n         \"0 0 0\\\\r\\\\n\" +\n         \"00000\",\n\n  // prettier-ignore\n  home: \"00000\\\\r\\\\n\" +\n        \"0   0\\\\r\\\\n\" +\n        \"0   0\\\\r\\\\n\" +\n        \"0   0\\\\r\\\\n\" +\n        \"00000\",\n\n  // prettier-ignore\n  dog: \"00 00\\\\r\\\\n\" +\n       \"00000\\\\r\\\\n\" +\n       \"0   0\\\\r\\\\n\" +\n       \"0 0 0\\\\r\\\\n\" +\n       \" 000 \"\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// #5678\nconst refreshTokenPayload = {\n  type: \"refreshToken\",\n  sub: this._id,\n  role: this.role,\n  // prettier-ignore\n  exp: now + (60 * 60 * 24 * 90), // (90 days)\n};\n\nexport default {\n  // prettier-ignore\n  protagonist: \"  0\\\\r\\\\n\" +\n               \"0 00\\\\r\\\\n\" +\n               \"00000\\\\r\\\\n\" +\n               \"0 0\\\\r\\\\n\" +\n               \"0 0\",\n\n  // prettier-ignore\n  wall: \"00000\\\\r\\\\n\" +\n        \"00000\\\\r\\\\n\" +\n        \"00000\\\\r\\\\n\" +\n        \"00000\\\\r\\\\n\" +\n        \"00000\",\n\n  // prettier-ignore\n  cheese: \"0\\\\r\\\\n\" +\n          \" 0\\\\r\\\\n\" +\n          \"000\\\\r\\\\n\" +\n          \"00 0\\\\r\\\\n\" +\n          \"00000\",\n\n  // prettier-ignore\n  enemy: \"0   0\\\\r\\\\n\" +\n         \"00 00\\\\r\\\\n\" +\n         \"00000\\\\r\\\\n\" +\n         \"0 0 0\\\\r\\\\n\" +\n         \"00000\",\n\n  // prettier-ignore\n  home: \"00000\\\\r\\\\n\" +\n        \"0   0\\\\r\\\\n\" +\n        \"0   0\\\\r\\\\n\" +\n        \"0   0\\\\r\\\\n\" +\n        \"00000\",\n\n  // prettier-ignore\n  dog: \"00 00\\\\r\\\\n\" +\n       \"00000\\\\r\\\\n\" +\n       \"0   0\\\\r\\\\n\" +\n       \"0 0 0\\\\r\\\\n\" +\n       \" 000 \",\n};");
}
#[test]
fn test_issue_5678_js_trailing_commaes_5_format_1_f2f00d3f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("es5")
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// #5678\nconst refreshTokenPayload = {\n    type: 'refreshToken',\n    sub: this._id,\n    role: this.role,\n  \t// prettier-ignore\n    exp: now + (60 * 60 * 24 * 90), // (90 days)\n  };\n\nexport default {\n  // prettier-ignore\n  protagonist: \"  0\\\\r\\\\n\" +\n               \"0 00\\\\r\\\\n\" +\n               \"00000\\\\r\\\\n\" +\n               \"0 0\\\\r\\\\n\" +\n               \"0 0\",\n\n  // prettier-ignore\n  wall: \"00000\\\\r\\\\n\" +\n        \"00000\\\\r\\\\n\" +\n        \"00000\\\\r\\\\n\" +\n        \"00000\\\\r\\\\n\" +\n        \"00000\",\n\n  // prettier-ignore\n  cheese: \"0\\\\r\\\\n\" +\n          \" 0\\\\r\\\\n\" +\n          \"000\\\\r\\\\n\" +\n          \"00 0\\\\r\\\\n\" +\n          \"00000\",\n\n  // prettier-ignore\n  enemy: \"0   0\\\\r\\\\n\" +\n         \"00 00\\\\r\\\\n\" +\n         \"00000\\\\r\\\\n\" +\n         \"0 0 0\\\\r\\\\n\" +\n         \"00000\",\n\n  // prettier-ignore\n  home: \"00000\\\\r\\\\n\" +\n        \"0   0\\\\r\\\\n\" +\n        \"0   0\\\\r\\\\n\" +\n        \"0   0\\\\r\\\\n\" +\n        \"00000\",\n\n  // prettier-ignore\n  dog: \"00 00\\\\r\\\\n\" +\n       \"00000\\\\r\\\\n\" +\n       \"0   0\\\\r\\\\n\" +\n       \"0 0 0\\\\r\\\\n\" +\n       \" 000 \"\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// #5678\nconst refreshTokenPayload = {\n  type: \"refreshToken\",\n  sub: this._id,\n  role: this.role,\n  // prettier-ignore\n  exp: now + (60 * 60 * 24 * 90), // (90 days)\n};\n\nexport default {\n  // prettier-ignore\n  protagonist: \"  0\\\\r\\\\n\" +\n               \"0 00\\\\r\\\\n\" +\n               \"00000\\\\r\\\\n\" +\n               \"0 0\\\\r\\\\n\" +\n               \"0 0\",\n\n  // prettier-ignore\n  wall: \"00000\\\\r\\\\n\" +\n        \"00000\\\\r\\\\n\" +\n        \"00000\\\\r\\\\n\" +\n        \"00000\\\\r\\\\n\" +\n        \"00000\",\n\n  // prettier-ignore\n  cheese: \"0\\\\r\\\\n\" +\n          \" 0\\\\r\\\\n\" +\n          \"000\\\\r\\\\n\" +\n          \"00 0\\\\r\\\\n\" +\n          \"00000\",\n\n  // prettier-ignore\n  enemy: \"0   0\\\\r\\\\n\" +\n         \"00 00\\\\r\\\\n\" +\n         \"00000\\\\r\\\\n\" +\n         \"0 0 0\\\\r\\\\n\" +\n         \"00000\",\n\n  // prettier-ignore\n  home: \"00000\\\\r\\\\n\" +\n        \"0   0\\\\r\\\\n\" +\n        \"0   0\\\\r\\\\n\" +\n        \"0   0\\\\r\\\\n\" +\n        \"00000\",\n\n  // prettier-ignore\n  dog: \"00 00\\\\r\\\\n\" +\n       \"00000\\\\r\\\\n\" +\n       \"0   0\\\\r\\\\n\" +\n       \"0 0 0\\\\r\\\\n\" +\n       \" 000 \",\n};");
}
#[test]
fn test_issue_5678_js_trailing_commanone_format_1_f2f00d3f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("none")
        .print_width(80)
        .parsers(vec!["babel", "flow", "typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// #5678\nconst refreshTokenPayload = {\n    type: 'refreshToken',\n    sub: this._id,\n    role: this.role,\n  \t// prettier-ignore\n    exp: now + (60 * 60 * 24 * 90), // (90 days)\n  };\n\nexport default {\n  // prettier-ignore\n  protagonist: \"  0\\\\r\\\\n\" +\n               \"0 00\\\\r\\\\n\" +\n               \"00000\\\\r\\\\n\" +\n               \"0 0\\\\r\\\\n\" +\n               \"0 0\",\n\n  // prettier-ignore\n  wall: \"00000\\\\r\\\\n\" +\n        \"00000\\\\r\\\\n\" +\n        \"00000\\\\r\\\\n\" +\n        \"00000\\\\r\\\\n\" +\n        \"00000\",\n\n  // prettier-ignore\n  cheese: \"0\\\\r\\\\n\" +\n          \" 0\\\\r\\\\n\" +\n          \"000\\\\r\\\\n\" +\n          \"00 0\\\\r\\\\n\" +\n          \"00000\",\n\n  // prettier-ignore\n  enemy: \"0   0\\\\r\\\\n\" +\n         \"00 00\\\\r\\\\n\" +\n         \"00000\\\\r\\\\n\" +\n         \"0 0 0\\\\r\\\\n\" +\n         \"00000\",\n\n  // prettier-ignore\n  home: \"00000\\\\r\\\\n\" +\n        \"0   0\\\\r\\\\n\" +\n        \"0   0\\\\r\\\\n\" +\n        \"0   0\\\\r\\\\n\" +\n        \"00000\",\n\n  // prettier-ignore\n  dog: \"00 00\\\\r\\\\n\" +\n       \"00000\\\\r\\\\n\" +\n       \"0   0\\\\r\\\\n\" +\n       \"0 0 0\\\\r\\\\n\" +\n       \" 000 \"\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// #5678\nconst refreshTokenPayload = {\n  type: \"refreshToken\",\n  sub: this._id,\n  role: this.role,\n  // prettier-ignore\n  exp: now + (60 * 60 * 24 * 90) // (90 days)\n};\n\nexport default {\n  // prettier-ignore\n  protagonist: \"  0\\\\r\\\\n\" +\n               \"0 00\\\\r\\\\n\" +\n               \"00000\\\\r\\\\n\" +\n               \"0 0\\\\r\\\\n\" +\n               \"0 0\",\n\n  // prettier-ignore\n  wall: \"00000\\\\r\\\\n\" +\n        \"00000\\\\r\\\\n\" +\n        \"00000\\\\r\\\\n\" +\n        \"00000\\\\r\\\\n\" +\n        \"00000\",\n\n  // prettier-ignore\n  cheese: \"0\\\\r\\\\n\" +\n          \" 0\\\\r\\\\n\" +\n          \"000\\\\r\\\\n\" +\n          \"00 0\\\\r\\\\n\" +\n          \"00000\",\n\n  // prettier-ignore\n  enemy: \"0   0\\\\r\\\\n\" +\n         \"00 00\\\\r\\\\n\" +\n         \"00000\\\\r\\\\n\" +\n         \"0 0 0\\\\r\\\\n\" +\n         \"00000\",\n\n  // prettier-ignore\n  home: \"00000\\\\r\\\\n\" +\n        \"0   0\\\\r\\\\n\" +\n        \"0   0\\\\r\\\\n\" +\n        \"0   0\\\\r\\\\n\" +\n        \"00000\",\n\n  // prettier-ignore\n  dog: \"00 00\\\\r\\\\n\" +\n       \"00000\\\\r\\\\n\" +\n       \"0   0\\\\r\\\\n\" +\n       \"0 0 0\\\\r\\\\n\" +\n       \" 000 \"\n};");
}
