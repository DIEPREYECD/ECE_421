mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-is-prime!");
}

#[wasm_bindgen]
pub fn check_prime(s: &JsValue) -> u32 {
    let input: String = s.as_string().unwrap();
    match input.parse::<u32>() {
        Ok(num) => {
            if is_prime(num) == 1 {
                alert("Input is Prime");
                return 1;
            } else {
                alert("Input is NOT Prime");
                return 0;
            }
        }
        Err(_) => {
            alert(&format!("Couldn't parse {}", input));
            return 0;
        }
    }
}

fn is_prime(n: u32) -> u32 {
    // add your code to check prime here
    if n <= 1 {
        return 0;
    }
    if n <= 3 {
        return 1;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return 0;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return 0;
        }
        i += 6;
    }
    1
}
