use proconio::input;
fn main() {
    input!{
        n: usize,
        a: [usize; n]
    }
    let a0 = a[0];
    let mut flg = "Yes";
    for i in 1..n {
        if a0 != a[i] { flg = "No"};
    }
    println!("{}", flg);
}