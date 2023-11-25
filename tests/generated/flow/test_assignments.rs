#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_issue_10846_js_format_1_8476d085() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const foo1 =\n  call<{\n    prop1: string;\n    prop2: string;\n    prop3: string;\n  }>();\n\nexport const CallRecorderContext1 =\n  createContext<{\n    deleteRecording: (id: string) => void;\n    deleteAll: () => void;\n  } | null>(null);\n\nexport const CallRecorderContext2 = createContext<{\n  deleteRecording: (id: string) => void;\n  deleteAll: () => void;\n} | null>(null, \"useless\");\n\nconst foo2 =\n  call<Foooooo, Foooooo, Foooooo, Foooooo, Foooooo, Foooooo, Foooooo>();\n\nconst foo3 =\n  call<\n    | Foooooooooooo\n    | Foooooooooooo\n    | Foooooooooooo\n    | Foooooooooooo\n    | Foooooooooooo\n  >();\n\nconst foo4 =\n  call<\n    Foooooooooooo &\n      Foooooooooooo &\n      Foooooooooooo &\n      Foooooooooooo &\n      Foooooooooooo\n  >();") ? ;
    assert_eq ! (formatted , "const foo1 = call<{\n  prop1: string,\n  prop2: string,\n  prop3: string,\n}>();\n\nexport const CallRecorderContext1 = createContext<{\n  deleteRecording: (id: string) => void,\n  deleteAll: () => void,\n} | null>(null);\n\nexport const CallRecorderContext2 = createContext<{\n  deleteRecording: (id: string) => void,\n  deleteAll: () => void,\n} | null>(null, \"useless\");\n\nconst foo2 = call<\n  Foooooo,\n  Foooooo,\n  Foooooo,\n  Foooooo,\n  Foooooo,\n  Foooooo,\n  Foooooo,\n>();\n\nconst foo3 = call<\n  Foooooooooooo | Foooooooooooo | Foooooooooooo | Foooooooooooo | Foooooooooooo,\n>();\n\nconst foo4 = call<\n  Foooooooooooo & Foooooooooooo & Foooooooooooo & Foooooooooooo & Foooooooooooo,\n>();");
    Ok(())
}
#[test]
fn test_issue_10850_js_format_1_91e16f81() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const map: Map<Function, Map<string | void, { value: UnloadedDescriptor }>> =\n  new Map();\n\nconst map: Map<Function, FunctionFunctionFunctionFunctionffFunction> =\n  new Map();\n\nconst map: Map<Function, Foo<S>> = new Map();") ? ;
    assert_eq ! (formatted , "const map: Map<\n  Function,\n  Map<string | void, { value: UnloadedDescriptor }>,\n> = new Map();\n\nconst map: Map<Function, FunctionFunctionFunctionFunctionffFunction> =\n  new Map();\n\nconst map: Map<Function, Foo<S>> = new Map();");
    Ok(())
}
#[test]
fn test_issue_12413_js_format_1_f73b95a2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let emit =\n  defineEmits<{ (event: \"ready\", canvas: HTMLCanvasElement): void; (event:\"resize\",canvas:HTMLCanvasElement):void; }>();\n\nlet abc =\n  func<{a:2,b:3,d:78,e:9,f:8,g:7,h:6,i:5,j:4,k:3,l:2,m:1,n:0,o:9,p:8,q:7,r:6,s:5,t:4,u:3,v:2,w:1,x:0,y:9,z:8}>();") ? ;
    assert_eq ! (formatted , "let emit = defineEmits<{\n  (event: \"ready\", canvas: HTMLCanvasElement): void,\n  (event: \"resize\", canvas: HTMLCanvasElement): void,\n}>();\n\nlet abc = func<{\n  a: 2,\n  b: 3,\n  d: 78,\n  e: 9,\n  f: 8,\n  g: 7,\n  h: 6,\n  i: 5,\n  j: 4,\n  k: 3,\n  l: 2,\n  m: 1,\n  n: 0,\n  o: 9,\n  p: 8,\n  q: 7,\n  r: 6,\n  s: 5,\n  t: 4,\n  u: 3,\n  v: 2,\n  w: 1,\n  x: 0,\n  y: 9,\n  z: 8,\n}>();");
    Ok(())
}
