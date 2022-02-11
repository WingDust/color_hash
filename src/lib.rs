extern crate wasm_bindgen;

use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;

/// 函数式的 pipe
fn pipe3<A, B, C, D>(fn1: fn(A) -> B, fn2: fn(B) -> C, fn3: fn(C) -> D) -> impl Fn(A) -> D {
    move |x: A| fn3(fn2(fn1(x)))
}
/// - [Rust: Convert sha256 to hex ](https://stackoverflow.com/questions/67966855/rust-convert-sha256-to-hex)
/// - [How to print sha256 hash in Rust? (GenericArray)](https://stackoverflow.com/questions/66728572/how-to-print-sha256-hash-in-rust-genericarray)
fn sha256_to_int(s: String) -> i32 {
    let mut hasher = Sha256::new();
    hasher.update(s);
    let hash = hasher.finalize();
    let h = format!("{:x}", hash);
    i32::from_str_radix(&h[0..8], 16).unwrap()
}

/// - [Decimal number to hexadecimal string](https://stackoverflow.com/questions/25007328/decimal-number-to-hexadecimal-string)
fn rgb2hex(rgbarr: [u8; 3]) -> String {
    let mut hex = String::from("#");
    rgbarr.map(|v| {
        if v < 16 {
            hex.push('0');
        }
        hex.push_str(&format!("{:x}", v));
    });
    hex
}
fn hsl2rgb(hsl: [f64; 3]) -> [u8; 3] {
    let mut h = hsl[0];
    let s = hsl[1];
    let l = hsl[2];
    h /= 360.0;
    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;
    [h + 1.0 / 3.0, h, h - 1.0 / 3.0].map(|mut color| {
        if color < 0.0 {
            color += 1.0;
        }
        if color > 1.0 {
            color -= 1.0;
        }

        if color < 1.0 / 6.0 {
            color = p + (q - p) * 6.0 * color;
        } else if color < 0.5 {
            color = q;
        } else if color < 2.0 / 3.0 {
            color = p + (q - p) * 6.0 * (2.0 / 3.0 - color);
        } else {
            color = p;
        }
        // 因为在最后将转成 rgb 所以值会在 0-255，可以转成u8
        (color * 255.0).round() as u8
    })
}

/// - [ErrorE0277: the type `u32` cannot be indexed by  `u32`](https://stackoverflow.com/questions/65261859/errore0277-the-type-u32-cannot-be-indexed-by-u32)
fn hsl(s1: String) -> [f64; 3] {
    let h: f64;
    let s: f64;
    let l: f64;

    let original = [0.35, 0.5, 0.65];
    // let s = [0.35, 0.5, 0.65];
    let hash = sha256_to_int(s1);
    // let hueResolution = 727;
    h = f64::from(hash % 359);
    let mut fhash = (f64::from(hash) / 360.0).ceil();
    // hash /= 360;
    // let i = fhash % 3;
    s = original[(fhash % 3.0) as usize];
    fhash = (fhash / 3.0).ceil();
    l = original[(fhash % 3.0) as usize];
    [h, s, l]
}

#[wasm_bindgen]
pub fn color_hash_hex(s: String) -> String {
    let exec = pipe3(hsl, hsl2rgb, rgb2hex);
    exec(s)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
