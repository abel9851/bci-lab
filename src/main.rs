use rand::prelude::*;
use rand_distr::{Distribution, Normal};

fn main() {

    // 기본 설정
    let fs: f32 = 250.0; // sampling
    let duration: f32 = 10.0; // seconds

    let n_samples = (fs * duration) as usize;
    let mut rng = thread_rng();
    let noise_dist = Normal::new(0.0_f32, 5.0_f32).unwrap();

    let alpha_freq: f32 = 10.0; // hz
    let alpha_amp: f32 = 20.0;
    let alpha_start: f32 = 3.0;
    let alpha_end: f32 = 7.0;
    
    let mut eeg: Vec<f32> = Vec::with_capacity(n_samples); // n_samples 수만큼 메모리에 n개만큼 공간 할당



    
}
