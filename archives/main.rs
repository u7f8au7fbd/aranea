use ndarray::{Array1, Array2};
use linfa::prelude::*;
use linfa_trees::DecisionTree;
use flate2::read::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

// バイナリデータを圧縮する関数
fn compress_data(data: &[u8]) -> Vec<u8> {
    let mut encoder = GzEncoder::new(data, Compression::default());
    let mut compressed_data = Vec::new();
    encoder.read_to_end(&mut compressed_data).expect("圧縮に失敗しました");
    compressed_data
}

// バイナリデータを読み込む関数
fn load_binary_data(file_path: &str) -> Vec<u8> {
    let mut file = File::open(file_path).expect("ファイルが開けませんでした");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("読み取りエラー");
    buffer
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 圧縮されたバイナリデータを格納
    let mut compressed_data_list = Vec::new();
    let mut max_compressed_size = 0; // 最も大きい圧縮データのサイズを保持する変数

    println!("各ファイルを圧縮し、最大の圧縮サイズを計算中...");

    // 各HTMLファイルを圧縮し、最大の圧縮後のサイズを求める
    for i in 1..=10 {
        let file_path = format!("./htmls/{}.html", i);
        if Path::new(&file_path).exists() {
            let binary_data = load_binary_data(&file_path);

            // データを圧縮する
            let compressed_data = compress_data(&binary_data);
            println!("{} を圧縮しました: {} -> {}", file_path, binary_data.len(), compressed_data.len());

            // 圧縮データをリストに追加
            compressed_data_list.push(compressed_data.clone());

            // 最大の圧縮データサイズを更新
            max_compressed_size = max_compressed_size.max(compressed_data.len());
        } else {
            println!("{} は存在しません", file_path);
        }
    }

    println!("最大の圧縮データサイズ: {}", max_compressed_size);

    // 圧縮されたデータを基準にゼロパディングして、データを揃える
    let mut padded_data_list = Vec::new();

    for (i, compressed_data) in compressed_data_list.into_iter().enumerate() {
        // 圧縮データが最大サイズに満たない場合、ゼロパディングを行う
        let mut padded_data = compressed_data.clone();
        if padded_data.len() < max_compressed_size {
            println!("データ {} にゼロパディング: {} -> {}", i + 1, padded_data.len(), max_compressed_size);
            padded_data.resize(max_compressed_size, 0); // ゼロパディング
        }

        // ゼロパディングされたデータをf64に変換して保存
        let binary_as_f64: Vec<f64> = padded_data.iter().map(|&b| b as f64).collect();
        padded_data_list.push(binary_as_f64);
    }

    // ndarrayの2次元配列に変換（サンプル × 特徴量）
    let rows = padded_data_list.len();
    let cols = max_compressed_size;
    let flat_data: Vec<f64> = padded_data_list.into_iter().flatten().collect();

    println!("データを配列に変換中: {} 行 × {} 列", rows, cols);

    // データ長がshapeに適合しているか確認
    if flat_data.len() != rows * cols {
        return Err(format!(
            "データの長さがshapeに適合しません: {} != {}",
            flat_data.len(),
            rows * cols
        ).into());
    }

    let array = Array2::from_shape_vec((rows, cols), flat_data)?;

    println!("データセットを作成中...");

    // 評価ラベル (1~10の順位に対応) をusizeにする
    let targets = Array1::from_iter(1..=10);

    // 決定木モデルのトレーニング
    println!("モデルをトレーニング中...");
    let dataset = Dataset::new(array, targets);
    let model = DecisionTree::params().fit(&dataset)?;

    // ./re/target.htmlのバイナリデータを読み込んで予測する
    println!("./re/target.html のバイナリデータを読み込み中...");
    let target_file = load_binary_data("./re/target.html");

    // データを圧縮する
    let compressed_target = compress_data(&target_file);

    // ターゲットデータも同じようにゼロパディング
    let mut padded_target = compressed_target.clone();
    if padded_target.len() < max_compressed_size {
        println!("./re/target.html にゼロパディング: {} -> {}", padded_target.len(), max_compressed_size);
        padded_target.resize(max_compressed_size, 0); // ゼロパディング
    }

    let target_as_f64: Vec<f64> = padded_target.iter().map(|&b| b as f64).collect();
    let target_array = Array2::from_shape_vec((1, max_compressed_size), target_as_f64)?;

    println!("予測を実行中...");
    // 予測結果を表示
    let prediction = model.predict(&target_array);
    println!("予測された順位: {:?}", prediction);

    Ok(())
}
