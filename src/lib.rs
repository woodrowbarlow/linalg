//! LINALGESIC
//! A rust library to make linear algebra painless.
//!
//! Author: Woodrow Barlow
//! License: MIT

/// Multiply a row by a scalar.
fn mult(r : Vec<f32>, k : f32) -> Vec<f32> {
    let mut r_prime = Vec::new();
    for i in 1..r.len() {
        r_prime.push(r[i] * k);
    }
    return r_prime;
}

/// Divide a row by a scalar.
fn div(r : Vec<f32>, k : f32) -> Vec<f32> {
    let mut r_prime = Vec::new();
    for i in 1..r.len() {
        r_prime.push(r[i] / k);
    }
    return r_prime;
}

/// Add the second row to the first row.
fn add(r1 : Vec<f32>, r2 : Vec<f32>) -> Vec<f32> {
    let mut r_prime = Vec::new();
    if r1.len() != r2.len() {
        panic!("incompatible row width");
    }
    for i in 1..r1.len() {
        r_prime.push(r1[i] + r2[i]);
    }
    return r_prime;
}

/// Subtract the second row from the first row.
fn sub(r1 : Vec<f32>, r2 : Vec<f32>) -> Vec<f32> {
    let mut r_prime = Vec::new();
    if r1.len() != r2.len() {
        panic!("incompatible row width");
    }
    for i in 1..r1.len() {
        r_prime.push(r1[i] - r2[i]);
    }
    return r_prime;
}

/// Multiply the second row by a scalar and add the result to the first row.
fn add_with_coeff(r1 : Vec<f32>, r2 : Vec<f32>, k : f32) -> Vec<f32> {
    let mut r_prime = Vec::new();
    if r1.len() != r2.len() {
        panic!("incompatible row width");
    }
    for i in 1..r1.len() {
        r_prime.push(r1[i] + k * r2[i]);
    }
    return r_prime;
}

/// Multiply the second row by a scalaer and subtract the result from the first row.
fn sub_with_coeff(r1 : Vec<f32>, r2 : Vec<f32>, k : f32) -> Vec<f32> {
    let mut r_prime = Vec::new();
    if r1.len() != r2.len() {
        panic!("incompatible row width");
    }
    for i in 1..r1.len() {
        r_prime.push(r1[i] - k * r2[i]);
    }
    return r_prime;
}
