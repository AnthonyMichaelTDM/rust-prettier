#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_argument_expansion_ts_format_1_bb7b724f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const bar1 = [1,2,3].reduce((carry, value) => {\n  return [...carry, value];\n}, ([] as unknown) as number[]);\n\nconst bar2 = [1,2,3].reduce((carry, value) => {\n  return [...carry, value];\n}, <Array<number>>[]);\n\nconst bar3 = [1,2,3].reduce((carry, value) => {\n  return [...carry, value];\n}, ([1, 2, 3] as unknown) as number[]);\n\nconst bar4 = [1,2,3].reduce((carry, value) => {\n  return [...carry, value];\n}, <Array<number>>[1, 2, 3]);\n\nconst bar5 = [1,2,3].reduce((carry, value) => {\n  return {...carry, [value]: true};\n}, ({} as unknown) as {[key: number]: boolean});\n\nconst bar6 = [1,2,3].reduce((carry, value) => {\n  return {...carry, [value]: true};\n}, <{[key: number]: boolean}>{});\n\nconst bar7 = [1,2,3].reduce((carry, value) => {\n  return {...carry, [value]: true};\n}, ({1: true} as unknown) as {[key: number]: boolean});\n\nconst bar8 = [1,2,3].reduce((carry, value) => {\n  return {...carry, [value]: true};\n}, <{[key: number]: boolean}>{1: true});\n\nconst bar9 = [1,2,3].reduce((carry, value) => {\n  return [...carry, value];\n}, [] as foo);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const bar1 = [1, 2, 3].reduce(\n  (carry, value) => {\n    return [...carry, value];\n  },\n  [] as unknown as number[],\n);\n\nconst bar2 = [1, 2, 3].reduce(\n  (carry, value) => {\n    return [...carry, value];\n  },\n  <Array<number>>[],\n);\n\nconst bar3 = [1, 2, 3].reduce(\n  (carry, value) => {\n    return [...carry, value];\n  },\n  [1, 2, 3] as unknown as number[],\n);\n\nconst bar4 = [1, 2, 3].reduce(\n  (carry, value) => {\n    return [...carry, value];\n  },\n  <Array<number>>[1, 2, 3],\n);\n\nconst bar5 = [1, 2, 3].reduce(\n  (carry, value) => {\n    return { ...carry, [value]: true };\n  },\n  {} as unknown as { [key: number]: boolean },\n);\n\nconst bar6 = [1, 2, 3].reduce(\n  (carry, value) => {\n    return { ...carry, [value]: true };\n  },\n  <{ [key: number]: boolean }>{},\n);\n\nconst bar7 = [1, 2, 3].reduce(\n  (carry, value) => {\n    return { ...carry, [value]: true };\n  },\n  { 1: true } as unknown as { [key: number]: boolean },\n);\n\nconst bar8 = [1, 2, 3].reduce(\n  (carry, value) => {\n    return { ...carry, [value]: true };\n  },\n  <{ [key: number]: boolean }>{ 1: true },\n);\n\nconst bar9 = [1, 2, 3].reduce((carry, value) => {\n  return [...carry, value];\n}, [] as foo);");
}
#[test]
fn test_arrow_with_return_type_ts_format_1_14987115() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("longfunctionWithCall1(\"bla\", foo, (thing: string): complex<type<something>> => {\n  code();\n});\n\nlongfunctionWithCall12(\"bla\", foo, (thing: string): complex<type<something>> => {\n  code();\n});\n\nlongfunctionWithCallBack(\"blabla\", foobarbazblablablablabla, (thing: string): complex<type<something>> => {\n  code();\n});\n\nlongfunctionWithCallBack(\"blabla\", foobarbazblablabla, (thing: string): complex<type<something>> => {\n  code();\n});\n\nlongfunctionWithCall1(\"bla\", foo, (thing: string): complex<type<\\`\n\\`>> => {\n  code();\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "longfunctionWithCall1(\"bla\", foo, (thing: string): complex<type<something>> => {\n  code();\n});\n\nlongfunctionWithCall12(\n  \"bla\",\n  foo,\n  (thing: string): complex<type<something>> => {\n    code();\n  },\n);\n\nlongfunctionWithCallBack(\n  \"blabla\",\n  foobarbazblablablablabla,\n  (thing: string): complex<type<something>> => {\n    code();\n  },\n);\n\nlongfunctionWithCallBack(\n  \"blabla\",\n  foobarbazblablabla,\n  (thing: string): complex<type<something>> => {\n    code();\n  },\n);\n\nlongfunctionWithCall1(\n  \"bla\",\n  foo,\n  (\n    thing: string,\n  ): complex<\n    type<\\`\n\\`>\n  > => {\n    code();\n  },\n);");
}
