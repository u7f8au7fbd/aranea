use linfa::prelude::*;
use linfa_linear::LinearRegression;
use ndarray::{array, Array2};

fn main() {
    // トレーニングデータ（特徴量とラベル）
    let x: Array2<f64> = array![[1.0, 2.0], [2.0, 3.0], [3.0, 4.0], [4.0, 5.0]];
    let y = array![1.0, 2.0, 3.0, 4.0];

    // データセットを作成 (明示的に型を指定)
    let dataset: Dataset<f64, f64, _> = Dataset::new(x, y);

    // 線形回帰モデルを作成し、データセットをフィット
    let model = LinearRegression::new()
        .fit(&dataset)
        .expect("モデルの学習に失敗しました");

    // 新しいデータポイントを作成
    let new_data: Array2<f64> = array![[5.0, 6.0]];

    // 予測を実行
    let prediction = model.predict(&new_data);

    // 予測結果を表示
    println!("予測結果: {:?}", prediction);
}
