//extern crate clap;
//use clap::{App, Arg};
extern crate rand_distr;
use rand_distr::{Normal, Uniform, Distribution};
extern crate rand;
use rand::seq::SliceRandom;

fn main() {
    let lr: f32 = 0.1; // controls value updating
    let k: u32 = 10; // the number of arms of the bandit
    let actions: Vec<u32> = (0..k).collect(); // the actions relating to the number of arms
    let exp_rate: f32 = 0.1; // the exploration rate of the trying new actions
    let ucb: bool = false;
    let c: f32 = 2.;
    let mut total_reward: f32 = 0.0;
    let mut avg_reward: Vec<f32> = vec![0.0; k as usize];
    let mut true_value: Vec<f32> = vec![0.0; k as usize];
    let mut values = vec![0.0; k as usize];
    let mut times: f32 = 0.;
    let mut action_times: Vec<f32> = vec![0.0; k as usize];

    let normal = Normal::new(0.0, 1.0).unwrap();
    let uniform = Uniform::new_inclusive(0.0, 1.0);

    for i in 0..k as usize {
        true_value[i] = normal.sample(&mut rand::thread_rng());
    }
}

fn choose_action(exp_rate: &f32, actions: &[u32], ucb: bool, 
    c: f32, times: &f32, action_times: &[f32], uniform: Uniform<f32>) {
    // sample a random uniform distribution and if it is below the exploration rate then 
    // choose an arbitrary action
    let explore_test = uniform.sample(&mut rand::thread_rng());
    let mut action: Option<u32> = None;
    if explore_test <= *exp_rate {
        action = Some(actions.choose(&mut rand::thread_rng()));
    }
}


