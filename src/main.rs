use ndarray::Array2;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let training_inputs: Array2<f32> =
        Array2::from_shape_vec((8, 8), (0..64).map(|_| rng.gen_range(0..64) as f32).collect())
            .unwrap();

    println!("{:?}", training_inputs);
}
