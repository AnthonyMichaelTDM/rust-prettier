#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comment_before_else_js_format_1_4fe5ed44() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow", "typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("if (cond) {\n  stuff;\n} /* comment */ else if (cond) {\n  stuff;\n}\n// comment\nelse {\n  stuff;\n}\n\nif (cond) stuff;\n// comment\nelse stuff;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "if (cond) {\n  stuff;\n} /* comment */ else if (cond) {\n  stuff;\n}\n// comment\nelse {\n  stuff;\n}\n\nif (cond) stuff;\n// comment\nelse stuff;");
}
#[test]
fn test_else_js_format_1_b6121974() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Both functions below should be formatted exactly the same\n\nfunction f() {\n  if (position)\n    return {name: pair};\n  else\n    return {name: pair.substring(0, position), value: pair.substring(position + 1)};\n}\n\nfunction f() {\n  if (position)\n    return {name: pair};\n  else\n    return {\n      name: pair.substring(0, position),\n      value: pair.substring(position + 1)\n    };\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Both functions below should be formatted exactly the same\n\nfunction f() {\n  if (position) return { name: pair };\n  else\n    return {\n      name: pair.substring(0, position),\n      value: pair.substring(position + 1),\n    };\n}\n\nfunction f() {\n  if (position) return { name: pair };\n  else\n    return {\n      name: pair.substring(0, position),\n      value: pair.substring(position + 1),\n    };\n}");
}
#[test]
fn test_expr_and_same_line_comments_js_format_1_5eb9ac88() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow", "typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("     if (a === 0) doSomething(); // comment A1\nelse if (a === 1) doSomethingElse(); // comment B1\nelse if (a === 2) doSomethingElse(); // comment C1\n\n     if (a === 0) doSomething(); /* comment A2 */\nelse if (a === 1) doSomethingElse(); /* comment B2 */\nelse if (a === 2) doSomethingElse(); /* comment C2 */\n\n     if (a === 0) expr; // comment A3\nelse if (a === 1) expr; // comment B3\nelse if (a === 2) expr; // comment C3\n\n     if (a === 0) expr; /* comment A4 */\nelse if (a === 1) expr; /* comment B4 */\nelse if (a === 2) expr; /* comment C4 */\n\n     if (a === 0) looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong; // comment A5\nelse if (a === 1) looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong; // comment B5\nelse if (a === 2) looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong; // comment C5\n\nfunction a() {\n  if (a) return; /* comment 6a */\n  else return 2;\n\n  if (a) return 1; /* comment 6b */\n  else return 2;\n\n  if (a) throw e; /* comment 6d */\n  else return 2;\n\n  // TODO[@fisker]: fix this\n  // if (a) var a = 1; /* comment 6e */\n  // else return 2;\n\n  if (a) if (b); /* comment 6f */\n  else return 2;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "if (a === 0) doSomething(); // comment A1\nelse if (a === 1) doSomethingElse(); // comment B1\nelse if (a === 2) doSomethingElse(); // comment C1\n\nif (a === 0) doSomething(); /* comment A2 */\nelse if (a === 1) doSomethingElse(); /* comment B2 */\nelse if (a === 2) doSomethingElse(); /* comment C2 */\n\nif (a === 0) expr; // comment A3\nelse if (a === 1) expr; // comment B3\nelse if (a === 2) expr; // comment C3\n\nif (a === 0) expr; /* comment A4 */\nelse if (a === 1) expr; /* comment B4 */\nelse if (a === 2) expr; /* comment C4 */\n\nif (a === 0)\n  looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong; // comment A5\nelse if (a === 1)\n  looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong; // comment B5\nelse if (a === 2)\n  looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong; // comment C5\n\nfunction a() {\n  if (a) return /* comment 6a */;\n  else return 2;\n\n  if (a) return 1 /* comment 6b */;\n  else return 2;\n\n  if (a) throw e /* comment 6d */;\n  else return 2;\n\n  // TODO[@fisker]: fix this\n  // if (a) var a = 1; /* comment 6e */\n  // else return 2;\n\n  if (a)\n    if (b /* comment 6f */);\n    else return 2;\n}");
}
#[test]
fn test_if_comments_js_format_1_108029f9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("async function f1() {\n  if (untrackedChoice === 0) /* Cancel */ {\n    return null;\n  } else if (untrackedChoice === 1) /* Add */ {\n    await repository.addAll(Array.from(untrackedChanges.keys()));\n    shouldAmend = true;\n  } else if (untrackedChoice === 2) /* Allow Untracked */ {\n    allowUntracked = true;\n  }\n}\n\nasync function f2() {\n  if (untrackedChoice === 0) /* Cancel */\n    null;\n  else if (untrackedChoice === 1) /* Add */\n    shouldAmend = true;\n  else if (untrackedChoice === 2) /* Allow Untracked */\n    allowUntracked = true;\n}\n\nasync function f3() {\n  if (untrackedChoice === 0) /* Cancel */ // Cancel\n    null;\n  else if (untrackedChoice === 1) /* Add */ // Add\n    shouldAmend = true;\n  else if (untrackedChoice === 2) /* Allow Untracked */ // Allow Untracked\n    allowUntracked = true;\n}\n\nasync function f4() {\n  if (untrackedChoice === 0)\n    /* Cancel */ {\n      return null;\n    }\n  else if (untrackedChoice === 1)\n    /* Add */ {\n      await repository.addAll(Array.from(untrackedChanges.keys()));\n      shouldAmend = true;\n    }\n  else if (untrackedChoice === 2)\n    /* Allow Untracked */ {\n      allowUntracked = true;\n    }\n}\n\nasync function f5() {\n  if (untrackedChoice === 0) {\n    /* Cancel */ return null;\n  } else if (untrackedChoice === 1) {\n    /* Add */ await repository.addAll(Array.from(untrackedChanges.keys()));\n    shouldAmend = true;\n  } else if (untrackedChoice === 2) {\n    /* Allow Untracked */ allowUntracked = true;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "async function f1() {\n  if (untrackedChoice === 0) {\n    /* Cancel */ return null;\n  } else if (untrackedChoice === 1) {\n    /* Add */ await repository.addAll(Array.from(untrackedChanges.keys()));\n    shouldAmend = true;\n  } else if (untrackedChoice === 2) {\n    /* Allow Untracked */ allowUntracked = true;\n  }\n}\n\nasync function f2() {\n  if (untrackedChoice === 0) /* Cancel */ null;\n  else if (untrackedChoice === 1) /* Add */ shouldAmend = true;\n  else if (untrackedChoice === 2) /* Allow Untracked */ allowUntracked = true;\n}\n\nasync function f3() {\n  if (untrackedChoice === 0)\n    /* Cancel */ // Cancel\n    null;\n  else if (untrackedChoice === 1)\n    /* Add */ // Add\n    shouldAmend = true;\n  else if (untrackedChoice === 2)\n    /* Allow Untracked */ // Allow Untracked\n    allowUntracked = true;\n}\n\nasync function f4() {\n  if (untrackedChoice === 0) {\n    /* Cancel */ return null;\n  } else if (untrackedChoice === 1) {\n    /* Add */ await repository.addAll(Array.from(untrackedChanges.keys()));\n    shouldAmend = true;\n  } else if (untrackedChoice === 2) {\n    /* Allow Untracked */ allowUntracked = true;\n  }\n}\n\nasync function f5() {\n  if (untrackedChoice === 0) {\n    /* Cancel */ return null;\n  } else if (untrackedChoice === 1) {\n    /* Add */ await repository.addAll(Array.from(untrackedChanges.keys()));\n    shouldAmend = true;\n  } else if (untrackedChoice === 2) {\n    /* Allow Untracked */ allowUntracked = true;\n  }\n}");
}
#[test]
fn test_trailing_comment_js_format_1_4ba2c1a2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("if (code === 92 /* '\\\\' */) {}\nif (code === 92 /* '\\\\' */ /* '\\\\' */) {}\n\nif (code === 92) /* '\\\\' */ {}\nif (code === 92) { /* '\\\\' */ }\n\nif (\n  1\n  // Comment\n) {\n  a;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "if (code === 92 /* '\\\\' */) {\n}\nif (code === 92 /* '\\\\' */ /* '\\\\' */) {\n}\n\nif (code === 92) {\n  /* '\\\\' */\n}\nif (code === 92) {\n  /* '\\\\' */\n}\n\nif (\n  1\n  // Comment\n) {\n  a;\n}");
}
