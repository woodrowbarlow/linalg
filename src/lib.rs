fn mult(r : Vec<i32>, k : i32) -> Vec<i32> {
    let mut r_prime = Vec::new();
    for i in 1..r.len() {
        r_prime.push(r[i] * k);
    }
    return r_prime;
}

fn add(r1 : Vec<i32>, r2 : Vec<i32>) -> Vec<i32> {
    let mut r_prime = Vec::new();
    if r1.len() != r2.len() {
        panic!("incompatible row width");
    }
    for i in 1..r1.len() {
        r_prime.push(r1[i] + r2[i]);
    }
    return r_prime;
}

fn sub(r1 : Vec<i32>, r2 : Vec<i32>) -> Vec<i32> {
    let mut r_prime = Vec::new();
    if r1.len() != r2.len() {
        panic!("incompatible row width");
    }
    for i in 1..r1.len() {
        r_prime.push(r1[i] - r2[i]);
    }
    return r_prime;
}

fn add_with_coeff(r1 : Vec<i32>, r2 : Vec<i32>, k : i32) -> Vec<i32> {
    let mut r_prime = Vec::new();
    if r1.len() != r2.len() {
        panic!("incompatible row width");
    }
    for i in 1..r1.len() {
        r_prime.push(r1[i] + k * r2[i]);
    }
    return r_prime;
}

fn sub_with_coeff(r1 : Vec<i32>, r2 : Vec<i32>, k : i32) -> Vec<i32> {
    let mut r_prime = Vec::new();
    if r1.len() != r2.len() {
        panic!("incompatible row width");
    }
    for i in 1..r1.len() {
        r_prime.push(r1[i] - k * r2[i]);
    }
    return r_prime;
}
