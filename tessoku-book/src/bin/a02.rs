use std::io;

fn main() {
    // input
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let vec: Vec<&str> = input.split_whitespace().collect();
    let n_num = vec[0].trim().parse().unwrap();
    let x = vec[1].trim().parse().unwrap_or(0);

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let vec: Vec<&str> = input.split_whitespace().collect();
    let mut vec1 = vec![];
    for i in 0..n_num{
        vec1.push(vec[i].trim().parse().unwrap_or(0));
    }

    // 処理
    let mut flag= "No";
    for _i in 0..n_num{
        if vec1[_i] == x {
            flag = "Yes";
        }
    }

    // output
    println!("{}", flag);
}
