static TARGET_URL: &str = "https://www.rust-lang.org/";

// ANSIカラーコード定数
#[macro_use]
mod macros;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //HTMLの取得
    let body = reqwest::get(TARGET_URL).await?.text().await?;
    println!("{}", mold_html!(&body));
    Ok(())
}
