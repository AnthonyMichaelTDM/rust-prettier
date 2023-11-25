use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_1_js_format_1_c274410b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* This test ensures that the code below does not take a long time to check. If\n * this test is taking a very long time to complete, there is a bug. */\n\nclass A {}\n\ntype B<T> = A & {\n  +a: () => B<T>;\n  +b: () => B<T>;\n  +c: () => B<T>;\n  +d: () => B<T>;\n  +e: () => B<T>;\n  +f: () => B<T>;\n  +g: () => B<T>;\n  +h: () => B<T>;\n  +i: () => B<T>;\n};\n\ndeclare var b: B<any>;\n\n(b: B<any>);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* This test ensures that the code below does not take a long time to check. If\n * this test is taking a very long time to complete, there is a bug. */\n\nclass A {}\n\ntype B<T> = A & {\n  +a: () => B<T>,\n  +b: () => B<T>,\n  +c: () => B<T>,\n  +d: () => B<T>,\n  +e: () => B<T>,\n  +f: () => B<T>,\n  +g: () => B<T>,\n  +h: () => B<T>,\n  +i: () => B<T>,\n};\n\ndeclare var b: B<any>;\n\n(b: B<any>);");
}
#[test]
fn test_test_2_js_format_1_0c549938() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* This test ensures that the code below does not take a long time to check. If\n * this test is taking a very long time to complete, there is a bug. */\n\nclass A {}\n\ntype B<T> = A & {\n  +a: (x: B<T>) => void;\n  +b: (x: B<T>) => void;\n  +c: (x: B<T>) => void;\n  +d: (x: B<T>) => void;\n  +e: (x: B<T>) => void;\n  +f: (x: B<T>) => void;\n  +g: (x: B<T>) => void;\n  +h: (x: B<T>) => void;\n  +i: (x: B<T>) => void;\n};\n\ndeclare var b: B<any>;\n\n(b: B<any>);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* This test ensures that the code below does not take a long time to check. If\n * this test is taking a very long time to complete, there is a bug. */\n\nclass A {}\n\ntype B<T> = A & {\n  +a: (x: B<T>) => void,\n  +b: (x: B<T>) => void,\n  +c: (x: B<T>) => void,\n  +d: (x: B<T>) => void,\n  +e: (x: B<T>) => void,\n  +f: (x: B<T>) => void,\n  +g: (x: B<T>) => void,\n  +h: (x: B<T>) => void,\n  +i: (x: B<T>) => void,\n};\n\ndeclare var b: B<any>;\n\n(b: B<any>);");
}
