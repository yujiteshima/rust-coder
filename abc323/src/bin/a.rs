use proconio::{input, marker::Chars};

fn main() {
    input!{
        s:Chars,
    }
    let mut ans = "Yes";
    for i in 2..=16 {
        if i % 2 == 0 && s[i - 1] == '1' {
            ans = "No";
        }
    }
    println!("{}", ans);
}
