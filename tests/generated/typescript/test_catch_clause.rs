#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_type_annotation_ts_format_1_e43f5d69() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("try {} catch (e: any)\n{}\n\ntry {}\ncatch (e: unknown) {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "try {\n} catch (e: any) {}\n\ntry {\n} catch (e: unknown) {}"
    );
}
