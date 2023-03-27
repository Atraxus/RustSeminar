fn fib_it(t: u32) -> u32 {
    if t <= 1 {return t};

    let mut data_vec = Vec::new();
    data_vec.push(0);
    data_vec.push(1);

    let mut i:usize = 2;
    while i <= t as usize {
        let x = data_vec[i-1] + data_vec[i-2];
        data_vec.push(x); 
        i += 1;
    }
    return data_vec[t as usize];
}

fn fib_rc(t: u32) -> u32 {
    if t <= 1 {return t};
    return fib_rc(t-1) + fib_rc(t-2);
}

fn gcd_it(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}

fn gcd_rc(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    else {
        return gcd_rc(b, a % b);
    }
}

fn swap(a: &mut i32, b: &mut i32) {
    let t = *a;
    *a = *b;
    *b = t;
}

fn main() {
    println!("{}", fib_it(7));
    println!("{}", fib_rc(7));

    println!("{}", gcd_it(14, 21));
    println!("{}", gcd_rc(14, 21));
    println!("Results should be 13, 13, 7, 7");

    
    let mut a = 1;
    let mut b = 2;
    swap(&mut a, &mut b);
    println!("a: {}, b: {}", a, b);
}
