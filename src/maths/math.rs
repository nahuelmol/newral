use rand::Rng;

#[allow(dead_code)]
pub fn procedure(x:i32) -> i32 {
    let result = x*2;
    result
}

pub fn sigmoid(entries:Vec<f32>) -> f32 {
    let mut polinom:f32 = 0.0;
    for x in entries.iter() {
        let mut rng = rand::thread_rng();
        let w:f32 = rng.gen_range(1..=100) as f32;
        polinom = polinom + (w * x);
    }

    return polinom;
}
