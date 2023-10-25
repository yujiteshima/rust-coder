use proconio::{input, marker::Chars};
fn main() {
    input!{
        n: usize,
        s: Chars,
    }
    let mut ans = 999;
    for i in 0..n-2 {
        if s[i] == 'A' {
            if s[i+1] == 'B' {
                if s[i + 2] == 'C' {
                    ans = i + 1;
                    break;
                }
            }
        }
    }
    if ans != 999 { println!("{}", ans); }
    else { println!("{}", "-1"); }
}
