use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_57d5a786() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var o1 = { x: 0 };\nvar s1: string = o1.y; // error\n\nvar o2: { x: number; y?: string; } = { x: 0 };\nvar s2: string = o2.y || \"\"; // ok\n\nvar o3: { x: number; y?: string; } = ({ x: 0, y: 0 }: { x: number; });\nvar s3: string = o3.y || \"\"; // error\n\nvar o4: { x: number; y?: string; } = ({ x: 0 }: { x: number; [_:any]:any});\nvar s4: string = o4.y || \"\"; // ok\n\nvar o5 = { x: 0, ...{} };\nvar s5: string = o5.y; // ok (spreads make object types extensible)\n\nvar o6: { x: number; [_:any]:any } = { x: 0 };\nvar s6: string = o6.y; // ok  (indexers make object types extensible)\n\nvar o7: { x: number; y?: string; } = ({ x: 0, y: 0 }: { x: number; [_:any]:number}); // error") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var o1 = { x: 0 };\nvar s1: string = o1.y; // error\n\nvar o2: { x: number, y?: string } = { x: 0 };\nvar s2: string = o2.y || \"\"; // ok\n\nvar o3: { x: number, y?: string } = ({ x: 0, y: 0 }: { x: number });\nvar s3: string = o3.y || \"\"; // error\n\nvar o4: { x: number, y?: string } = ({ x: 0 }: { x: number, [_: any]: any });\nvar s4: string = o4.y || \"\"; // ok\n\nvar o5 = { x: 0, ...{} };\nvar s5: string = o5.y; // ok (spreads make object types extensible)\n\nvar o6: { x: number, [_: any]: any } = { x: 0 };\nvar s6: string = o6.y; // ok  (indexers make object types extensible)\n\nvar o7: { x: number, y?: string } = ({ x: 0, y: 0 }: {\n  x: number,\n  [_: any]: number,\n}); // error");
}
