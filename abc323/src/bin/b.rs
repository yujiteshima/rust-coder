use proconio::input;
fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    let mut cnt: Vec<(usize, usize)> = vec![(0, 0); n];

    for i in 0..n {
        cnt[i] = (i + 1, s[i].matches("o").count());
    }

    cnt.sort_by(|a, b| (b.1).cmp(&a.1));
    for i in cnt {
        println!("{}", i.0);
    }
}
