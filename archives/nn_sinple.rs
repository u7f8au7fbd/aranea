use nalgebra::{DMatrix, DVector};
use rand::random;

struct SimpleNeuralNetwork {
    weights_input_hidden: DMatrix<f32>,
    weights_hidden_output: DMatrix<f32>,
}

impl SimpleNeuralNetwork {
    fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        let weights_input_hidden =
            DMatrix::from_fn(hidden_size, input_size, |_, _| random::<f32>() * 2.0 - 1.0);
        let weights_hidden_output =
            DMatrix::from_fn(output_size, hidden_size, |_, _| random::<f32>() * 2.0 - 1.0);

        SimpleNeuralNetwork {
            weights_input_hidden,
            weights_hidden_output,
        }
    }

    fn forward(&self, input: &DVector<f32>) -> (DVector<f32>, DVector<f32>) {
        let hidden = (self.weights_input_hidden.clone() * input).map(|x| x.max(0.0));
        let output = self.weights_hidden_output.clone() * &hidden;
        (hidden, output)
    }

    fn train(
        &mut self,
        inputs: &DMatrix<f32>,
        targets: &DMatrix<f32>,
        learning_rate: f32,
        epochs: usize,
    ) {
        for _ in 0..epochs {
            for (input_row, target_row) in inputs.row_iter().zip(targets.row_iter()) {
                // 各行をベクトルに変換
                let input = DVector::from_vec(input_row.iter().cloned().collect());
                let target = DVector::from_vec(target_row.iter().cloned().collect());

                // 前向き伝播
                let (hidden, output) = self.forward(&input);

                // 誤差計算
                let error = &output - &target;

                // 勾配計算
                let grad_hidden_output = error.clone() * hidden.transpose();
                let delta_hidden = self.weights_hidden_output.transpose() * &error;
                let grad_input_hidden =
                    delta_hidden.component_mul(&hidden.map(|x| if x > 0.0 { 1.0 } else { 0.0 }))
                        * input.transpose();

                // 重み更新
                self.weights_hidden_output -= grad_hidden_output * learning_rate;
                self.weights_input_hidden -= grad_input_hidden * learning_rate;
            }
        }
    }
}

fn main() {
    let input_size = 2;
    let hidden_size = 4;
    let output_size = 1;

    let mut nn = SimpleNeuralNetwork::new(input_size, hidden_size, output_size);

    // データは適宜入力
    let x: Vec<f32> = vec![
        8.0, 7.0, 8.0, 5.0, 8.0, 3.0, 6.0, 5.0, 9.0, 8.0, 6.0, 3.0, 8.0, 9.0, 7.0, 3.0, 7.0, 3.0,
        3.0, 6.0, 7.0, 3.0, 6.0, 8.0, 2.0, 2.0, 7.0, 9.0, 3.0, 4.0, 6.0, 1.0, 9.0, 7.0, 3.0, 7.0,
        5.0, 3.0, 8.0, 5.0, 7.0, 5.0, 6.0, 6.0, 7.0, 7.0, 2.0, 1.0, 5.0, 2.0, 4.0, 7.0, 4.0, 4.0,
        8.0, 1.0, 3.0, 5.0, 1.0, 1.0, 1.0, 7.0, 2.0, 5.0,
    ];
    let y: Vec<f32> = vec![
        7.0, 4.0, 1.0, 4.0, 9.0, 7.0, 8.0, 2.0, 1.0, 4.0, 7.0, 9.0, 3.0, 8.0, 8.0, 5.0, 6.0, 8.0,
        2.0, 8.0, 8.0, 4.0, 5.0, 5.0, 9.0, 6.0, 5.0, 9.0, 8.0, 3.0, 6.0, 2.0, 4.0, 3.0, 4.0, 8.0,
        9.0, 5.0, 5.0, 6.0, 2.0, 8.0, 8.0, 3.0, 9.0, 8.0, 9.0, 9.0, 7.0, 5.0, 7.0, 3.0, 3.0, 8.0,
        4.0, 3.0, 4.0, 3.0, 5.0, 8.0, 8.0, 7.0, 4.0, 9.0,
    ];
    let z: Vec<f32> = vec![
        7.5, 5.5, 4.5, 4.5, 8.5, 5.0, 7.0, 3.5, 5.0, 6.0, 6.5, 6.0, 5.5, 8.5, 7.5, 4.0, 6.5, 5.5,
        2.5, 7.0, 7.5, 3.5, 5.5, 6.5, 5.5, 4.0, 6.0, 9.0, 5.5, 3.5, 6.0, 1.5, 6.5, 5.0, 3.5, 7.5,
        7.0, 4.0, 6.5, 5.5, 4.5, 6.5, 7.0, 4.5, 8.0, 7.5, 5.5, 5.0, 6.0, 3.5, 5.5, 5.0, 3.5, 6.0,
        6.0, 2.0, 3.5, 4.0, 3.0, 4.5, 4.5, 7.0, 3.0, 7.0,
    ];

    let inputs = DMatrix::<f32>::from_vec(
        x.len(),
        2,
        x.iter().cloned().chain(y.iter().cloned()).collect(),
    );

    let targets = DMatrix::<f32>::from_vec(z.len(), 1, z.clone());

    let learning_rate = 0.01;
    let epochs = 1000;
    nn.train(&inputs, &targets, learning_rate, epochs);

    let ax = 12.;
    let ay = 32.;
    let new_input = DVector::from_vec(vec![ax, ay]);
    let (_, prediction) = nn.forward(&new_input);

    println!("Predicted output for ({}, {}): {:?}", ax, ay, prediction);
}
