#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_block_comments_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_block_comments_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_block_comments_js_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_block_comments_js_format_1_c0f4802e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("bifornCringerMoshedPerplexSawder\n|> foo1\n|> foo2 /* comment1 */\n|> foo3 /* comment2 */\n|> kochabCooieGameOnOboleUnweave\n|> glimseGlyphsHazardNoopsTieTie;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "bifornCringerMoshedPerplexSawder\n  |> foo1\n  |> foo2 /* comment1 */\n  |> foo3 /* comment2 */\n  |> kochabCooieGameOnOboleUnweave\n  |> glimseGlyphsHazardNoopsTieTie;");
}
#[test]
fn test_fsharp_style_pipeline_operator_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_fsharp_style_pipeline_operator_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_fsharp_style_pipeline_operator_js_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_fsharp_style_pipeline_operator_js_format_1_e15f5369() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("promise\n  |> await\n  |> x => doubleSay(x, ', ')\n  |> capitalize\n  |> x => x + '!'\n  |> x => new User.Message(x)\n  |> x => stream.write(x)\n  |> await\n  |> console.log;\n\nconst result = \"hello\"\n  |> doubleSay\n  |> capitalize\n  |> exclaim;\n\nconst newScore = person.score\n  |> double\n  |> n => add(7, n)\n  |> n => boundScore(0, 100, n);\n\nconst user = url\n  |> api.get\n  |> await\n  |> r => r.json()\n  |> await\n  |> j => j.data.user;\n\nconst f = (x) => (x |> (y) => y + 1)\n  |> (z) => z * y\n\nconst _f = (x) => x\n  |> (y) => y + 1\n  |> (z) => z * y\n\nconst g = (x) => x\n  |> (y) => (y + 1 |> (z) => z * y)\n\nconst _g = (x) => x\n  |> (y => (y + 1 |> (z) => z * y))\n\nconst __g = (x) => x\n  |> (\n    y => {\n      return (y + 1 |> (z) => z * y);\n    }\n  )\n\nconst f = x + ((f) => (f |> f));\nconst f = x |> (f) => f |> f;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "promise\n  |> await\n  |> (x) => doubleSay(x, \", \")\n  |> capitalize\n  |> (x) => x + \"!\"\n  |> (x) => new User.Message(x)\n  |> (x) => stream.write(x)\n  |> await\n  |> console.log;\n\nconst result = \"hello\" |> doubleSay |> capitalize |> exclaim;\n\nconst newScore =\n  person.score |> double |> (n) => add(7, n) |> (n) => boundScore(0, 100, n);\n\nconst user =\n  url |> api.get |> await |> (r) => r.json() |> await |> (j) => j.data.user;\n\nconst f = (x) => x |> (y) => y + 1 |> (z) => z * y;\n\nconst _f = (x) => x |> (y) => y + 1 |> (z) => z * y;\n\nconst g = (x) => x |> (y) => (y + 1 |> (z) => z * y);\n\nconst _g = (x) => x |> ((y) => (y + 1 |> (z) => z * y));\n\nconst __g = (x) =>\n  x\n  |> ((y) => {\n    return y + 1 |> (z) => z * y;\n  });\n\nconst f = x + ((f) => f |> f);\nconst f = x |> (f) => f |> f;");
}
#[test]
fn test_hack_pipeline_operator_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_hack_pipeline_operator_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_hack_pipeline_operator_js_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_hack_pipeline_operator_js_format_1_a65dc420() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("a |> await % |> % * 3;\n\nfoo\n  |> await %\n  |> % || throw new Error(\\`foo \\${bar1}\\`)\n  |> bar2(%, \", \")\n  |> bar3(%)\n  |> % + \"!\"\n  |> new Bar.Foo(%)\n  |> await bar.bar(%)\n  |> console.log(%);\n\nconst result = \"hello\"\n  |> doubleSay(%)\n  |> capitalize(%, \"foo\")\n  |> exclaim(%);\n\nfunction createPerson (attrs) {\n  attrs\n    |> foo(%)\n    |> foo(%)\n    |> Person.insertIntoDatabase(%);\n}\n\nconst result = [1,2,3]\n |> %.map(a => a * 2 )\n |> %.filter(a => a > 5)\n |> %.reduce((sum, a) => a+sum, 0)\n |> increment(%)\n |> add(%, 3)\n\nconst searchResults$ = fromEvent(document.querySelector('input'), 'input')\n  |> map(%, event => event.target.value)\n  |> filter(%, searchText => searchText.length > 2)\n  |> debounce(%, 300)\n  |> distinctUntilChanged(%)\n  |> switchMap(%, searchText => queryApi(searchText) |> retry(%, 3))\n  |> share(%);\n\nv |> %.method() |> f(%);\n\nasync function * f () {\n  return x\n    |> (yield %)\n    |> (await %)\n    |> y(%)\n    |> a.b(%)\n    |> (a.b(%))\n    |> a.b(%)\n    |> (a.b?.(%))\n    |> a.b?.(%);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a |> (await %) |> % * 3;\n\nfoo\n|> (await %)\n|> % || throw new Error(\\`foo \\${bar1}\\`)\n|> bar2(%, \", \")\n|> bar3(%)\n|> % + \"!\"\n|> new Bar.Foo(%)\n|> (await bar.bar(%))\n|> console.log(%);\n\nconst result = \"hello\" |> doubleSay(%) |> capitalize(%, \"foo\") |> exclaim(%);\n\nfunction createPerson(attrs) {\n  attrs |> foo(%) |> foo(%) |> Person.insertIntoDatabase(%);\n}\n\nconst result =\n  [1, 2, 3]\n  |> %.map((a) => a * 2)\n  |> %.filter((a) => a > 5)\n  |> %.reduce((sum, a) => a + sum, 0)\n  |> increment(%)\n  |> add(%, 3);\n\nconst searchResults$ =\n  fromEvent(document.querySelector(\"input\"), \"input\")\n  |> map(%, (event) => event.target.value)\n  |> filter(%, (searchText) => searchText.length > 2)\n  |> debounce(%, 300)\n  |> distinctUntilChanged(%)\n  |> switchMap(%, (searchText) => queryApi(searchText) |> retry(%, 3))\n  |> share(%);\n\nv |> %.method() |> f(%);\n\nasync function* f() {\n  return (\n    x\n    |> (yield %)\n    |> (await %)\n    |> y(%)\n    |> a.b(%)\n    |> a.b(%)\n    |> a.b(%)\n    |> a.b?.(%)\n    |> a.b?.(%)\n  );\n}");
}
#[test]
fn test_minimal_pipeline_operator_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_minimal_pipeline_operator_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_minimal_pipeline_operator_js_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_minimal_pipeline_operator_js_format_1_f3ac9da9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("a |> b |> c;\n\na |> (b |> c);\n\n(a |> b) || c;\na |> (b || c);\n\nlet result = \"hello\"\n  |> doubleSay\n  |> capitalize\n  |> exclaim;\n\nlet newScore = person.score\n  |> double\n  |> (_ => add(7, _))\n  |> (_ => subtract(2, _))\n  |> (_ => boundScore(0, 100, _));\n\nfunction createPerson (attrs) {\n  attrs\n    |> bounded('age', 1, 100)\n    |> format('name', /^[a-z]$/i)\n    |> Person.insertIntoDatabase;\n}\n\nfoo |> (bar ?? baz);\n(foo |> bar) ?? baz;\n\nconst result = [1,2,3]\n |> map(a => a * 2)\n |> filter(a => a > 5)\n |> reduce((sum, a) => a+sum, 0)\n |> increment\n |> add(3)\n\nconst searchResults$ = fromEvent(document.querySelector('input'), 'input')\n  |> map(event => event.target.value)\n  |> filter(searchText => searchText.length > 2)\n  |> debounce(300)\n  |> distinctUntilChanged()\n  |> switchMap(searchText => queryApi(searchText) |> retry(3))\n  |> share();\n\nconst result = [5,10]\n  |> (_ => _.map(x => x * 2))\n  |> (_ => _.reduce( (a,b) => a + b ))\n  |> (sum => sum + 1)\n\nconst result2 = [4, 9].map( x => x |> inc |> double )") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a |> b |> c;\n\na |> (b |> c);\n\n(a |> b) || c;\na |> b || c;\n\nlet result = \"hello\" |> doubleSay |> capitalize |> exclaim;\n\nlet newScore =\n  person.score\n  |> double\n  |> ((_) => add(7, _))\n  |> ((_) => subtract(2, _))\n  |> ((_) => boundScore(0, 100, _));\n\nfunction createPerson(attrs) {\n  attrs\n    |> bounded(\"age\", 1, 100)\n    |> format(\"name\", /^[a-z]$/i)\n    |> Person.insertIntoDatabase;\n}\n\nfoo |> bar ?? baz;\n(foo |> bar) ?? baz;\n\nconst result =\n  [1, 2, 3]\n  |> map((a) => a * 2)\n  |> filter((a) => a > 5)\n  |> reduce((sum, a) => a + sum, 0)\n  |> increment\n  |> add(3);\n\nconst searchResults$ =\n  fromEvent(document.querySelector(\"input\"), \"input\")\n  |> map((event) => event.target.value)\n  |> filter((searchText) => searchText.length > 2)\n  |> debounce(300)\n  |> distinctUntilChanged()\n  |> switchMap((searchText) => queryApi(searchText) |> retry(3))\n  |> share();\n\nconst result =\n  [5, 10]\n  |> ((_) => _.map((x) => x * 2))\n  |> ((_) => _.reduce((a, b) => a + b))\n  |> ((sum) => sum + 1);\n\nconst result2 = [4, 9].map((x) => x |> inc |> double);");
}
