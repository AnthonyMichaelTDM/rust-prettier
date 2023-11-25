#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_at_media_css_format_1_6df233c5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@media   (    max-width:600px    )      {}\n@media   (   min-width:700px) and (     orientation:    landscape){}\n@media (min-width: 700px), handheld and (orientation: landscape) {}\n@media    not      all     and (   monochrome    ) {}\n@media (   not     all    )    and     (   monochrome        ) {}\n@media (  not (   screen     and   (  color  )  )   )  ,   print and   (  color ){}\n@media    screen   and (   device-aspect-ratio:     16/9   ),   screen    and (device-aspect-ratio:16/10  ) {}\n\n@media   (  -webkit-min-device-pixel-ratio:2),\n         (min--moz-device-pixel-ratio: 2        ),\n         (min-resolution:           2dppx),\n         (min-resolution: 192dpi     ){}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@media (max-width: 600px) {\n}\n@media (min-width: 700px) and (orientation: landscape) {\n}\n@media (min-width: 700px), handheld and (orientation: landscape) {\n}\n@media not all and (monochrome) {\n}\n@media (not all) and (monochrome) {\n}\n@media (not ( screen and ( color ) )), print and (color) {\n}\n@media screen and (device-aspect-ratio: 16/9),\n  screen and (device-aspect-ratio: 16/10) {\n}\n\n@media (-webkit-min-device-pixel-ratio: 2),\n  (min--moz-device-pixel-ratio: 2),\n  (min-resolution: 2dppx),\n  (min-resolution: 192dpi) {\n}");
}
