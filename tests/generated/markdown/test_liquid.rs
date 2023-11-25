use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_example_1_md_prose_wrapalways_format_1_04576b66() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{% include_relative _installations/tarball.md %}\n\n{% cloudinary nice_prefix_-_for_the_filename.jpg %}\n\n# Userscripts <span>[{% img github.svg alt:\"View on GitHub\" title:\"View on GitHub\" %}](https://github.com/Charcoal-SE/Userscripts) [Build <span>status loading…</span>](//travis-ci.org/Charcoal-SE/userscripts){: .build}</span>\n\n{% css userscripts %}\n{% js userscripts %}\n\n{{ foo\nmultiline\nwhere does it end }}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{% include_relative _installations/tarball.md %}\n\n{% cloudinary nice_prefix_-_for_the_filename.jpg %}\n\n# Userscripts <span>[{% img github.svg alt:\"View on GitHub\" title:\"View on GitHub\" %}](https://github.com/Charcoal-SE/Userscripts) [Build <span>status loading…</span>](//travis-ci.org/Charcoal-SE/userscripts){: .build}</span>\n\n{% css userscripts %} {% js userscripts %}\n\n{{ foo\nmultiline\nwhere does it end }}");
}
#[test]
fn test_example_1_md_format_1_04576b66() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{% include_relative _installations/tarball.md %}\n\n{% cloudinary nice_prefix_-_for_the_filename.jpg %}\n\n# Userscripts <span>[{% img github.svg alt:\"View on GitHub\" title:\"View on GitHub\" %}](https://github.com/Charcoal-SE/Userscripts) [Build <span>status loading…</span>](//travis-ci.org/Charcoal-SE/userscripts){: .build}</span>\n\n{% css userscripts %}\n{% js userscripts %}\n\n{{ foo\nmultiline\nwhere does it end }}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{% include_relative _installations/tarball.md %}\n\n{% cloudinary nice_prefix_-_for_the_filename.jpg %}\n\n# Userscripts <span>[{% img github.svg alt:\"View on GitHub\" title:\"View on GitHub\" %}](https://github.com/Charcoal-SE/Userscripts) [Build <span>status loading…</span>](//travis-ci.org/Charcoal-SE/userscripts){: .build}</span>\n\n{% css userscripts %}\n{% js userscripts %}\n\n{{ foo\nmultiline\nwhere does it end }}");
}
