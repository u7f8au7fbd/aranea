use rand::Rng;

// 行列とベクトルの型エイリアスを定義
type Matrix = Vec<Vec<f64>>;
type Vector = Vec<f64>;

// 行列の転置を行う関数
fn transpose(matrix: &Matrix) -> Matrix {
    // 転置行列のサイズを設定し、0.0で初期化された行列を作成
    let mut transposed = vec![vec![0.0; matrix.len()]; matrix[0].len()];
    // 元の行列をイテレーションして転置を行う
    for i in 0..matrix.len() {
        // 転置行列の各行をイテレーション
        for (j, row) in transposed.iter_mut().enumerate() {
            row[i] = matrix[i][j]; // 元の行列の要素を転置行列に代入
        }
    }
    transposed // 転置された行列を返す
}

// 行列同士の積を計算する関数
fn matmul(a: &Matrix, b: &Matrix) -> Matrix {
    // 積の結果を格納する行列を初期化
    let mut product = vec![vec![0.0; b[0].len()]; a.len()];
    // 行列aの各行に対してループ
    for i in 0..a.len() {
        // 行列bの各列に対してループ
        for j in 0..b[0].len() {
            // 行列bの行をイテレーションし、積を計算
            for (k, b_elem) in b.iter().enumerate() {
                product[i][j] += a[i][k] * b_elem[j]; // 行列の要素の積を加算
            }
        }
    }
    product // 積の結果を返す
}

// 行列とベクトルの積を計算する関数
fn matvecmul(a: &Matrix, b: &Vector) -> Vector {
    // 行列aの各行とベクトルbのドット積を計算し、結果をベクトルとして返す
    a.iter()
        .map(|row| row.iter().zip(b).map(|(x, y)| x * y).sum()) // ドット積を計算
        .collect() // 結果をベクトルとして収集
}

// 行列の逆行列を計算する関数（2x2の簡単なケースに限る）
fn inverse(matrix: &Matrix) -> Matrix {
    // 行列の行列式（determinant）を計算
    let det = matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
    // 行列式の逆数を計算
    let inv_det = 1.0 / det;
    // 逆行列を計算し、2x2行列として返す
    vec![
        vec![matrix[1][1] * inv_det, -matrix[0][1] * inv_det], // 上半分の要素
        vec![-matrix[1][0] * inv_det, matrix[0][0] * inv_det], // 下半分の要素
    ]
}

// データの正規化を行う関数
fn normalize(
    // 関数の宣言。行列の各列を正規化した新しい行列を返す
    matrix: &Matrix, // 入力行列の参照
) -> Matrix {
    // 戻り値は正規化された行列
    let mut normalized = matrix.clone(); // 入力行列をクローンして、正規化された行列を作成
    for j in 0..matrix[0].len() {
        // 各列に対して繰り返し処理を行う
        let mean = matrix // 平均を計算
            .iter() // 行ごとにイテレート
            .map(|row| row[j]) // 各行の該当する列の値を取得
            .sum::<f64>()
            / matrix.len() as f64; // 値の合計を行数で割り、平均を求める

        let std_dev = (matrix // 標準偏差を計算
            .iter() // 行ごとにイテレート
            .map(|row| (row[j] - mean).powi(2)) // 各行の該当する列の値と平均との差の二乗を計算
            .sum::<f64>() // その合計を計算
            / matrix.len() as f64) // 行数で割る
            .sqrt(); // 平方根を取り、標準偏差を求める

        for i in 0..matrix.len() {
            // 各行に対して繰り返し処理を行う
            normalized[i][j] = (matrix[i][j] - mean) / std_dev; // 該当する値を正規化し、normalizedに代入
        }
    }
    normalized // 正規化された行列を返す
}

// 正則化付き線形回帰モデルの訓練を行う関数
fn linear_regression_with_regularization(x: &Matrix, y: &Vector, lambda: f64) -> Vector {
    // 行列xを転置する
    let xt = transpose(x);

    // 転置行列xtと行列xの積を計算する
    let xtx = matmul(&xt, x);

    // 単位行列にλを掛けた行列identityを作成する
    let identity: Vec<Vec<f64>> = (0..xtx.len())
        .map(|i| {
            (0..xtx.len())
                .map(|j| if i == j { lambda } else { 0.0 }) // 対角成分にλを、それ以外には0を配置
                .collect()
        })
        .collect();

    // xtx行列と正則化項identityを足し合わせた行列xtx_regを作成する
    let xtx_reg = xtx
        .iter()
        .zip(identity.iter())
        .map(|(row_xtx, row_identity)| {
            row_xtx
                .iter()
                .zip(row_identity)
                .map(|(&v_xtx, &v_identity)| v_xtx + v_identity) // 対応する要素同士を足す
                .collect()
        })
        .collect();

    // 正則化されたxtx行列の逆行列を計算する
    let xtx_inv = inverse(&xtx_reg);

    // 転置行列xtとベクトルyの積を計算する
    let xty = matvecmul(&xt, y);

    // 逆行列xtx_invとxtyベクトルの積を計算し、回帰係数ベクトルを返す
    matvecmul(&xtx_inv, &xty)
}

// データをスコアリングし、元の順番に並び替える関数
fn sort_by_prediction(
    // 関数の宣言。データと係数を使用して予測値でソートされたデータを返す
    data: &Matrix,         // 入力データの参照。Matrix型は行列を表す
    coefficients: &Vector, // 予測に使用する係数ベクターの参照
) -> Vec<(usize, f64, Vec<f64>)> {
    // 関数の戻り値は、(インデックス, スコア, 元データ)を要素とするタプルのベクタ
    let mut scored_data: Vec<(
        // scored_dataという可変なベクタを宣言し、値を代入
        usize,    // インデックス（行番号）
        f64,      // 予測値（スコア）
        Vec<f64>, // 元データのベクター
    )> = data // dataをイテレーション
        .iter() // 行ごとにイテレートするイテレータを取得
        .enumerate() // 各行にインデックスを付与
        .map(|(i, x)| {
            (
                // 各行に対して、インデックスと予測値、元データのタプルを作成
                i,                                            // 行番号（インデックス）
                matvecmul(&vec![x.clone()], coefficients)[0], // 行と係数ベクターの行列積を計算し、最初の要素（スコア）を取得
                x.clone(),                                    // 元データのベクターをクローン
            )
        })
        .collect(); // すべてのタプルをベクターに収集

    scored_data.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); // 予測値（スコア）に基づいて降順にソート
    scored_data // ソートされたデータを返す
}

fn evaluate_sorted_data(
    // 関数の宣言。sorted_dataとtrue_dataを引数に取り、正しくソートされた行の数を返す
    sorted_data: &[(usize, f64, Vec<f64>)], // ソートされたデータの参照。タプルで、(インデックス, 評価値, ベクター)を持つ
    true_data: &[Vec<f64>],                 // 真のデータの参照。各行がf64のベクターで構成される
) -> usize {
    // 関数の戻り値は正しくソートされた行の数（usize）
    sorted_data // sorted_dataをイテレーション
        .iter() // イテレータを取得
        .zip(true_data.iter()) // true_dataと対応する要素をペアにする
        .filter(|((_, _, predicted), true_row)| {
            // ソートされたデータと真のデータを比較するフィルタを適用
            predicted // predicted（予測されたベクター）を取り出し
                .iter() // ベクターの要素をイテレート
                .zip(true_row.iter()) // true_row（真のデータのベクター）と要素ごとにペアにする
                .all(|(a, b)| (a - b).abs() < f64::EPSILON) // すべての要素について、差がf64::EPSILON未満かチェック
        })
        .count() // 条件を満たす行の数をカウント
}

fn main() {
    // 元データと順位のラベル
    let x = vec![
        vec![1.0, 16.0, 60.0, 20.0, 5.0, 71.0, 8.0, 26.0, 90.0, 34.0],
        vec![2.0, 49.0, 99.0, 88.0, 82.0, 10.0, 68.0, 14.0, 19.0, 62.0],
        vec![3.0, 34.0, 68.0, 89.0, 38.0, 27.0, 77.0, 79.0, 78.0, 42.0],
        vec![4.0, 9.0, 97.0, 55.0, 63.0, 36.0, 58.0, 29.0, 21.0, 45.0],
        vec![5.0, 78.0, 51.0, 76.0, 50.0, 89.0, 40.0, 13.0, 78.0, 71.0],
        vec![6.0, 17.0, 51.0, 56.0, 90.0, 17.0, 67.0, 69.0, 29.0, 82.0],
        vec![7.0, 42.0, 57.0, 34.0, 88.0, 45.0, 72.0, 58.0, 58.0, 8.0],
        vec![8.0, 28.0, 38.0, 23.0, 91.0, 12.0, 99.0, 62.0, 90.0, 92.0],
        vec![9.0, 62.0, 70.0, 63.0, 2.0, 96.0, 18.0, 30.0, 84.0, 100.0],
        vec![10.0, 17.0, 6.0, 16.0, 85.0, 94.0, 71.0, 4.0, 53.0, 23.0],
    ];

    // 正しい新しいデータの順位
    let true_new_data = vec![
        vec![7.0, 42.0, 57.0, 34.0, 88.0, 45.0, 72.0, 58.0, 58.0, 8.0],
        vec![8.0, 28.0, 38.0, 23.0, 91.0, 12.0, 99.0, 62.0, 90.0, 92.0],
        vec![14.0, 9.0, 97.0, 55.0, 63.0, 36.0, 58.0, 29.0, 21.0, 45.0],
        vec![15.0, 78.0, 51.0, 76.0, 50.0, 89.0, 40.0, 13.0, 78.0, 71.0],
        vec![20.0, 17.0, 6.0, 16.0, 85.0, 94.0, 71.0, 4.0, 53.0, 23.0],
        vec![21.0, 16.0, 60.0, 20.0, 5.0, 71.0, 8.0, 26.0, 90.0, 34.0],
        vec![23.0, 34.0, 68.0, 89.0, 38.0, 27.0, 77.0, 79.0, 78.0, 42.0],
        vec![29.0, 62.0, 70.0, 63.0, 2.0, 96.0, 18.0, 30.0, 84.0, 100.0],
        vec![46.0, 17.0, 51.0, 56.0, 90.0, 17.0, 67.0, 69.0, 29.0, 82.0],
        vec![52.0, 49.0, 99.0, 88.0, 82.0, 10.0, 68.0, 14.0, 19.0, 62.0],
    ];

    // 新しいバラバラのデータ群
    let new_data = vec![
        vec![21.0, 16.0, 60.0, 20.0, 5.0, 71.0, 8.0, 26.0, 90.0, 34.0],
        vec![52.0, 49.0, 99.0, 88.0, 82.0, 10.0, 68.0, 14.0, 19.0, 62.0],
        vec![23.0, 34.0, 68.0, 89.0, 38.0, 27.0, 77.0, 79.0, 78.0, 42.0],
        vec![14.0, 9.0, 97.0, 55.0, 63.0, 36.0, 58.0, 29.0, 21.0, 45.0],
        vec![15.0, 78.0, 51.0, 76.0, 50.0, 89.0, 40.0, 13.0, 78.0, 71.0],
        vec![46.0, 17.0, 51.0, 56.0, 90.0, 17.0, 67.0, 69.0, 29.0, 82.0],
        vec![7.0, 42.0, 57.0, 34.0, 88.0, 45.0, 72.0, 58.0, 58.0, 8.0],
        vec![8.0, 28.0, 38.0, 23.0, 91.0, 12.0, 99.0, 62.0, 90.0, 92.0],
        vec![29.0, 62.0, 70.0, 63.0, 2.0, 96.0, 18.0, 30.0, 84.0, 100.0],
        vec![20.0, 17.0, 6.0, 16.0, 85.0, 94.0, 71.0, 4.0, 53.0, 23.0],
    ];

    let mut rng = rand::thread_rng();
    let mut correct_count = 0;
    let mut coefficients = vec![];

    while correct_count < true_new_data.len() {
        // 動的にターゲット変数をランダムに設定
        let y: Vector = (0..x.len()).map(|_| rng.gen_range(0.0..1.0)).collect();

        // データの正規化
        let normalized_x = normalize(&x);

        // モデルの訓練（L2正則化付き）
        let lambda = 0.1;
        coefficients = linear_regression_with_regularization(&normalized_x, &y, lambda);

        // スコアリングして並び替え
        let sorted_data = sort_by_prediction(&new_data, &coefficients);

        // 並び替えた結果と正解データを比較して一致する行数を計算
        correct_count = evaluate_sorted_data(&sorted_data, &true_new_data);

        println!("並び替えた後のデータ:");
        for (index, score, data) in &sorted_data {
            println!(
                "データ番号: {}, スコア: {}, データ: {:?}",
                index, score, data
            );
        }
    }

    // 正解にたどり着いた際のターゲット変数を表示
    println!("正解にたどり着いたターゲット変数: {:?}", coefficients);
}
