use proconio::input;

fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [usize; N],
        list: [[usize; 2]; Q],
    }

    let mut sum_list = vec![];
    sum_list.push(0);
    for i in A {
        sum_list.push(sum_list.last().unwrap() + i);
    }
    for i in 0..Q {
        println!("{}", sum_list[list[i][1]] - sum_list[list[i][0] - 1])
    }
}
