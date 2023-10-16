use proconio::input;
fn main() {
    input!{
        n: i64
    }
    print!("{}", shinsu(n, 2));
}

fn shinsu(mut x:i64, b: i64) -> String {
    let mut ans = String::new();
    for _ in (0..10).rev() {
        let value = x % 2;
        ans.push_str(&value.to_string());
        x /= b;
    }
    ans.chars().rev().collect()
}