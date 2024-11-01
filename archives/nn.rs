use ndarray::{Array1, Array2, Axis};
use rand::distributions::Uniform;
use rand::prelude::*;

// シグモイド関数
fn sigmoid(x: &Array1<f64>) -> Array1<f64> {
    x.mapv(|v| 1.0 / (1.0 + (-v).exp()))
}

// シグモイドの導関数: sigmoid(x) * (1 - sigmoid(x))
fn sigmoid_derivative(sig: &Array1<f64>) -> Array1<f64> {
    sig * (1.0 - sig)
}

// 二乗誤差
fn mean_squared_error(y_true: &Array1<f64>, y_pred: &Array1<f64>) -> f64 {
    y_true
        .iter()
        .zip(y_pred.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f64>()
        / y_true.len() as f64
}

// ニューラルネットワーク構造
struct NeuralNetwork {
    input_size: usize,
    hidden_size: usize,
    output_size: usize,
    // 重みとバイアス
    w1: Array2<f64>, // 入力層から隠れ層
    b1: Array1<f64>,
    w2: Array2<f64>, // 隠れ層から出力層
    b2: Array1<f64>,
}

impl NeuralNetwork {
    fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        let mut rng = thread_rng();
        let uniform = Uniform::new(-1.0, 1.0);
        // 重みをランダムに初期化
        let w1 = Array2::from_shape_fn((hidden_size, input_size), |_| rng.sample(&uniform));
        let b1 = Array1::from_shape_fn(hidden_size, |_| rng.sample(&uniform));
        let w2 = Array2::from_shape_fn((output_size, hidden_size), |_| rng.sample(&uniform));
        let b2 = Array1::from_shape_fn(output_size, |_| rng.sample(&uniform));

        NeuralNetwork {
            input_size,
            hidden_size,
            output_size,
            w1,
            b1,
            w2,
            b2,
        }
    }

    // フォワードパス
    fn forward(&self, input: &Array1<f64>) -> (Array1<f64>, Array1<f64>, Array1<f64>, Array1<f64>) {
        let z1 = self.w1.dot(input) + &self.b1;
        let a1 = sigmoid(&z1);
        let z2 = self.w2.dot(&a1) + &self.b2;
        let a2 = z2.clone(); // 出力層は線形活性化（活性化なし）
        (z1, a1, z2, a2)
    }

    // トレーニング（単純なバックプロパゲーション）
    fn train(&mut self, inputs: &Array2<f64>, targets: &Array2<f64>, epochs: usize, lr: f64) {
        for epoch in 0..epochs {
            let mut total_loss = 0.0;

            for (input, target) in inputs.outer_iter().zip(targets.outer_iter()) {
                let input = input.to_owned();
                let target = target.to_owned();

                // フォワードパス
                let (z1, a1, z2, a2) = self.forward(&input);

                // 損失の計算
                let loss = mean_squared_error(&target, &a2);
                total_loss += loss;

                // バックプロパゲーション
                let error_output = &a2 - &target;
                let delta2 = error_output.clone(); // 出力層は線形活性化のため導関数は1

                let error_hidden = self.w2.t().dot(&delta2);
                let delta1 = &error_hidden * &sigmoid_derivative(&a1);

                // 重みとバイアスの更新
                self.w2 -= &(delta2
                    .view()
                    .insert_axis(Axis(1))
                    .dot(&a1.view().insert_axis(Axis(0)))
                    * lr);
                self.b2 -= &(delta2 * lr);

                self.w1 -= &(delta1
                    .view()
                    .insert_axis(Axis(1))
                    .dot(&input.view().insert_axis(Axis(0)))
                    * lr);
                self.b1 -= &(delta1 * lr);
            }

            if epoch % 1000 == 0 {
                println!(
                    "Epoch {}: Loss {}",
                    epoch,
                    total_loss / inputs.nrows() as f64
                );
            }
        }
    }

    // 予測
    fn predict(&self, input: &Array1<f64>) -> (Array1<f64>, Array1<f64>, Array1<f64>, Array1<f64>) {
        self.forward(input)
    }
}

fn main() {
    // 教師データの定義
    // ユーザーの要求に基づいて、d1~d8の8つの入力とdaの出力を設定
    // 各サンプルは累積的に1が増えていく形式
    let training_inputs = Array2::from_shape_vec(
        (8, 8), // 8サンプル、8特徴
        vec![
            2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, // da1
            1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 8.0, // da2
            1.0, 0.0, 1.0, 3.0, 0.0, 0.0, 0.0, 0.0, // da3
            1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, // da4
            1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 5.0, // da5
            1.0, 1.0, 1.0, 2.0, 8.0, 1.0, 0.0, 0.0, // da6
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 9.0, // da7
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, // da8
        ],
    )
    .expect("データの形状が不正です");

    let training_targets = Array2::from_shape_vec(
        (8, 1), // 8サンプル、1出力
        vec![
            2.0,  // da1
            10.0, // da2
            5.0,  // da3
            2.0,  // da4
            10.0, // da5
            14.0, // da6
            16.0, // da7
            8.0,  // da8
        ],
    )
    .expect("データの形状が不正です");

    // ニューラルネットワークの初期化
    let mut nn = NeuralNetwork::new(8, 16, 1);

    // トレーニング
    println!("--- トレーニング開始 ---");
    nn.train(&training_inputs, &training_targets, 10000, 0.1);
    println!("--- トレーニング終了 ---\n");

    // 推論と詳細な出力
    println!("--- 推論結果 ---");
    for (i, input) in training_inputs.outer_iter().enumerate() {
        let input = input.to_owned();
        let (z1, a1, z2, a2) = nn.predict(&input);
        let target = training_targets.row(i).to_owned();
        let loss = mean_squared_error(&target, &a2);
        println!("サンプル {}:", i + 1);
        println!("入力: {:?}", input);
        println!("隠れ層の出力 (a1): {:?}", a1);
        println!("最終出力 (a2): {:?}", a2);
        println!("期待される出力: {:?}", target);
        println!("損失: {:.4}", loss);
        println!("---------------------------");
    }

    // 任意のテスト入力に対する予測
    let test_input = Array1::from(vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 9.0, 0.0]);
    let (z1, a1, z2, a2) = nn.predict(&test_input);
    let test_target = 15.0; // 期待される出力
    let test_loss = ((a2[0] - test_target).powi(2)) / 1.0;
    println!("\n--- テスト入力に対する予測 ---");
    println!("入力: {:?}", test_input);
    println!("隠れ層の出力 (a1): {:?}", a1);
    println!("最終出力 (a2): {:?}", a2);
    println!("期待される出力: {}", test_target);
    println!("損失: {:.4}", test_loss);
}
