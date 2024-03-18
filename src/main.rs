// pub mod features;
mod features;
use features::{
    feature_nested::feature_one_one::FeatureOneOneStruct, feature_one::FeatureOneStruct,
};

fn main() {
    println!("Focus Rust!");
    let fos = FeatureOneStruct {};
    let foos = FeatureOneOneStruct {};
}
