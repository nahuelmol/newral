use rand::Rng;
use std::f64;

#[allow(dead_code)]
pub fn procedure(x:i32) -> i32 {
    let result = x*2;
    result
}

pub fn random() -> f32 {
    let mut rng = rand::thread_rng();
    let w:f32 = rng.gen();
    return w;
}

pub fn sigmoid(x:f32) -> f32 {
    let output:f32 = 1.0/(1.0 + (-x).exp()) as f32;
    return output;
}

pub fn tanh(x:f32) -> f32 {
    let down:f32 = (x).exp() + (-x).exp();
    let up:f32 = (x).exp() - (-x).exp();
    let response:f32 = up/down;
    return response;
}

//pub fn relu(x:f32) -> {}

#[allow(dead_code)]
pub fn back_propagation(){}
