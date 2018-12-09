extern crate cfg_if;
extern crate wasm_bindgen;
// extern crate rand;

mod utils;

// use rand::Rng;
use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use std::cmp::Ordering;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub enum Result {
    Less,
    Greater,
    Equal,
}

#[wasm_bindgen]
pub struct Guessing {
    number: u32,
}

#[wasm_bindgen]
impl Guessing {
    // TODO: issue rust-random/rand/issues/616
    #[wasm_bindgen(constructor)]
    pub fn new(number: u32) -> Guessing {
        Guessing {
            number: number,
        }
    }

    pub fn check(&mut self, input: u32) -> Result{
        match self.number.cmp(&input) {
            Ordering::Less    => return Result::Less,
            Ordering::Greater => return Result::Greater,
            Ordering::Equal   => return Result::Equal,
        }
    }
}
