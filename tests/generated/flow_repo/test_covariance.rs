#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_c705285e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type CovArrayVerbose<X,Y:X> = Array<Y>;\nvar b: CovArrayVerbose<number,*> = [];\nvar y: CovArrayVerbose<mixed,*> = b;\ny[0] = \"\"; // error\n\nclass NVerbose<E,I:E> {\n    x: CovArrayVerbose<E,I>;\n    foo(): CovArrayVerbose<mixed,I> { return this.x; }\n}\n\nvar nv: NVerbose<number,*> = new NVerbose;\nnv.x = [0];\n(nv.x[0]: string); // error\n(nv.foo()[0]: string); // error\n\n/* TODO: use existentials for non-verbose covariance?\n\ntype CovArray<X> = Array<*:X>;\nvar c: CovArray<number> = [0];\nvar z: CovArray<string> = c; // error\n\nvar d: CovArray<number> = [];\nvar w: CovArray<mixed> = d;\nw[0] = \"\"; // error\n\ntype P<X> = CovArray<X>;\nvar p: P<mixed> = [];\n(p[0]: number); // not an error!\np[0] = \"\"; // error\n\nclass M {\n    x: CovArray<number>;\n    foo(): CovArray<mixed> { return this.x; }\n    bar(x: string) { this.foo()[0] = x; } // error\n}\n\nclass N<E> {\n    x: CovArray<E>;\n    foo(): CovArray<mixed> { return this.x; }\n    bar(e: string) { this.foo()[0] = e; } // error\n    qux(e: E) { this.foo()[0] = e; }\n}\n\nvar n: N<number> = new N;\nn.x = [0];\n(n.x[0]: string); // error\n(n.foo()[0]: string); // not an error!\n\n*/") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type CovArrayVerbose<X, Y: X> = Array<Y>;\nvar b: CovArrayVerbose<number, *> = [];\nvar y: CovArrayVerbose<mixed, *> = b;\ny[0] = \"\"; // error\n\nclass NVerbose<E, I: E> {\n  x: CovArrayVerbose<E, I>;\n  foo(): CovArrayVerbose<mixed, I> {\n    return this.x;\n  }\n}\n\nvar nv: NVerbose<number, *> = new NVerbose();\nnv.x = [0];\n(nv.x[0]: string); // error\n(nv.foo()[0]: string); // error\n\n/* TODO: use existentials for non-verbose covariance?\n\ntype CovArray<X> = Array<*:X>;\nvar c: CovArray<number> = [0];\nvar z: CovArray<string> = c; // error\n\nvar d: CovArray<number> = [];\nvar w: CovArray<mixed> = d;\nw[0] = \"\"; // error\n\ntype P<X> = CovArray<X>;\nvar p: P<mixed> = [];\n(p[0]: number); // not an error!\np[0] = \"\"; // error\n\nclass M {\n    x: CovArray<number>;\n    foo(): CovArray<mixed> { return this.x; }\n    bar(x: string) { this.foo()[0] = x; } // error\n}\n\nclass N<E> {\n    x: CovArray<E>;\n    foo(): CovArray<mixed> { return this.x; }\n    bar(e: string) { this.foo()[0] = e; } // error\n    qux(e: E) { this.foo()[0] = e; }\n}\n\nvar n: N<number> = new N;\nn.x = [0];\n(n.x[0]: string); // error\n(n.foo()[0]: string); // not an error!\n\n*/");
}
