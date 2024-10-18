use mods::extract;
use mods::strfn;

mod mods;

const FILE_PATH: &str = "./db/0.html";
const FIRST_QUERY: &str = "北海道";

fn main() {
    //Titleとh1タグに異なる文字が含まれているかを確認する
    let title = extract::extract_title(FILE_PATH);
    let h1 = extract::extract_h1(FILE_PATH);
    println!("{}", title);
    println!("{}", h1);
    println!("{}", strfn::contains_word(&h1, &title));
    //リンクに説明文を付与

    //Titleに第一クエリを設定
    let title = extract::extract_title(FILE_PATH);
    println!("{}", strfn::contains_word(FIRST_QUERY, &title));
    //ボタンと認識可能な名前を設定
    //DOMサイズを1500いかに設定
    //すべての画像のアスペクト比を適切に設定
    //HTMLファイルにDoctype宣言を設定
    //Titleに画像を設定しているか
    //SSLを設定
    //文字・画像が最初に表示される時間を４秒以下に設定
    //見出しを降順に設定
    //Titleの文字数を40文字以下に設定
    //viewportの設定
    //Titleに第二クエリを設定
    //タップターゲットの可動域
    //Descriptionに第一クエリを設定
    //metaタグに第一クエリを設定
    //metaタグにog_urlを設定
    //metaタグにog_typeを設定
    //metaタグにog_imageを設定
    //Descriptionの文字数を120文字以下に設定
    //metaタグにog_descriptionを設定
    //metaタグにog_site_nameを設定
    //h1タグに第一クエリを設定
    //Descriptionに第二クエリを設定
    //すべての画像の解像度を適切に設定
    //Webフォントをロード中に代替テキストの設定
    //テキストの圧縮適応
    //metaキーワードに第一クエリを設定
    //h1タグに第二クエリを設定
    //テキストの文字数を増加
    //Webページの表示速度を６秒以下に
    //metaキーワードに第二クエリを設定
    //リンク化されている画像にalt属性を設定
    //画像の使用枚数を増加
    //アンカーテキストを排除したテキストの文字数を増加
    //テキスト内の第一クエリの使用回数を増加
    //テキスト内の第二クエリの使用回数を増加
    //aタグ以外のテキスト内の第二クエリの使用回数を増加
    //aタグ以外のテキスト内の第一クエリの使用回数を増加
    //主要なコンテンツの表示速度を４秒以下に設定
    //第一クエリを含むaltの個数を増加」
    //Webページが操作できるまでの時間を７秒以下に設定
    //第一クエリに含むstrongタグの使用回数を増加
    //画像に対してキャプションを設定
    //第二クエリに含むstrongタグの使用回数を増加
}
