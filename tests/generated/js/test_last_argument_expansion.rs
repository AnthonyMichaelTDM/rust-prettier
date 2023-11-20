#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_arrow_js_format_1_426c2092() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export default function searchUsers(action$) {\n  return action$.ofType(ActionTypes.SEARCHED_USERS)\n    .map(action => action.payload.query)\n    .filter(q => !!q)\n    .switchMap(q =>\n      Observable.timer(800) // debounce\n        .takeUntil(action$.ofType(ActionTypes.CLEARED_SEARCH_RESULTS))\n        .mergeMap(() => Observable.merge(\n          Observable.of(replace(\\`?q=\\${q}\\`)),\n          ajax.getJSON(\\`https://api.github.com/search/users?q=\\${q}\\`)\n            .map(res => res.items)\n            .map(receiveUsers)\n        ))\n    );\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export default function searchUsers(action$) {\n  return action$\n    .ofType(ActionTypes.SEARCHED_USERS)\n    .map((action) => action.payload.query)\n    .filter((q) => !!q)\n    .switchMap((q) =>\n      Observable.timer(800) // debounce\n        .takeUntil(action$.ofType(ActionTypes.CLEARED_SEARCH_RESULTS))\n        .mergeMap(() =>\n          Observable.merge(\n            Observable.of(replace(\\`?q=\\${q}\\`)),\n            ajax\n              .getJSON(\\`https://api.github.com/search/users?q=\\${q}\\`)\n              .map((res) => res.items)\n              .map(receiveUsers),\n          ),\n        ),\n    );\n}");
}
#[test]
fn test_assignment_pattern_js_format_1_4c710d58() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("bob.doL(({ a, b = () => {\n  console.log;\n}}) => something.else.else({}));");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "bob.doL(\n  ({\n    a,\n    b = () => {\n      console.log;\n    },\n  }) => something.else.else({}),\n);");
}
#[test]
fn test_break_parent_js_format_1_b78e4c28() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("({\n  processors: [\n    require(\"autoprefixer\", {\n      browsers: [\"> 1%\", \"last 2 versions\", \"ie >= 11\", \"Firefox ESR\"]\n    }),\n    require(\"postcss-url\")({\n      url: url =>\n        url.startsWith(\"/\") || /^[a-z]+:/.test(url) ? url : \\`/static/\\${url}\\`\n    })\n  ]\n});\n\ntrue\n  ? test({\n      a: 1\n    })\n  : <div\n      a={123412342314}\n      b={123412341234}\n      c={123412341234}\n      d={123412341234}\n      e={123412341234}\n      f={123412341234}\n      g={123412341234}\n    />;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "({\n  processors: [\n    require(\"autoprefixer\", {\n      browsers: [\"> 1%\", \"last 2 versions\", \"ie >= 11\", \"Firefox ESR\"],\n    }),\n    require(\"postcss-url\")({\n      url: (url) =>\n        url.startsWith(\"/\") || /^[a-z]+:/.test(url) ? url : \\`/static/\\${url}\\`,\n    }),\n  ],\n});\n\ntrue ? (\n  test({\n    a: 1,\n  })\n) : (\n  <div\n    a={123412342314}\n    b={123412341234}\n    c={123412341234}\n    d={123412341234}\n    e={123412341234}\n    f={123412341234}\n    g={123412341234}\n  />\n);");
}
#[test]
fn test_dangling_comment_in_arrow_function_js_format_1_cd54bb2b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo(\n  (\n    // foo\n  ) => {}\n);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo(() =>\n  // foo\n  {},\n);");
}
#[test]
fn test_edge_case_js_format_1_ce5e343b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n\na(\n  SomethingVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong,\n  [\n    {\n      SomethingVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong: 1\n    }\n  ]\n);\n\nexports.examples = [\n  {\n    render: withGraphQLQuery(\n      'node(1234567890){image{uri}}',\n      function(container, data) {\n        return (\n          <div>\n            <InlineBlock>\n              <img\n                src={data[1234567890].image.uri}\n                style={{position: 'absolute', top: '0', left: '0', zIndex:'-1'}}\n              />\n            </InlineBlock>\n          </div>\n        );\n      }\n    )\n  }\n];\n\nsomeReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReally.a([\n  [],\n  // comment\n  [],\n]);\n\n(function webpackUniversalModuleDefinition() {})(this, function(__WEBPACK_EXTERNAL_MODULE_85__, __WEBPACK_EXTERNAL_MODULE_115__) {\nreturn /******/ (function(modules) { // webpackBootstrap\n\n/******/ })\n/************************************************************************/\n/******/ ([\n/* 0 */\n/***/ function(module, exports, __webpack_require__) {\n\n/***/ },\n/* 1 */\n/***/ function(module, exports, __webpack_require__) {\n\n/***/ },\n/* 2 */\n/***/ function(module, exports, __webpack_require__) {\n\n/***/ }\n/******/ ])\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a(\n  SomethingVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong,\n  [\n    {\n      SomethingVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong: 1,\n    },\n  ],\n);\n\nexports.examples = [\n  {\n    render: withGraphQLQuery(\n      \"node(1234567890){image{uri}}\",\n      function (container, data) {\n        return (\n          <div>\n            <InlineBlock>\n              <img\n                src={data[1234567890].image.uri}\n                style={{\n                  position: \"absolute\",\n                  top: \"0\",\n                  left: \"0\",\n                  zIndex: \"-1\",\n                }}\n              />\n            </InlineBlock>\n          </div>\n        );\n      },\n    ),\n  },\n];\n\nsomeReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReally.a(\n  [\n    [],\n    // comment\n    [],\n  ],\n);\n\n(function webpackUniversalModuleDefinition() {})(\n  this,\n  function (__WEBPACK_EXTERNAL_MODULE_85__, __WEBPACK_EXTERNAL_MODULE_115__) {\n    return /******/ (function (modules) {\n      // webpackBootstrap\n      /******/\n    })(\n      /************************************************************************/\n      /******/ [\n        /* 0 */\n        /***/ function (module, exports, __webpack_require__) {\n          /***/\n        },\n        /* 1 */\n        /***/ function (module, exports, __webpack_require__) {\n          /***/\n        },\n        /* 2 */\n        /***/ function (module, exports, __webpack_require__) {\n          /***/\n        },\n        /******/\n      ],\n    );\n  },\n);");
}
#[test]
fn test_embed_js_format_1_32c1037a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("foo(/* HTML */ \\`<!-- bar1 --> bar <!-- bar2 -->\\`);\nfoo(/* HTML */ \\` <!-- bar1 --> bar <!-- bar2 --> \\`);\nfoo(/* HTML */ \\`<div><p>bar</p>foo</div>\\`);\nfoo(/* HTML */ \\` <div><p>bar</p>foo</div> \\`);\nfoo(/* GraphQL */ \\`query { foo { bar } }\\`);\nfoo(/* ... */ css\\`color:magenta\\`);\nconst a = b => /* HTML */ \\`<!-- bar1 --> bar <!-- bar2 -->\\`\nconst c = b => /* HTML */ \\` <!-- bar1 --> bar <!-- bar2 --> \\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "foo(\n  /* HTML */ \\`<!-- bar1 -->\n    bar\n    <!-- bar2 -->\\`,\n);\nfoo(/* HTML */ \\`\n  <!-- bar1 -->\n  bar\n  <!-- bar2 -->\n\\`);\nfoo(\n  /* HTML */ \\`<div>\n    <p>bar</p>\n    foo\n  </div>\\`,\n);\nfoo(/* HTML */ \\`\n  <div>\n    <p>bar</p>\n    foo\n  </div>\n\\`);\nfoo(/* GraphQL */ \\`\n  query {\n    foo {\n      bar\n    }\n  }\n\\`);\nfoo(/* ... */ css\\`\n  color: magenta;\n\\`);\nconst a = (b) =>\n  /* HTML */ \\`<!-- bar1 -->\n    bar\n    <!-- bar2 -->\\`;\nconst c = (b) => /* HTML */ \\`\n  <!-- bar1 -->\n  bar\n  <!-- bar2 -->\n\\`;");
}
#[test]
fn test_empty_lines_js_format_1_59bb0024() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("all_verylongcall_verylongcall_verylongcall_verylongcall_verylongcall(\n  (a,\n\n   b) => {\n    console.log()\n  }\n)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "all_verylongcall_verylongcall_verylongcall_verylongcall_verylongcall(\n  (\n    a,\n\n    b,\n  ) => {\n    console.log();\n  },\n);");
}
#[test]
fn test_empty_object_js_format_1_7e3ee37a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("func(first, second, third, fourth, fifth, aReallyLongArgumentsListToForceItToBreak, {\n  // comment\n})\n\nfunc({\n  // comment\n})\n\nfunc(\n  {} // comment\n)\n\nfunc(\n  {}\n  // comment\n)\n\nfunc(\n  // comment\n  {}\n)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "func(\n  first,\n  second,\n  third,\n  fourth,\n  fifth,\n  aReallyLongArgumentsListToForceItToBreak,\n  {\n    // comment\n  },\n);\n\nfunc({\n  // comment\n});\n\nfunc(\n  {}, // comment\n);\n\nfunc(\n  {},\n  // comment\n);\n\nfunc(\n  // comment\n  {},\n);");
}
#[test]
fn test_function_body_in_mode_break_js_format_1_b18867b1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("fs.readdirSync(suiteLoc).forEach(function (testName) {\n  (skip ? it.skip : it)(\n    testName,\n    buildTest(binName, testName, opts),\n    2_000_000,\n  );\n});\n\n{\n  (skip ? it.skip : it)(\n    testName,\n    buildTest(binName, testName, opts),\n    2_000_000,\n  );\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "fs.readdirSync(suiteLoc).forEach(function (testName) {\n  (skip ? it.skip : it)(\n    testName,\n    buildTest(binName, testName, opts),\n    2_000_000,\n  );\n});\n\n{\n  (skip ? it.skip : it)(\n    testName,\n    buildTest(binName, testName, opts),\n    2_000_000,\n  );\n}");
}
#[test]
fn test_function_expression_js_format_1_ccba622e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function* mySagas() {\n  yield effects.takeEvery(\n    rexpress.actionTypes.REQUEST_START,\n    function*({ id }) {\n      console.log(id);\n      yield rexpress.actions(store).writeHead(id, 400);\n      yield rexpress.actions(store).end(id, 'pong');\n      console.log('pong');\n    }\n  );\n}\n\nfunction mySagas2() {\n  return effects.takeEvery(\n    rexpress.actionTypes.REQUEST_START,\n    function({ id }) {\n      console.log(id);\n    }\n  );\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function* mySagas() {\n  yield effects.takeEvery(\n    rexpress.actionTypes.REQUEST_START,\n    function* ({ id }) {\n      console.log(id);\n      yield rexpress.actions(store).writeHead(id, 400);\n      yield rexpress.actions(store).end(id, \"pong\");\n      console.log(\"pong\");\n    },\n  );\n}\n\nfunction mySagas2() {\n  return effects.takeEvery(\n    rexpress.actionTypes.REQUEST_START,\n    function ({ id }) {\n      console.log(id);\n    },\n  );\n}");
}
#[test]
fn test_function_expression_issue_2239_js_format_1_21b54d55() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("someFunctionCallWithBigArgumentsAndACallback(thisArgumentIsQuiteLong, function(cool) {\n  return cool\n})") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "someFunctionCallWithBigArgumentsAndACallback(\n  thisArgumentIsQuiteLong,\n  function (cool) {\n    return cool;\n  },\n);");
}
#[test]
fn test_issue_7518_js_format_1_4f97f087() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const Broken = React.forwardRef(({\n\tchildren,\n\t// 1\n\t// 2\n\ttitle,\n\thidden,\n\t// 3\n}, ref) => (\n\t<div ref={ref}>\n\t\t{children}\n\t</div>\n))") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const Broken = React.forwardRef(\n  (\n    {\n      children,\n      // 1\n      // 2\n      title,\n      hidden,\n      // 3\n    },\n    ref,\n  ) => <div ref={ref}>{children}</div>,\n);");
}
#[test]
fn test_issue_10708_js_format_1_ef9a97da() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "bob\n\t.doL(({ a, b: {\n\t\t// comment\n\t}}) => something\n\t\t.else\n      \t.else({}))",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "bob.doL(\n  ({\n    a,\n    b: {\n      // comment\n    },\n  }) => something.else.else({}),\n);");
}
#[test]
fn test_jsx_js_format_1_7d07eb33() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const els = items.map(item => (\n  <div className=\"whatever\">\n    <span>{children}</span>\n  </div>\n));") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const els = items.map((item) => (\n  <div className=\"whatever\">\n    <span>{children}</span>\n  </div>\n));");
}
#[test]
fn test_number_only_array_js_format_1_3463f89e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("instantiate(game, [\n  transform([-0.7, 0.5, 0]),\n  render_colored_diffuse(game.MaterialDiffuse, game.Meshes[\"monkey_flat\"], [1, 1, 0.3, 1]),\n]);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "instantiate(game, [\n  transform([-0.7, 0.5, 0]),\n  render_colored_diffuse(\n    game.MaterialDiffuse,\n    game.Meshes[\"monkey_flat\"],\n    [1, 1, 0.3, 1],\n  ),\n]);");
}
#[test]
fn test_object_js_format_1_7d3c79b9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const formatData = pipe(\n  zip,\n  map(([ ref, data ]) => ({\n    nodeId: ref.nodeId.toString(),\n    ...attributeFromDataValue(ref.attributeId, data)\n  })),\n  groupBy(prop('nodeId')),\n  map(mergeAll),\n  values\n);\n\nexport const setProp = y => ({\n  ...y,\n  a: 'very, very, very long very, very long text'\n});\n\nexport const log = y => {\n  console.log('very, very, very long very, very long text')\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const formatData = pipe(\n  zip,\n  map(([ref, data]) => ({\n    nodeId: ref.nodeId.toString(),\n    ...attributeFromDataValue(ref.attributeId, data),\n  })),\n  groupBy(prop(\"nodeId\")),\n  map(mergeAll),\n  values,\n);\n\nexport const setProp = (y) => ({\n  ...y,\n  a: \"very, very, very long very, very long text\",\n});\n\nexport const log = (y) => {\n  console.log(\"very, very, very long very, very long text\");\n};");
}
#[test]
fn test_overflow_js_format_1_9f458c14() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("SuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperLongCall((err, result) => {\n  // comment\n});\n\nfunc(one, two, three, four, five, six, seven, eig, is, this, too, long, no, []);\nfunc(one, two, three, four, five, six, seven, eig, is, this, too, long, yes, []);\nfunc(one, two, three, four, five, six, seven, eig, is, this, too, long, yes, [\n  // Comments\n]);\nfunc(five, six, seven, eig, is, this, too, long, yes, [\n  // Comments\n]);\n\nfunc(one, two, three, four, five, six, seven, eig, is, this, too, long, no, {});\nfunc(one, two, three, four, five, six, seven, eig, is, this, too, long, yes, {});\nfunc(one, two, three, four, five, six, seven, eig, is, this, too, long, yes, {\n  // Comments\n});\n\nfoo(\n  (\n    one,\n    two,\n    three,\n    four,\n    five,\n    six,\n    seven,\n    eight,\n    nine,\n    ten,\n    eleven,\n    twelve,\n    thirteen,\n    fourteen,\n  ) => {},\n);\n\nconst contentTypes = function(tile, singleSelection) {\n  return compute(\n    function contentTypesContentTypes(\n      tile,\n      searchString = '',\n      filteredContentTypes = [],\n      contentTypesArray = [],\n      selectedGroup,\n      singleSelection) {\n      selectedGroup = (tile.state && tile.state.group) || selectedGroup;\n    }\n  );\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "SuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperLongCall(\n  (err, result) => {\n    // comment\n  },\n);\n\nfunc(one, two, three, four, five, six, seven, eig, is, this, too, long, no, []);\nfunc(\n  one,\n  two,\n  three,\n  four,\n  five,\n  six,\n  seven,\n  eig,\n  is,\n  this,\n  too,\n  long,\n  yes,\n  [],\n);\nfunc(one, two, three, four, five, six, seven, eig, is, this, too, long, yes, [\n  // Comments\n]);\nfunc(five, six, seven, eig, is, this, too, long, yes, [\n  // Comments\n]);\n\nfunc(one, two, three, four, five, six, seven, eig, is, this, too, long, no, {});\nfunc(\n  one,\n  two,\n  three,\n  four,\n  five,\n  six,\n  seven,\n  eig,\n  is,\n  this,\n  too,\n  long,\n  yes,\n  {},\n);\nfunc(one, two, three, four, five, six, seven, eig, is, this, too, long, yes, {\n  // Comments\n});\n\nfoo(\n  (\n    one,\n    two,\n    three,\n    four,\n    five,\n    six,\n    seven,\n    eight,\n    nine,\n    ten,\n    eleven,\n    twelve,\n    thirteen,\n    fourteen,\n  ) => {},\n);\n\nconst contentTypes = function (tile, singleSelection) {\n  return compute(function contentTypesContentTypes(\n    tile,\n    searchString = \"\",\n    filteredContentTypes = [],\n    contentTypesArray = [],\n    selectedGroup,\n    singleSelection,\n  ) {\n    selectedGroup = (tile.state && tile.state.group) || selectedGroup;\n  });\n};");
}
