use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let d1: Vec<f32> = (0..64).map(|_| rng.gen_range(1..=64) as f32).collect();
    let d2: Vec<f32> = (0..64).map(|_| rng.gen_range(1..=64) as f32).collect();
    let d3: Vec<f32> = (0..64).map(|_| rng.gen_range(1..=64) as f32).collect();
    let d4: Vec<f32> = (0..64).map(|_| rng.gen_range(1..=64) as f32).collect();
    let d5: Vec<f32> = (0..64).map(|_| rng.gen_range(1..=64) as f32).collect();
    let d6: Vec<f32> = (0..64).map(|_| rng.gen_range(1..=64) as f32).collect();
    let d7: Vec<f32> = (0..64).map(|_| rng.gen_range(1..=64) as f32).collect();
    let d8: Vec<f32> = (0..64).map(|_| rng.gen_range(1..=64) as f32).collect();

    let da: Vec<f32> = d1
        .iter()
        .zip(&d2)
        .zip(&d3)
        .zip(&d4)
        .zip(&d5)
        .zip(&d6)
        .zip(&d7)
        .zip(&d8)
        .map(|(((((((a, b), c), d), e), f), g), h)| (a + b + c + d + e + f + g + h) / 8.0)
        .collect();

    println!("let d1:Vec<f32> =vec!{:?};", d1);
    println!("let d2:Vec<f32> =vec!{:?};", d2);
    println!("let d3:Vec<f32> =vec!{:?};", d3);
    println!("let d4:Vec<f32> =vec!{:?};", d4);
    println!("let d5:Vec<f32> =vec!{:?};", d5);
    println!("let d6:Vec<f32> =vec!{:?};", d6);
    println!("let d7:Vec<f32> =vec!{:?};", d7);
    println!("let d8:Vec<f32> =vec!{:?};", d8);
    println!("let da:Vec<f32> =vec!{:?};", da);
}
