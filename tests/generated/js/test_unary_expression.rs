#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comments_js_format_1_5e4d0161() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("!x;\n!(x /* foo */);\n!(/* foo */ x);\n!(\n  /* foo */\n  x\n);\n!(\n  x\n  /* foo */\n);\n!(\n  x // foo\n);\n\n!(x + y);\n!(x + y /* foo */);\n!(/* foo */ x + y);\n!(\n  /* foo */\n  x + y\n);\n!(\n  x + y\n  /* foo */\n);\n!(\n  x + y // foo\n);\n\n!(x || y);\n!(/* foo */ x || y);\n!(x || y /* foo */);\n!(\n  /* foo */\n  x || y\n);\n!(\n  x || y\n  /* foo */\n);\n!(\n  x || y // foo\n);\n\n![1, 2, 3];\n!([1, 2, 3] /* foo */);\n!(/* foo */ [1, 2, 3]);\n!(\n  /* foo */\n  [1, 2, 3]\n);\n!(\n  [1, 2, 3]\n  /* foo */\n);\n!(\n  [1, 2, 3] // foo\n);\n\n!{ a: 1, b: 2 };\n!({ a: 1, b: 2 } /* foo */);\n!(/* foo */ { a: 1, b: 2 });\n!(\n  /* foo */\n  { a: 1, b: 2 }\n);\n!(\n  { a: 1, b: 2 }\n  /* foo */\n);\n!(\n  { a: 1, b: 2 } // foo\n)\n\n!function() {\n  return x;\n};\n!(\n  function() {\n    return x;\n  } /* foo */\n);\n!(\n  /* foo */ function() {\n    return x;\n  }\n);\n!(\n  /* foo */\n  function() {\n    return x;\n  }\n);\n!(\n  function() {\n    return x;\n  }\n  /* foo */\n);\n!(\n  function() {\n    return x;\n  } // foo\n)\n\n!+3;\n!(+3 /* foo */);\n!(/* foo */ +3);\n!(\n  /* foo */\n  +3\n);\n!(\n  +3\n  /* foo */\n);\n!(\n  +3 // foo\n);\n\n!+(\n  /* foo */\n  3\n);\n!(/* foo */ +(3 /* foo */));\n!(+(3 /* foo */) /* foo */);\n!(\n  /* foo */\n  +(\n    /* foo */\n    3\n  )\n);\n!(\n  +(\n    3\n    /* foo */\n  )\n  /* foo */\n);\n!(\n  +(\n    3 /* foo */\n  ) // foo\n);\n\n!(x = y);\n!(x = y /* foo */);\n!(/* foo */ x = y);\n!(\n  /* foo */\n  x = y\n);\n!(\n  x = y\n  /* foo */\n);\n!(\n  x = y // foo\n);\n\n!x.y;\n!(x.y /* foo */);\n!(/* foo */ x.y);\n!(\n  /* foo */\n  x.y\n);\n!(\n  x.y\n  /* foo */\n);\n!(\n  x.y // foo\n);\n\n!(x ? y : z);\n!(x ? y : z /* foo */);\n!(/* foo */ x ? y : z);\n!(\n  /* foo */\n  x ? y : z\n);\n!(\n  x ? y : z\n  /* foo */\n);\n!(\n  x ? y : z // foo\n);\n\n!x();\n!(x() /* foo */);\n!(/* foo */ x());\n!(\n  /* foo */\n  x()\n);\n!(\n  x()\n  /* foo */\n);\n!(\n  x() // foo\n);\n\n!new x();\n!(new x() /* foo */);\n!(/* foo */ new x());\n!(\n  /* foo */\n  new x()\n);\n!(\n  new x()\n  /* foo */\n);\n!(\n  new x() // foo\n);\n\n!(x, y);\n!(x, y /* foo */);\n!(/* foo */ x, y);\n!(\n  /* foo */\n  x, y\n);\n!(\n  x, y\n  /* foo */\n);\n!(\n  x.y // foo\n);\n\n!(() => 3);\n!(() => 3 /* foo */);\n!(/* foo */ () => 3);\n!(\n  /* foo */\n  () => 3\n);\n!(\n  () => 3\n  /* foo */\n);\n!(\n  () => 3 // foo\n);\n\nfunction* bar() {\n  !(yield x);\n  !(yield x /* foo */);\n  !(/* foo */ yield x);\n  !(\n    /* foo */\n    yield x\n  );\n  !(\n    yield x\n    /* foo */\n  );\n  !(\n    yield x // foo\n  );\n}\n\nasync function bar2() {\n  !(await x);\n  !(await x /* foo */);\n  !(/* foo */ await x);\n  !(\n  /* foo */\n  await x\n  );\n  !(\n    await x\n    /* foo */\n  );\n  !(\n    await x // foo\n  );\n}") ? ;
    assert_eq ! (formatted , "!x;\n!(x /* foo */);\n!(/* foo */ x);\n!(\n  /* foo */\n  x\n);\n!(\n  x\n  /* foo */\n);\n!(\n  x // foo\n);\n\n!(x + y);\n!((x + y) /* foo */);\n!(/* foo */ (x + y));\n!(\n  /* foo */\n  (x + y)\n);\n!(\n  (x + y)\n  /* foo */\n);\n!(\n  (x + y) // foo\n);\n\n!(x || y);\n!(/* foo */ (x || y));\n!((x || y) /* foo */);\n!(\n  /* foo */\n  (x || y)\n);\n!(\n  (x || y)\n  /* foo */\n);\n!(\n  (x || y) // foo\n);\n\n![1, 2, 3];\n!([1, 2, 3] /* foo */);\n!(/* foo */ [1, 2, 3]);\n!(\n  /* foo */\n  [1, 2, 3]\n);\n!(\n  [1, 2, 3]\n  /* foo */\n);\n!(\n  [1, 2, 3] // foo\n);\n\n!{ a: 1, b: 2 };\n!({ a: 1, b: 2 } /* foo */);\n!(/* foo */ { a: 1, b: 2 });\n!(\n  /* foo */\n  { a: 1, b: 2 }\n);\n!(\n  { a: 1, b: 2 }\n  /* foo */\n);\n!(\n  { a: 1, b: 2 } // foo\n);\n\n!function () {\n  return x;\n};\n!(\n  function () {\n    return x;\n  } /* foo */\n);\n!(\n  /* foo */ function () {\n    return x;\n  }\n);\n!(\n  /* foo */\n  function () {\n    return x;\n  }\n);\n!(\n  function () {\n    return x;\n  }\n  /* foo */\n);\n!(\n  function () {\n    return x;\n  } // foo\n);\n\n!+3;\n!(+3 /* foo */);\n!(/* foo */ +3);\n!(\n  /* foo */\n  +3\n);\n!(\n  +3\n  /* foo */\n);\n!(\n  +3 // foo\n);\n\n!+(\n  /* foo */\n  3\n);\n!(/* foo */ +(3 /* foo */));\n!(+(3 /* foo */) /* foo */);\n!(\n  /* foo */\n  +(\n    /* foo */\n    3\n  )\n);\n!(\n  +(\n    3\n    /* foo */\n  )\n  /* foo */\n);\n!(\n  +(3 /* foo */) // foo\n);\n\n!(x = y);\n!((x = y) /* foo */);\n!(/* foo */ (x = y));\n!(\n  /* foo */\n  (x = y)\n);\n!(\n  (x = y)\n  /* foo */\n);\n!(\n  (x = y) // foo\n);\n\n!x.y;\n!(x.y /* foo */);\n!(/* foo */ x.y);\n!(\n  /* foo */\n  x.y\n);\n!(\n  x.y\n  /* foo */\n);\n!(\n  x.y // foo\n);\n\n!(x ? y : z);\n!((x ? y : z) /* foo */);\n!(/* foo */ (x ? y : z));\n!(\n  /* foo */\n  (x ? y : z)\n);\n!(\n  (x ? y : z)\n  /* foo */\n);\n!(\n  (x ? y : z) // foo\n);\n\n!x();\n!(x() /* foo */);\n!(/* foo */ x());\n!(\n  /* foo */\n  x()\n);\n!(\n  x()\n  /* foo */\n);\n!(\n  x() // foo\n);\n\n!new x();\n!(new x() /* foo */);\n!(/* foo */ new x());\n!(\n  /* foo */\n  new x()\n);\n!(\n  new x()\n  /* foo */\n);\n!(\n  new x() // foo\n);\n\n!(x, y);\n!((x, y) /* foo */);\n!(/* foo */ (x, y));\n!(\n  /* foo */\n  (x, y)\n);\n!(\n  (x, y)\n  /* foo */\n);\n!(\n  x.y // foo\n);\n\n!(() => 3);\n!((() => 3) /* foo */);\n!(/* foo */ (() => 3));\n!(\n  /* foo */\n  (() => 3)\n);\n!(\n  (() => 3)\n  /* foo */\n);\n!(\n  (() => 3) // foo\n);\n\nfunction* bar() {\n  !(yield x);\n  !((yield x) /* foo */);\n  !(/* foo */ (yield x));\n  !(\n    /* foo */\n    (yield x)\n  );\n  !(\n    (yield x)\n    /* foo */\n  );\n  !(\n    (yield x) // foo\n  );\n}\n\nasync function bar2() {\n  !(await x);\n  !((await x) /* foo */);\n  !(/* foo */ (await x));\n  !(\n    /* foo */\n    (await x)\n  );\n  !(\n    (await x)\n    /* foo */\n  );\n  !(\n    (await x) // foo\n  );\n}");
    Ok(())
}
#[test]
fn test_urnary_expression_js_format_1_57234989() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("!!x\nx++\nx--;\n-+1;\nx + + + + 1;\nx + (+ (+ (+ 1)))\nx * +y;\n+x * y;")?;
    assert_eq!(
        formatted,
        "!!x;\nx++;\nx--;\n-+1;\nx + +(+(+1));\nx + +(+(+1));\nx * +y;\n+x * y;"
    );
    Ok(())
}
