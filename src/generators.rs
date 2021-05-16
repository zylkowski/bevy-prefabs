use serde::{Serialize,Deserialize};
use rand::seq::SliceRandom;

#[typetag::serde(tag = "_generator")]
pub trait Generator{
    fn generate<T>(&self) -> T;
}

// #[derive(Debug,Serialize,Deserialize)]
// pub enum BaseGenerators<T: Clone> {
//     Vector { values: Vec<T> },
//     Fixed { value: T },
// }

// impl<'de,T: Clone> Generator<'de,T> for BaseGenerators<T>{
//     fn generate(&self) -> T{
//         match self {
//             BaseGenerators::Vector { ref values } => values.choose(&mut rand::thread_rng()).unwrap().clone(),
//             BaseGenerators::Fixed { ref value } => value.clone(),
//         }
//     }
// }


// impl<T: Clone> Generator<T> {
//     pub fn generate(&self) -> T {
//         match self {
//             Generator::Vector { ref values } => values.choose(&mut rand::thread_rng()).unwrap().clone(),
//             Generator::Fixed { ref value } => value.clone(),
//         }
//     }
// }