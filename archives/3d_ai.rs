use rand::Rng;

// 三次元の行列の型エイリアスを定義
type Tensor3D = Vec<Vec<Vec<f64>>>; // 三次元配列
type Vector = Vec<f64>; // 一次元配列

// 三次元行列の各要素を正規化する関数
fn normalize_tensor(tensor: &Tensor3D) -> Tensor3D {
    let mut normalized_tensor = tensor.clone();

    for i in 0..tensor.len() {
        for j in 0..tensor[i].len() {
            let mean: f64 = tensor[i][j].iter().sum::<f64>() / tensor[i][j].len() as f64;
            let std_dev: f64 = (tensor[i][j].iter().map(|x| (x - mean).powi(2)).sum::<f64>()
                / tensor[i][j].len() as f64)
                .sqrt();

            for k in 0..tensor[i][j].len() {
                normalized_tensor[i][j][k] = (tensor[i][j][k] - mean) / std_dev;
            }
        }
    }

    normalized_tensor
}

// 三次元行列の「行列ごとの積」を計算する関数
fn tensor_matmul(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut product = vec![vec![0.0; b[0].len()]; a.len()];
    for i in 0..a.len() {
        for j in 0..b[0].len() {
            for k in 0..b.len() {
                product[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    product
}

// 行列とベクトルの積を計算する関数
fn matvecmul(a: &Vec<Vec<f64>>, b: &Vector) -> Vector {
    a.iter()
        .map(|row| row.iter().zip(b).map(|(x, y)| x * y).sum())
        .collect()
}

// 三次元行列の転置（各行列ごとに転置）を行う関数
fn transpose_tensor(tensor: &Tensor3D) -> Tensor3D {
    tensor
        .iter()
        .map(|matrix| {
            let mut transposed = vec![vec![0.0; matrix.len()]; matrix[0].len()];
            for i in 0..matrix.len() {
                for j in 0..matrix[i].len() {
                    transposed[j][i] = matrix[i][j];
                }
            }
            transposed
        })
        .collect()
}

// 三次元行列の逆行列を計算する関数（2x2行列に限定）
fn inverse_matrix(matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let det = matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
    let inv_det = 1.0 / det;
    vec![
        vec![matrix[1][1] * inv_det, -matrix[0][1] * inv_det],
        vec![-matrix[1][0] * inv_det, matrix[0][0] * inv_det],
    ]
}

// 正則化付き線形回帰モデルの訓練を行う関数（L2正則化付き、三次元対応）
fn linear_regression_with_regularization(tensor: &Tensor3D, y: &Vector, lambda: f64) -> Vector {
    let xt = transpose_tensor(tensor);
    let xtx = tensor
        .iter()
        .zip(xt.iter())
        .map(|(a, b)| tensor_matmul(a, b))
        .collect::<Vec<_>>();

    let identity: Vec<Vec<Vec<f64>>> = xtx
        .iter()
        .map(|xtx_matrix| {
            (0..xtx_matrix.len())
                .map(|i| {
                    (0..xtx_matrix.len())
                        .map(|j| if i == j { lambda } else { 0.0 })
                        .collect()
                })
                .collect()
        })
        .collect();

    let xtx_reg = xtx
        .iter()
        .zip(identity.iter())
        .map(|(xtx_matrix, identity_matrix)| {
            xtx_matrix
                .iter()
                .zip(identity_matrix.iter())
                .map(|(xtx_row, identity_row)| {
                    xtx_row
                        .iter()
                        .zip(identity_row)
                        .map(|(a, b)| a + b)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let xtx_inv = xtx_reg
        .iter()
        .map(|xtx_matrix| inverse_matrix(xtx_matrix))
        .collect::<Vec<_>>();

    let xty = tensor
        .iter()
        .map(|matrix| matvecmul(matrix, y))
        .collect::<Vec<_>>();

    xtx_inv
        .iter()
        .zip(xty.iter())
        .map(|(inv_matrix, y_vec)| matvecmul(inv_matrix, y_vec)[0])
        .collect()
}

// データをスコアリングし、元の順番に並び替える関数
fn sort_by_prediction(data: &Tensor3D, coefficients: &Vector) -> Vec<(usize, f64, Vec<Vec<f64>>)> {
    let mut scored_data: Vec<(usize, f64, Vec<Vec<f64>>)> = data
        .iter()
        .enumerate()
        .map(|(i, matrix)| (i, matvecmul(&matrix, coefficients)[0], matrix.clone()))
        .collect();

    scored_data.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    scored_data
}

// 並び替えたデータと正しいデータを評価する関数
fn evaluate_sorted_data(
    sorted_data: &[(usize, f64, Vec<Vec<f64>>)],
    true_data: &Tensor3D,
) -> usize {
    sorted_data
        .iter()
        .zip(true_data.iter())
        .filter(|((_, _, predicted), true_matrix)| {
            predicted.iter().zip(true_matrix.iter()).all(|(a, b)| {
                a.iter()
                    .zip(b.iter())
                    .all(|(x, y)| (x - y).abs() < f64::EPSILON)
            })
        })
        .count()
}

fn main() {
    // 三次元データ
    let x = vec![
        vec![vec![1.0, 3.0], vec![16.0, 2.0], vec![60.0, 4.2]],
        vec![vec![2.0, 9.0], vec![99.0, 88.0], vec![82.0, 10.0]],
        vec![vec![3.0, 32.0], vec![16.0, 2.0], vec![60.0, 4.2]],
        vec![vec![4.0, 1.0], vec![99.0, 88.0], vec![82.0, 10.0]],
        vec![vec![5.0, 43.0], vec![16.0, 2.0], vec![60.0, 4.2]],
        vec![vec![6.0, 40.0], vec![99.0, 88.0], vec![82.0, 10.0]],
    ];

    // 正しい新しいデータの順位（三次元対応）
    let true_new_data = vec![
        vec![vec![21.0, 16.0], vec![60.0, 20.0], vec![5.0, 71.0]],
        vec![vec![52.0, 49.0], vec![99.0, 88.0], vec![82.0, 10.0]],
        vec![vec![92.0, 16.0], vec![60.0, 20.0], vec![5.0, 71.0]],
        vec![vec![101.0, 49.0], vec![99.0, 88.0], vec![82.0, 10.0]],
    ];

    // 新しいバラバラのデータ群（三次元対応）
    let new_data = vec![
        vec![vec![92.0, 16.0], vec![60.0, 20.0], vec![5.0, 71.0]],
        vec![vec![101.0, 49.0], vec![99.0, 88.0], vec![82.0, 10.0]],
        vec![vec![21.0, 16.0], vec![60.0, 20.0], vec![5.0, 71.0]],
        vec![vec![52.0, 49.0], vec![99.0, 88.0], vec![82.0, 10.0]],
    ];

    let mut rng = rand::thread_rng();
    let mut correct_count = 0;
    let mut coefficients = vec![];

    while correct_count < true_new_data.len() {
        let y: Vector = (0..x.len()).map(|_| rng.gen_range(0.0..1.0)).collect();

        let normalized_x = normalize_tensor(&x);

        let lambda = 0.1;
        coefficients = linear_regression_with_regularization(&normalized_x, &y, lambda);

        let sorted_data = sort_by_prediction(&new_data, &coefficients);

        println!("並び替えた後のデータ:");
        for (index, score, data) in &sorted_data {
            println!(
                "データ番号: {}, スコア: {}, データ: {:?}",
                index, score, data
            );
        }

        correct_count = evaluate_sorted_data(&sorted_data, &true_new_data);
    }

    println!("正解にたどり着いたターゲット変数: {:?}", coefficients);
}
