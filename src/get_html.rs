// ヘッディングタグをまとめた構造体
#[derive(Debug, Default)]
pub struct HTags {
    h1: i32,
    h2: i32,
    h3: i32,
    h4: i32,
    h5: i32,
    h6: i32,
}

// テキスト関連タグをまとめた構造体
#[derive(Debug, Default)]
pub struct TextTags {
    p: i32,
    strong: i32,
    em: i32,
    span: i32,
    b: i32,
    i: i32,
    small: i32,
    mark: i32,
    del: i32,
    ins: i32,
    sub: i32,
    sup: i32,
    code: i32,
    var_: i32, // varは予約語なので var_ を使用
    samp: i32,
    kbd: i32,
    q: i32,
    blockquote: i32,
    pre: i32,
}

// リスト関連のタグをまとめた構造体
#[derive(Debug, Default)]
pub struct ListTags {
    ul: i32,
    ol: i32,
    li: i32,
    dl: i32,
    dt: i32,
    dd: i32,
}

// フォーム関連のタグをまとめた構造体
#[derive(Debug, Default)]
pub struct FormTags {
    form: i32,
    input: i32,
    textarea: i32,
    button: i32,
    select: i32,
    option: i32,
    optgroup: i32,
    fieldset: i32,
    legend: i32,
    label: i32,
    datalist: i32,
    output: i32,
    progress: i32,
    meter: i32,
}

// メディア関連のタグをまとめた構造体
#[derive(Debug, Default)]
pub struct MediaTags {
    img: i32,
    audio: i32,
    video: i32,
    source: i32,
    track: i32,
    map: i32,
    area: i32,
    picture: i32,
    canvas: i32,
    svg: i32,
    object: i32,
    embed: i32,
    iframe: i32,
}

// テーブル関連のタグをまとめた構造体
#[derive(Debug, Default)]
pub struct TableTags {
    table: i32,
    caption: i32,
    thead: i32,
    tbody: i32,
    tfoot: i32,
    tr: i32,
    th: i32,
    td: i32,
    col: i32,
    colgroup: i32,
}

// メタデータ関連のタグをまとめた構造体
#[derive(Debug, Default)]
pub struct MetaTags {
    head: i32,
    meta: i32,
    link: i32,
    style: i32,
    title: i32,
    base: i32,
    script: i32,
    noscript: i32,
    template: i32,
}

// セクショニング関連のタグをまとめた構造体
#[derive(Debug, Default)]
pub struct SectionTags {
    header: i32,
    nav: i32,
    section: i32,
    article: i32,
    aside: i32,
    footer: i32,
    main: i32,
    address: i32,
}

// HTML全体の構造体に、他の構造体をネスト
#[derive(Debug, Default)]
pub struct HtmlTags {
    headings: HTags,       // ヘッディングタグ (h1 ～ h6)
    text: TextTags,        // テキスト関連のタグ
    lists: ListTags,       // リスト関連のタグ
    forms: FormTags,       // フォーム関連のタグ
    media: MediaTags,      // メディア関連のタグ
    tables: TableTags,     // テーブル関連のタグ
    meta: MetaTags,        // メタデータ関連のタグ
    sections: SectionTags, // セクショニング関連のタグ
    html: i32,             // 個別の html タグ
    body: i32,             // 個別の body タグ
    div: i32,              // 個別の div タグ
    a: i32,                // 個別の a タグ
    br: i32,               // 個別の br タグ
    hr: i32,               // 個別の hr タグ
    wbr: i32,              // 個別の wbr タグ
}
