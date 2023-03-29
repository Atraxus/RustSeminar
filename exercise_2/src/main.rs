// fn max<'a>(args: &'a [&i32]) -> &'a i32 {
//     return args[0];
// }
fn max<'a>(args: &[&'a i32]) -> &'a i32 {
    return args[0];
}

fn main() {
    let b;
    {
        let a = [&1];
        b = max(&a);
    }
    println!("{}", b);
}