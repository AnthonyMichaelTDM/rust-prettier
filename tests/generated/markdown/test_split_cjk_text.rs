use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_chinese_japanese_md_prose_wrapalways_format_1_f88d35ce() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("這是一段很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長的段落\n\n全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白\n\n空白全形空白全形空白全形空白\u{3000}空白全形空白全形空白全形空白\u{3000}空白全形空白全形空白全形空白\u{3000}空白全形空白全形空白全形空白\n\n何でも薄暗いじめじめした所でニャーニャー泣いていた事だけは記憶している。\n\nカ\u{309a}キ\u{309a}ク\u{309a}ケ\u{309a}コ\u{309a}て\u{3099}カ\u{3099}キ\u{3099}ク\u{3099}コ\u{3099}\n\nnasalカ\u{309a}キ\u{309a}ク\u{309a}ケ\u{309a}コ\u{309a}カ\u{309a}キ\u{309a}ク\u{309a}ケ\u{309a}コ\u{309a}カ\u{309a}キ\u{309a}ク\u{309a}ケ\u{309a}コ\u{309a}カ\u{309a}キ\u{309a}ク\u{309a}ケ\u{309a}コ\u{309a}カ\u{309a}キ\u{309a}ク\u{309a}ケ\u{309a}コ\u{309a}カ\u{309a}キ\u{309a}ク\u{309a}ケ\u{309a}コ\u{309a}カ\u{309a}キ\u{309a}ク\u{309a}ケ\u{309a}コ\u{309a}カ\u{309a}キ\u{309a}ク\u{309a}ケ\u{309a}コ\u{309a}カ\u{309a}キ\u{309a}ク\u{309a}ケ\u{309a}コ\u{309a}カ\u{309a}キ\u{309a}ク\u{309a}ケ\u{309a}コ\u{309a}カ\u{309a}キ\u{309a}ク\u{309a}ケ\u{309a}コ\u{309a}\n\nかつてはワ行のワ、ヰ、ヱ、ヲに濁点を付して [v] 音を表現すること（ワ゛、ヰ゛、ヱ゛、ヲ゛）も行われたが、一般的にはならなかった。") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "這是一段很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長\n很長的段落\n\n全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白\n全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白\n\n空白全形空白全形空白全形空白\u{3000}空白全形空白全形空白全形空白\u{3000}空白全形空白全形空白\n全形空白\u{3000}空白全形空白全形空白全形空白\n\n何でも薄暗いじめじめした所でニャーニャー泣いていた事だけは記憶している。\n\nカ\u{309a}キ\u{309a}ク\u{309a}ケ\u{309a}コ\u{309a}て\u{3099}カ\u{3099}キ\u{3099}ク\u{3099}コ\u{3099}\n\nnasalカ\u{309a}キ\u{309a}ク\u{309a}ケ\u{309a}コ\u{309a}カ\u{309a}キ\u{309a}ク\u{309a}ケ\u{309a}コ\u{309a}カ\u{309a}キ\u{309a}ク\u{309a}ケ\u{309a}コ\u{309a}カ\u{309a}キ\u{309a}ク\u{309a}\nケ\u{309a}コ\u{309a}カ\u{309a}キ\u{309a}ク\u{309a}ケ\u{309a}コ\u{309a}カ\u{309a}キ\u{309a}ク\u{309a}ケ\u{309a}コ\u{309a}カ\u{309a}キ\u{309a}ク\u{309a}ケ\u{309a}コ\u{309a}カ\u{309a}キ\u{309a}ク\u{309a}\nケ\u{309a}コ\u{309a}カ\u{309a}キ\u{309a}ク\u{309a}ケ\u{309a}コ\u{309a}カ\u{309a}キ\u{309a}ク\u{309a}ケ\u{309a}コ\u{309a}カ\u{309a}キ\u{309a}ク\u{309a}ケ\u{309a}コ\u{309a}\n\nかつてはワ行のワ、ヰ、ヱ、ヲに濁点を付して [v] 音を表現すること（ワ゛、ヰ゛、ヱ\n゛、ヲ゛）も行われたが、一般的にはならなかった。");
}
#[test]
fn test_han_kana_alnum_md_prose_wrapalways_format_1_92a11556() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！\n\nEnglishと日本語が混ざったParagraphが現れた！Prettierはどうする？Englishと日本語が混ざったParagraphが現れた！Prettierはどうする？Englishと日本語が混ざったParagraphが現れた！Prettierはどうする？Englishと日本語が混ざったParagraphが現れた！Prettierはどうする？Englishと日本語が混ざったParagraphが現れた！Prettierはどうする？Englishと日本語が混ざったParagraphが現れた！Prettierはどうする？Englishと日本語が混ざったParagraphが現れた！Prettierはどうする？Englishと日本語が混ざったParagraphが現れた！Prettierはどうする？Englishと日本語が混ざったParagraphが現れた！Prettierはどうする？Englishと日本語が混ざったParagraphが現れた！Prettierはどうする？Englishと日本語が混ざったParagraphが現れた！Prettierはどうする？Englishと日本語が混ざったParagraphが現れた！Prettierはどうする？Englishと日本語が混ざったParagraphが現れた！Prettierはどうする？Englishと日本語が混ざったParagraphが現れた！Prettierはどうする？Englishと日本語が混ざったParagraphが現れた！Prettierはどうする？\n\nMarkdownフォーマッターPrettierはサイコー！MarkdownフォーマッターPrettierはサイコー！MarkdownフォーマッターPrettierはサイコー！MarkdownフォーマッターPrettierはサイコー！MarkdownフォーマッターPrettierはサイコー！MarkdownフォーマッターPrettierはサイコー！MarkdownフォーマッターPrettierはサイコー！MarkdownフォーマッターPrettierはサイコー！MarkdownフォーマッターPrettierはサイコー！MarkdownフォーマッターPrettierはサイコー！MarkdownフォーマッターPrettierはサイコー！MarkdownフォーマッターPrettierはサイコー！MarkdownフォーマッターPrettierはサイコー！MarkdownフォーマッターPrettierはサイコー！\n\nPrettierの最初のCommitは2016年11月29日に行われました。Prettier的第一次Commit是在2016年11月29日。Prettierの最初のCommitは2016年11月29日に行われました。Prettier的第一次Commit是在2016年11月29日。Prettierの最初のCommitは2016年11月29日に行われました。Prettier的第一次Commit是在2016年11月29日。Prettierの最初のCommitは2016年11月29日に行われました。Prettier的第一次Commit是在2016年11月29日。Prettierの最初のCommitは2016年11月29日に行われました。Prettier的第一次Commit是在2016年11月29日。Prettierの最初のCommitは2016年11月29日に行われました。Prettier的第一次Commit是在2016年11月29日。Prettierの最初のCommitは2016年11月29日に行われました。Prettier的第一次Commit是在2016年11月29日。Prettierの最初のCommitは2016年11月29日に行われました。Prettier的第一次Commit是在2016年11月29日。Prettierの最初のCommitは2016年11月29日に行われました。Prettier的第一次Commit是在2016年11月29日。") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段\nParagraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的\n一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中\n文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合\n著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English\n混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個\nEnglish混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是\n一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！\n這是一個English混合著中文的一段Paragraph！\n\nEnglishと日本語が混ざったParagraphが現れた！Prettierはどうする？Englishと日本語\nが混ざったParagraphが現れた！Prettierはどうする？Englishと日本語が混ざった\nParagraphが現れた！Prettierはどうする？Englishと日本語が混ざったParagraphが現れ\nた！Prettierはどうする？Englishと日本語が混ざったParagraphが現れた！Prettierはど\nうする？Englishと日本語が混ざったParagraphが現れた！Prettierはどうする？English\nと日本語が混ざったParagraphが現れた！Prettierはどうする？Englishと日本語が混ざっ\nたParagraphが現れた！Prettierはどうする？Englishと日本語が混ざったParagraphが現\nれた！Prettierはどうする？Englishと日本語が混ざったParagraphが現れた！Prettierは\nどうする？Englishと日本語が混ざったParagraphが現れた！Prettierはどうす\nる？Englishと日本語が混ざったParagraphが現れた！Prettierはどうする？Englishと日\n本語が混ざったParagraphが現れた！Prettierはどうする？Englishと日本語が混ざった\nParagraphが現れた！Prettierはどうする？Englishと日本語が混ざったParagraphが現れ\nた！Prettierはどうする？\n\nMarkdownフォーマッターPrettierはサイコー！MarkdownフォーマッターPrettierはサイ\nコー！MarkdownフォーマッターPrettierはサイコー！MarkdownフォーマッターPrettierは\nサイコー！MarkdownフォーマッターPrettierはサイコー！Markdownフォーマッター\nPrettierはサイコー！MarkdownフォーマッターPrettierはサイコー！Markdownフォーマッ\nターPrettierはサイコー！MarkdownフォーマッターPrettierはサイコー！Markdownフォー\nマッターPrettierはサイコー！MarkdownフォーマッターPrettierはサイコー！Markdown\nフォーマッターPrettierはサイコー！MarkdownフォーマッターPrettierはサイ\nコー！MarkdownフォーマッターPrettierはサイコー！\n\nPrettierの最初のCommitは2016年11月29日に行われました。Prettier的第一次Commit是在\n2016年11月29日。Prettierの最初のCommitは2016年11月29日に行われました。Prettier的\n第一次Commit是在2016年11月29日。Prettierの最初のCommitは2016年11月29日に行われま\nした。Prettier的第一次Commit是在2016年11月29日。Prettierの最初のCommitは2016年11\n月29日に行われました。Prettier的第一次Commit是在2016年11月29日。Prettierの最初の\nCommitは2016年11月29日に行われました。Prettier的第一次Commit是在2016年11月29\n日。Prettierの最初のCommitは2016年11月29日に行われました。Prettier的第一次Commit\n是在2016年11月29日。Prettierの最初のCommitは2016年11月29日に行われまし\nた。Prettier的第一次Commit是在2016年11月29日。Prettierの最初のCommitは2016年11月\n29日に行われました。Prettier的第一次Commit是在2016年11月29日。Prettierの最初の\nCommitは2016年11月29日に行われました。Prettier的第一次Commit是在2016年11月29日。");
}
#[test]
fn test_korean_md_prose_wrapalways_format_1_763c15b1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("예문Latin예문Latin 예문Latin예문 Latin예문Latin 예문 Latin 예문\n\n한국어와 English를 섞어 보았습니다. 한국어와 English를 섞어 보았습니다.\n한국어와\nEnglish를\n섞어\n보았습니다.\n한국어와\nEnglish를\n섞어\n보았습니다.\n한국어와 English를 섞어 보았습니다. 한국어와 English를 섞어 보았습니다.\n한국어와\nEnglish를\n섞어\n보았습니다.\n한국어와\nEnglish를\n섞어\n보았습니다.\n한국어와 English를 섞어 보았습니다. 한국어와 English를 섞어 보았습니다.\n\nNVIDIA의 RTX3000 시리즈는 삼성전자와 TSMC에 의해 제조된다.\nNVIDIA의 RTX3000 시리즈는 삼성전자와 TSMC에 의해 제조된다.\nNVIDIA의\nRTX3000\n시리즈는\n삼성전자와\nTSMC에\n의해\n제조된다.\nNVIDIA의 RTX3000 시리즈는 삼성전자와 TSMC에 의해 제조된다.\nNVIDIA의 RTX3000 시리즈는 삼성전자와 TSMC에 의해 제조된다.\n\n대한민국(듣기 (도움말·정보), 大韓民國, 영어: Republic of Korea; ROK[3])은 동아시아의 한반도 중남부에 있는 공화국이다.\n\n서쪽으로는 서해를 사이에 두고 중화인민공화국이, 동쪽으로는 동해를 사이에 두고 일본이 있으며\n북쪽으로는 조선민주주의인민공화국과 맞닿아 있다.\n\n역대 대통령은 李承晩 과 許政 과 尹潽善 과 朴正熙 과 崔圭夏 과 朴忠勳 과 全斗煥 과 盧泰愚 과 金泳三 과 金大中 과 盧武鉉 과 李明博 과 朴槿惠 과 文在寅 과 尹錫悅 과 음...\n역대 대통령은\n李承晩\n과\n許政\n과\n尹潽善\n과\n朴正熙 과\n崔圭夏 과\n朴忠勳 과\n全斗煥 과\n盧泰愚 과\n金泳三\n과 金大中\n과 盧武鉉\n과 李明博\n과 朴槿惠\n과 文在寅\n과 尹錫悅 과\n음...\n\n역대 대통령은 李承晩과 許政과 尹潽善과 朴正熙과 崔圭夏과 朴忠勳과 全斗煥과 盧泰愚과 金泳三과 金大中과 盧武鉉과 李明博과 朴槿惠과 文在寅과 尹錫悅과 음... 역대 대통령은 李承晩과 許政과 尹潽善과 朴正熙과 崔圭夏과 朴忠勳과 全斗煥과 盧泰愚과 金泳三과 金大中과 盧武鉉과 李明博과 朴槿惠과 文在寅과 尹錫悅과 음...") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "예문Latin예문Latin 예문Latin예문 Latin예문Latin 예문 Latin 예문\n\n한국어와 English를 섞어 보았습니다. 한국어와 English를 섞어 보았습니다. 한국어와\nEnglish를 섞어 보았습니다. 한국어와 English를 섞어 보았습니다. 한국어와\nEnglish를 섞어 보았습니다. 한국어와 English를 섞어 보았습니다. 한국어와\nEnglish를 섞어 보았습니다. 한국어와 English를 섞어 보았습니다. 한국어와\nEnglish를 섞어 보았습니다. 한국어와 English를 섞어 보았습니다.\n\nNVIDIA의 RTX3000 시리즈는 삼성전자와 TSMC에 의해 제조된다. NVIDIA의 RTX3000\n시리즈는 삼성전자와 TSMC에 의해 제조된다. NVIDIA의 RTX3000 시리즈는 삼성전자와\nTSMC에 의해 제조된다. NVIDIA의 RTX3000 시리즈는 삼성전자와 TSMC에 의해 제조된다.\nNVIDIA의 RTX3000 시리즈는 삼성전자와 TSMC에 의해 제조된다.\n\n대한민국(듣기 (도움말·정보), 大韓民國, 영어: Republic of Korea; ROK[3])은\n동아시아의 한반도 중남부에 있는 공화국이다.\n\n서쪽으로는 서해를 사이에 두고 중화인민공화국이, 동쪽으로는 동해를 사이에 두고\n일본이 있으며 북쪽으로는 조선민주주의인민공화국과 맞닿아 있다.\n\n역대 대통령은 李承晩 과 許政 과 尹潽善 과 朴正熙 과 崔圭夏 과 朴忠勳 과 全斗煥\n과 盧泰愚 과 金泳三 과 金大中 과 盧武鉉 과 李明博 과 朴槿惠 과 文在寅 과 尹錫悅\n과 음... 역대 대통령은 李承晩 과 許政 과 尹潽善 과 朴正熙 과 崔圭夏 과 朴忠勳 과\n全斗煥 과 盧泰愚 과 金泳三 과 金大中 과 盧武鉉 과 李明博 과 朴槿惠 과 文在寅 과\n尹錫悅 과 음...\n\n역대 대통령은 李承晩과 許政과 尹潽善과 朴正熙과 崔圭夏과 朴忠勳과 全斗煥과 盧泰\n愚과 金泳三과 金大中과 盧武鉉과 李明博과 朴槿惠과 文在寅과 尹錫悅과 음... 역대\n대통령은 李承晩과 許政과 尹潽善과 朴正熙과 崔圭夏과 朴忠勳과 全斗煥과 盧泰愚과\n金泳三과 金大中과 盧武鉉과 李明博과 朴槿惠과 文在寅과 尹錫悅과 음...");
}
#[test]
fn test_link_md_prose_wrapalways_format_1_44e7f1dd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[這是一段很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長的段落][]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[這是一段很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長的段落][]");
}
#[test]
fn test_mixed_md_prose_wrapalways_format_1_5264ed27() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段\nParagraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的\n一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中\n文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合\n著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English\n混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個\nEnglish混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是\n一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！\n這是一個English混合著中文的一段Paragraph！");
}
#[test]
fn test_space_md_prose_wrapalways_format_1_b140f499() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("這是一個 English 混合著中文的一段 Paragraph！這是一個 English 混合著中文的一段 Paragraph！這是一個 English 混合著中文的一段 Paragraph！這是一個 English 混合著中文的一段 Paragraph！這是一個 English 混合著中文的一段 Paragraph！這是一個 English 混合著中文的一段 Paragraph！這是一個 English 混合著中文的一段 Paragraph！這是一個 English 混合著中文的一段 Paragraph！這是一個 English 混合著中文的一段 Paragraph！這是一個 English 混合著中文的一段 Paragraph！這是一個 English 混合著中文的一段 Paragraph！這是一個 English 混合著中文的一段 Paragraph！這是一個 English 混合著中文的一段 Paragraph！這是一個 English 混合著中文的一段 Paragraph！這是一個 English 混合著中文的一段 Paragraph！這是一個 English 混合著中文的一段 Paragraph！") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "這是一個 English 混合著中文的一段 Paragraph！這是一個 English 混合著中文的一段\nParagraph！這是一個 English 混合著中文的一段 Paragraph！這是一個 English 混合著\n中文的一段 Paragraph！這是一個 English 混合著中文的一段 Paragraph！這是一個\nEnglish 混合著中文的一段 Paragraph！這是一個 English 混合著中文的一段\nParagraph！這是一個 English 混合著中文的一段 Paragraph！這是一個 English 混合著\n中文的一段 Paragraph！這是一個 English 混合著中文的一段 Paragraph！這是一個\nEnglish 混合著中文的一段 Paragraph！這是一個 English 混合著中文的一段\nParagraph！這是一個 English 混合著中文的一段 Paragraph！這是一個 English 混合著\n中文的一段 Paragraph！這是一個 English 混合著中文的一段 Paragraph！這是一個\nEnglish 混合著中文的一段 Paragraph！");
}
#[test]
fn test_symbol_space_new_line_md_prose_wrapalways_format_1_854839b0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("日本語\n、\nにほんご\n。\n汉语,\n中文.\n日\n本\n語\n，\nに\nほ\nん\nご\n．\nEnglish\nwords!?\n漢字\n！\n汉字\n？\n「セリフ」\n(括弧)\n文字\n（括弧）\n文字\n【括弧】\n日本語\nEnglish\n\n「禁則（きんそく）処理（しょり）！」\n「禁則（きんそく）処理（しょり）！」\n「禁則（きんそく）処理（しょり）！」\n「禁則（きんそく）処理（しょり）」\n「禁則（きんそく）処理（しょり）！」\n「禁則（きんそく）処理（しょり）！」\n「禁則（きんそく）処理（しょり）」\n「禁則（きんそく）処理（しょり）」\n「禁則（きんそく）処理（しょり）！！！！」\n「禁則（きんそく）処理（しょり）！！！」\n「禁則（きんそく）処理（しょり）！」\n「禁則（きんそく）処理（しょり）！」\n「禁則（きんそく）処理（しょり）！」\n「禁則（きんそく）処理（しょり）！」\n「禁則（きんそく）処理（しょり）！」\n「禁則（きんそく）処理（しょり）！」\n\n中点\n・\n中点\n\n禁則処理にわざと違反した文章のテストを今から行います。準備はいいでしょうか？レデ\nィ、ゴー！\n\n[ウ\nィキペディア]\n\n[ウ\nィキペディア]: https://ja.wikipedia.org/") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "日本語、にほんご。汉语, 中文. 日本語，にほんご．English words!? 漢字！汉字？「セ\nリフ」(括弧) 文字（括弧）文字【括弧】日本語English\n\n「禁則（きんそく）処理（しょり）！」「禁則（きんそく）処理（しょり）！」「禁則\n（きんそく）処理（しょり）！」「禁則（きんそく）処理（しょり）」「禁則（きんそ\nく）処理（しょり）！」「禁則（きんそく）処理（しょり）！」「禁則（きんそく）処理\n（しょり）」「禁則（きんそく）処理（しょり）」「禁則（きんそく）処理（しょ\nり）！！！！」「禁則（きんそく）処理（しょり）！！！」「禁則（きんそく）処理\n（しょり）！」「禁則（きんそく）処理（しょり）！」「禁則（きんそく）処理（しょ\nり）！」「禁則（きんそく）処理（しょり）！」「禁則（きんそく）処理（しょり）！」\n「禁則（きんそく）処理（しょり）！」\n\n中点・中点\n\n禁則処理にわざと違反した文章のテストを今から行います。準備はいいでしょうか？レ\nディ、ゴー！\n\n[ウ ィキペディア]\n\n[ウ ィキペディア]: https://ja.wikipedia.org/");
}
