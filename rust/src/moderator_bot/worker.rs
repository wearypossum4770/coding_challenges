use toml::Value;
use serde_derive::Deserialize;
use std::fs;
#[derive(Deserialize)]
struct Config {
    ip: String,
    port: Option<u16>,
    keys: Keys,
}
use std::fs;
fn load_config(filename:&str){
    let contents = fs::read_to_string(filename);
    println!("{:?}", contents);
}
fn required_env(variable){

}
let moderator_scores  = ("toxic", "severe_toxic", "obscence", "insult", "identity_hate")

pub fn warm(){

}

pub  fn predict(){}