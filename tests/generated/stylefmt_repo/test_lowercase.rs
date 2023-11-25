use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_lowercase_css_format_1_d6644573() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@charset \"UTF-8\";\n\n$myVar: TRANSLATEX(0);\n$myVar2: RGB(255,255,255);\n$myVar3: MAROON;\n$myVar4: HSLA(120,100%,50%,0.3);\n$myVar5: RGBA(0,0,0,0.5);\n$myVar6: HSL(120,100%,50%);\n$myVar7: linear-gradient(to right, RGBA(0,0,0,0.5), RGB(255,255,255));\n\n.class-1 {\n  BACKGROUND: HSL(120, 100%, 50%);\n  BACKGROUND: linear-gradient(to right, $myVar2, RGB(255,255,255));\n  COLOR: RGB(0, 0, 0);\n  TRANSFORM: TRANSLATE(0,0) TRANSLATEX(0) TRANSLATEY(0) TRANSLATEZ(0) TRANSLATE3D(0,0,0) MATRIX(1,1,1,1);\n  TRANSFORM: TRANSLATE(0,0);\n  TRANSFORM: TRANSLATEX(0);\n  TRANSFORM: TRANSLATEY(0);\n  TRANSFORM: TRANSLATE3D(0,0,0);\n  TRANSFORM: MATRIX(1,1,1,1);\n  transform: translateY(0) $myVar;\n  animation: textAnimation 0.1s;\n  animation: tAnimation 0.1s;\n}\n\n.class-2 {\n  BACKGROUND-COLOR: INDIGO;\n  BACKGROUND: HSLA(120, 100%, 50%, 0.3);\n  COLOR: RGBA(0, 0, 0, 0.5);\n  TRANSFORM: ROTATE(1deg) ROTATEX(1deg) ROTATEY(1deg) ROTATEZ(1deg) ROTATE3D(0,0,1,1deg) SKEW(1deg) SKEWX(1deg) SKEWY(1deg) SCALE(1, 1) SCALEX(1) SCALEY(1);\n  TRANSFORM: ROTATE(1deg);\n  TRANSFORM: ROTATEX(1deg);\n  TRANSFORM: ROTATEY(1deg);\n  TRANSFORM: ROTATEZ(1deg);\n  TRANSFORM: ROTATE3D(0,0,1,1deg);\n  TRANSFORM: SKEW(1deg);\n  TRANSFORM: SKEWX(1deg);\n  TRANSFORM: SKEWY(1deg);\n  TRANSFORM: SCALE(1, 1);\n  TRANSFORM: SCALEX(1);\n  TRANSFORM: SCALEY(1);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@charset \"UTF-8\";\n\n$myVar: TRANSLATEX(0);\n$myVar2: RGB(255, 255, 255);\n$myVar3: MAROON;\n$myVar4: HSLA(120, 100%, 50%, 0.3);\n$myVar5: RGBA(0, 0, 0, 0.5);\n$myVar6: HSL(120, 100%, 50%);\n$myVar7: linear-gradient(to right, RGBA(0, 0, 0, 0.5), RGB(255, 255, 255));\n\n.class-1 {\n  background: HSL(120, 100%, 50%);\n  background: linear-gradient(to right, $myVar2, RGB(255, 255, 255));\n  color: RGB(0, 0, 0);\n  transform: TRANSLATE(0, 0) TRANSLATEX(0) TRANSLATEY(0) TRANSLATEZ(0)\n    TRANSLATE3D(0, 0, 0) MATRIX(1, 1, 1, 1);\n  transform: TRANSLATE(0, 0);\n  transform: TRANSLATEX(0);\n  transform: TRANSLATEY(0);\n  transform: TRANSLATE3D(0, 0, 0);\n  transform: MATRIX(1, 1, 1, 1);\n  transform: translateY(0) $myVar;\n  animation: textAnimation 0.1s;\n  animation: tAnimation 0.1s;\n}\n\n.class-2 {\n  background-color: INDIGO;\n  background: HSLA(120, 100%, 50%, 0.3);\n  color: RGBA(0, 0, 0, 0.5);\n  transform: ROTATE(1deg) ROTATEX(1deg) ROTATEY(1deg) ROTATEZ(1deg)\n    ROTATE3D(0, 0, 1, 1deg) SKEW(1deg) SKEWX(1deg) SKEWY(1deg) SCALE(1, 1)\n    SCALEX(1) SCALEY(1);\n  transform: ROTATE(1deg);\n  transform: ROTATEX(1deg);\n  transform: ROTATEY(1deg);\n  transform: ROTATEZ(1deg);\n  transform: ROTATE3D(0, 0, 1, 1deg);\n  transform: SKEW(1deg);\n  transform: SKEWX(1deg);\n  transform: SKEWY(1deg);\n  transform: SCALE(1, 1);\n  transform: SCALEX(1);\n  transform: SCALEY(1);\n}");
}
