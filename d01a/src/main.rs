fn main() {
    let (left, right): (Vec<_>, Vec<_>) = include_str!("../input.txt")
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .enumerate()
        .partition(|(index, _)| index % 2 == 0);
    let mut left = left
        .into_iter()
        .map(|(_, value)| value)
        .collect::<Vec<i32>>();
    let mut right = right
        .into_iter()
        .map(|(_, value)| value)
        .collect::<Vec<i32>>();
    left.sort();
    right.sort();
    let res = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();
    println!("{}", res)
}
