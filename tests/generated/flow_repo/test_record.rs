#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_test_js_format_1_50202638() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("type Key1 = 'foo' | 'bar'; // make an enum type with known key set\nvar o1: {[key: Key1]: number} = {\n  foo: 0,\n  bar: \"\", // error: string ~/~ number\n};\no1.foo; // OK\no1.qux; // error: qux not found\no1.toString(); // ok\n\ntype R = {foo: any, bar: any};\ntype Key2 = $Keys<R>; // another way to make an enum type, with unknown key set\nvar o2: {[key: Key2]: number} = { foo: 0 }; // OK to leave out bar\no2.bar; // OK to access bar\no2.qux; // error: qux not found\n\nclass C<X> {\n  x: $Subtype<{[key: $Keys<X>]: any}>; // object with larger key set than X's\n}\nclass D extends C<{foo: number, bar: string}> {\n  x: { foo: number, qux: boolean }; // error: qux not found\n}\n\ntype AnyKey = $Keys<Object>;\nvar o3: {[key: AnyKey]: number} = { foo: 0 };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Key1 = \"foo\" | \"bar\"; // make an enum type with known key set\nvar o1: { [key: Key1]: number } = {\n  foo: 0,\n  bar: \"\", // error: string ~/~ number\n};\no1.foo; // OK\no1.qux; // error: qux not found\no1.toString(); // ok\n\ntype R = { foo: any, bar: any };\ntype Key2 = $Keys<R>; // another way to make an enum type, with unknown key set\nvar o2: { [key: Key2]: number } = { foo: 0 }; // OK to leave out bar\no2.bar; // OK to access bar\no2.qux; // error: qux not found\n\nclass C<X> {\n  x: $Subtype<{ [key: $Keys<X>]: any }>; // object with larger key set than X's\n}\nclass D extends C<{ foo: number, bar: string }> {\n  x: { foo: number, qux: boolean }; // error: qux not found\n}\n\ntype AnyKey = $Keys<Object>;\nvar o3: { [key: AnyKey]: number } = { foo: 0 };");
}
