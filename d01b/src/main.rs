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
    let right = right
        .into_iter()
        .map(|(_, value)| value);
    
    let res = left
        .iter()
        .map( |a| right.clone().filter(|x| x == a).count() as i32 * *a)
        .sum::<i32>();
    println!("{}", res)
}
