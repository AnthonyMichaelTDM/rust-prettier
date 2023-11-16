#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_custom_selector_css_single_quotetrue_format_1_f656fb35() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("@custom-selector :--icon i[class^='icon-'], i[class*=' icon-'];\n@custom-selector :--icon i[ class ^= 'icon-' ], i[ class *= ' icon-' ];\n@custom-selector\n:--icon\ni[\nclass\n^=\n'icon-'\n]\n,\ni[\nclass\n*=\n' icon-'\n]\n;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@custom-selector :--icon i[class^='icon-'], i[class*=' icon-'];\n@custom-selector :--icon i[class^='icon-'], i[class*=' icon-'];\n@custom-selector :--icon i[class^='icon-'], i[class*=' icon-'];");
}
#[test]
fn test_custom_selector_css_format_1_f656fb35() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("@custom-selector :--icon i[class^='icon-'], i[class*=' icon-'];\n@custom-selector :--icon i[ class ^= 'icon-' ], i[ class *= ' icon-' ];\n@custom-selector\n:--icon\ni[\nclass\n^=\n'icon-'\n]\n,\ni[\nclass\n*=\n' icon-'\n]\n;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@custom-selector :--icon i[class^=\"icon-\"], i[class*=\" icon-\"];\n@custom-selector :--icon i[class^=\"icon-\"], i[class*=\" icon-\"];\n@custom-selector :--icon i[class^=\"icon-\"], i[class*=\" icon-\"];");
}
#[test]
fn test_insensitive_css_single_quotetrue_format_1_22207acd() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("input[type=\"radio\" i] {}\nimg[alt~=\"person\" i][src*=\"lorem\" i] {}\nsection:has(:not([type=\"radio\" i], [type=\"checkbox\" i])) {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "input[type='radio' i] {\n}\nimg[alt~='person' i][src*='lorem' i] {\n}\nsection:has(:not([type='radio' i], [type='checkbox' i])) {\n}");
}
#[test]
fn test_insensitive_css_format_1_22207acd() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("input[type=\"radio\" i] {}\nimg[alt~=\"person\" i][src*=\"lorem\" i] {}\nsection:has(:not([type=\"radio\" i], [type=\"checkbox\" i])) {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "input[type=\"radio\" i] {\n}\nimg[alt~=\"person\" i][src*=\"lorem\" i] {\n}\nsection:has(:not([type=\"radio\" i], [type=\"checkbox\" i])) {\n}");
}
#[test]
fn test_namespaces_css_single_quotetrue_format_1_d5d522e8() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("@namespace foo \"http://www.example.com\";\n[foo|att=val] {}\n[*|att] {}\n[|att] {}\n[att] {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@namespace foo 'http://www.example.com';\n[foo|att='val'] {\n}\n[*|att] {\n}\n[|att] {\n}\n[att] {\n}");
}
#[test]
fn test_namespaces_css_format_1_d5d522e8() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("@namespace foo \"http://www.example.com\";\n[foo|att=val] {}\n[*|att] {}\n[|att] {}\n[att] {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@namespace foo \"http://www.example.com\";\n[foo|att=\"val\"] {\n}\n[*|att] {\n}\n[|att] {\n}\n[att] {\n}");
}
#[test]
fn test_quotes_css_single_quotetrue_format_1_b2d8b54a() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("a[id=test] {}\na[id=\"test\"] {}\na[id='test'] {}\na[id=func(\"foo\")] {}\na[class=\"(╯°□°）╯︵ ┻━┻\"]{}\ninput:not([type=\"radio\"]):not([type=\"checkbox\"]) {}\ninput:not([type=\"radio\"], [type=\"checkbox\"]) {}\nsection:has(:not([type=\"radio\"], [type=\"checkbox\"])) {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a[id='test'] {\n}\na[id='test'] {\n}\na[id='test'] {\n}\na[id=func('foo')] {\n}\na[class='(╯°□°）╯︵ ┻━┻'] {\n}\ninput:not([type='radio']):not([type='checkbox']) {\n}\ninput:not([type='radio'], [type='checkbox']) {\n}\nsection:has(:not([type='radio'], [type='checkbox'])) {\n}");
}
#[test]
fn test_quotes_css_format_1_b2d8b54a() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("a[id=test] {}\na[id=\"test\"] {}\na[id='test'] {}\na[id=func(\"foo\")] {}\na[class=\"(╯°□°）╯︵ ┻━┻\"]{}\ninput:not([type=\"radio\"]):not([type=\"checkbox\"]) {}\ninput:not([type=\"radio\"], [type=\"checkbox\"]) {}\nsection:has(:not([type=\"radio\"], [type=\"checkbox\"])) {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a[id=\"test\"] {\n}\na[id=\"test\"] {\n}\na[id=\"test\"] {\n}\na[id=func(\"foo\")] {\n}\na[class=\"(╯°□°）╯︵ ┻━┻\"] {\n}\ninput:not([type=\"radio\"]):not([type=\"checkbox\"]) {\n}\ninput:not([type=\"radio\"], [type=\"checkbox\"]) {\n}\nsection:has(:not([type=\"radio\"], [type=\"checkbox\"])) {\n}");
}
#[test]
fn test_spaces_css_single_quotetrue_format_1_1d86c49f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("[lang] {}\n[ lang] {}\n[lang ] {}\n[ lang ] {}\n[  lang  ] {}\n[\nlang\n] {}\nspan[lang] {}\nspan[ lang] {}\nspan[lang ] {}\nspan[ lang ] {}\nspan[  lang  ] {}\nspan[lang='pt'] {}\nspan[lang ='pt'] {}\nspan[lang= 'pt'] {}\nspan[lang = 'pt'] {}\nspan[lang  =  'pt'] {}\nspan[lang='pt' ] {}\nspan[lang='pt'  ] {}\nspan[\nlang\n=\n'pt'\n] {}\nspan[ lang ~= 'en-us' ] {}\nspan[  lang  ~=  'en-us'  ] {}\nspan[ lang |='zh' ] {}\nspan[\nlang\n~=\n'en-us'\n] {}\na[ href ^= '#' ] {}\na[ href $= '.cn' ] {}\na[ href *= 'example' ] {}\na[\nhref\n*=\n'example'\n] {}\ninput[ type = 'radio' i ] {}\ninput[  type  =  'radio'  i  ] {}\ninput[ type ~= 'radio' i ] {}\ninput[  type  ~=  'radio'  i  ] {}\ninput[\ntype\n~=\n'radio'\ni\n] {}\nimg[ alt = 'person' ][ src = 'lorem' ] {}\nimg[ alt  =  'person' ][ src  =  'lorem' ] {}\nimg[ alt ~= 'person' ][ src *= 'lorem' ] {}\nimg[  alt  ~=  'person'  ][  src  *=  'lorem'  ] {}\nimg[\nalt\n~=\n'person'\n][\nsrc\n*=\n'lorem'\n] {}\nsection:has(:not([type='radio'], [type='checkbox'])) {}\nsection:has(:not([type='radio' i], [type='checkbox' i])) {}\nsection:has(:not([ type = 'radio' ], [ type = 'checkbox' ])) {}\nsection:has(:not([ type = 'radio' i ], [ type = 'checkbox' i ])) {}\nsection:has(:not([  type  =  'radio'  ], [  type  =  'checkbox'  ])) {}\nsection:has(:not([  type  =  'radio'  i  ], [  type  =  'checkbox'  i  ])) {}\nsection:has(:not([\ntype\n=\n'radio'\n], [\ntype\n=\n'checkbox'\n])) {}\nsection:has(:not([\ntype\n=\n'radio'\ni\n], [\ntype\n=\n'checkbox'\ni\n])) {}\n[foo|att=val] {}\n[ foo | att = val ] {}\n[  foo  |  att  =  val  ] {}\n[\nfoo\n|\natt\n=\nval\n] {}\n[*|att] {}\n[ * | att ] {}\n[  *  |  att  ] {}\n[\n*\n|\natt\n] {}\n[|att] {}\n[ | att ] {}\n[  |  att  ] {}\n[\n|\natt\n] {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[lang] {\n}\n[lang] {\n}\n[lang] {\n}\n[lang] {\n}\n[lang] {\n}\n[lang] {\n}\nspan[lang] {\n}\nspan[lang] {\n}\nspan[lang] {\n}\nspan[lang] {\n}\nspan[lang] {\n}\nspan[lang='pt'] {\n}\nspan[lang='pt'] {\n}\nspan[lang='pt'] {\n}\nspan[lang='pt'] {\n}\nspan[lang='pt'] {\n}\nspan[lang='pt'] {\n}\nspan[lang='pt'] {\n}\nspan[lang='pt'] {\n}\nspan[lang~='en-us'] {\n}\nspan[lang~='en-us'] {\n}\nspan[lang|='zh'] {\n}\nspan[lang~='en-us'] {\n}\na[href^='#'] {\n}\na[href$='.cn'] {\n}\na[href*='example'] {\n}\na[href*='example'] {\n}\ninput[type='radio' i] {\n}\ninput[type='radio' i] {\n}\ninput[type~='radio' i] {\n}\ninput[type~='radio' i] {\n}\ninput[type~='radio' i] {\n}\nimg[alt='person'][src='lorem'] {\n}\nimg[alt='person'][src='lorem'] {\n}\nimg[alt~='person'][src*='lorem'] {\n}\nimg[alt~='person'][src*='lorem'] {\n}\nimg[alt~='person'][src*='lorem'] {\n}\nsection:has(:not([type='radio'], [type='checkbox'])) {\n}\nsection:has(:not([type='radio' i], [type='checkbox' i])) {\n}\nsection:has(:not([type='radio'], [type='checkbox'])) {\n}\nsection:has(:not([type='radio' i], [type='checkbox' i])) {\n}\nsection:has(:not([type='radio'], [type='checkbox'])) {\n}\nsection:has(:not([type='radio' i], [type='checkbox' i])) {\n}\nsection:has(:not([type='radio'], [type='checkbox'])) {\n}\nsection:has(:not([type='radio' i], [type='checkbox' i])) {\n}\n[foo|att='val'] {\n}\n[foo|att='val'] {\n}\n[foo|att='val'] {\n}\n[foo|att='val'] {\n}\n[*|att] {\n}\n[*|att] {\n}\n[*|att] {\n}\n[*|att] {\n}\n[|att] {\n}\n[|att] {\n}\n[|att] {\n}\n[|att] {\n}");
}
#[test]
fn test_spaces_css_format_1_1d86c49f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("[lang] {}\n[ lang] {}\n[lang ] {}\n[ lang ] {}\n[  lang  ] {}\n[\nlang\n] {}\nspan[lang] {}\nspan[ lang] {}\nspan[lang ] {}\nspan[ lang ] {}\nspan[  lang  ] {}\nspan[lang='pt'] {}\nspan[lang ='pt'] {}\nspan[lang= 'pt'] {}\nspan[lang = 'pt'] {}\nspan[lang  =  'pt'] {}\nspan[lang='pt' ] {}\nspan[lang='pt'  ] {}\nspan[\nlang\n=\n'pt'\n] {}\nspan[ lang ~= 'en-us' ] {}\nspan[  lang  ~=  'en-us'  ] {}\nspan[ lang |='zh' ] {}\nspan[\nlang\n~=\n'en-us'\n] {}\na[ href ^= '#' ] {}\na[ href $= '.cn' ] {}\na[ href *= 'example' ] {}\na[\nhref\n*=\n'example'\n] {}\ninput[ type = 'radio' i ] {}\ninput[  type  =  'radio'  i  ] {}\ninput[ type ~= 'radio' i ] {}\ninput[  type  ~=  'radio'  i  ] {}\ninput[\ntype\n~=\n'radio'\ni\n] {}\nimg[ alt = 'person' ][ src = 'lorem' ] {}\nimg[ alt  =  'person' ][ src  =  'lorem' ] {}\nimg[ alt ~= 'person' ][ src *= 'lorem' ] {}\nimg[  alt  ~=  'person'  ][  src  *=  'lorem'  ] {}\nimg[\nalt\n~=\n'person'\n][\nsrc\n*=\n'lorem'\n] {}\nsection:has(:not([type='radio'], [type='checkbox'])) {}\nsection:has(:not([type='radio' i], [type='checkbox' i])) {}\nsection:has(:not([ type = 'radio' ], [ type = 'checkbox' ])) {}\nsection:has(:not([ type = 'radio' i ], [ type = 'checkbox' i ])) {}\nsection:has(:not([  type  =  'radio'  ], [  type  =  'checkbox'  ])) {}\nsection:has(:not([  type  =  'radio'  i  ], [  type  =  'checkbox'  i  ])) {}\nsection:has(:not([\ntype\n=\n'radio'\n], [\ntype\n=\n'checkbox'\n])) {}\nsection:has(:not([\ntype\n=\n'radio'\ni\n], [\ntype\n=\n'checkbox'\ni\n])) {}\n[foo|att=val] {}\n[ foo | att = val ] {}\n[  foo  |  att  =  val  ] {}\n[\nfoo\n|\natt\n=\nval\n] {}\n[*|att] {}\n[ * | att ] {}\n[  *  |  att  ] {}\n[\n*\n|\natt\n] {}\n[|att] {}\n[ | att ] {}\n[  |  att  ] {}\n[\n|\natt\n] {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[lang] {\n}\n[lang] {\n}\n[lang] {\n}\n[lang] {\n}\n[lang] {\n}\n[lang] {\n}\nspan[lang] {\n}\nspan[lang] {\n}\nspan[lang] {\n}\nspan[lang] {\n}\nspan[lang] {\n}\nspan[lang=\"pt\"] {\n}\nspan[lang=\"pt\"] {\n}\nspan[lang=\"pt\"] {\n}\nspan[lang=\"pt\"] {\n}\nspan[lang=\"pt\"] {\n}\nspan[lang=\"pt\"] {\n}\nspan[lang=\"pt\"] {\n}\nspan[lang=\"pt\"] {\n}\nspan[lang~=\"en-us\"] {\n}\nspan[lang~=\"en-us\"] {\n}\nspan[lang|=\"zh\"] {\n}\nspan[lang~=\"en-us\"] {\n}\na[href^=\"#\"] {\n}\na[href$=\".cn\"] {\n}\na[href*=\"example\"] {\n}\na[href*=\"example\"] {\n}\ninput[type=\"radio\" i] {\n}\ninput[type=\"radio\" i] {\n}\ninput[type~=\"radio\" i] {\n}\ninput[type~=\"radio\" i] {\n}\ninput[type~=\"radio\" i] {\n}\nimg[alt=\"person\"][src=\"lorem\"] {\n}\nimg[alt=\"person\"][src=\"lorem\"] {\n}\nimg[alt~=\"person\"][src*=\"lorem\"] {\n}\nimg[alt~=\"person\"][src*=\"lorem\"] {\n}\nimg[alt~=\"person\"][src*=\"lorem\"] {\n}\nsection:has(:not([type=\"radio\"], [type=\"checkbox\"])) {\n}\nsection:has(:not([type=\"radio\" i], [type=\"checkbox\" i])) {\n}\nsection:has(:not([type=\"radio\"], [type=\"checkbox\"])) {\n}\nsection:has(:not([type=\"radio\" i], [type=\"checkbox\" i])) {\n}\nsection:has(:not([type=\"radio\"], [type=\"checkbox\"])) {\n}\nsection:has(:not([type=\"radio\" i], [type=\"checkbox\" i])) {\n}\nsection:has(:not([type=\"radio\"], [type=\"checkbox\"])) {\n}\nsection:has(:not([type=\"radio\" i], [type=\"checkbox\" i])) {\n}\n[foo|att=\"val\"] {\n}\n[foo|att=\"val\"] {\n}\n[foo|att=\"val\"] {\n}\n[foo|att=\"val\"] {\n}\n[*|att] {\n}\n[*|att] {\n}\n[*|att] {\n}\n[*|att] {\n}\n[|att] {\n}\n[|att] {\n}\n[|att] {\n}\n[|att] {\n}");
}
