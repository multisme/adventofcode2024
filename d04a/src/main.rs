
use std::usize;


fn test_word(word: &str, index_data: usize, data: &str, range: usize) -> i32 {
    for (index_word, letter) in word.chars().enumerate() {
        let offset = index_data + index_word * range;
        if data.chars().nth(offset) != Some(letter) {
            return 0;
        }
    }
    1
}

fn main() {
    let data = include_str!("../input.txt");
    let mut res = 0;
    let row_size = data.lines().next().unwrap().len() + 1;
    for (index_data, data_point) in data.chars().enumerate() {
        if data_point == 'X' {
            'range: for range in [1, row_size - 1, row_size, row_size + 1] {
                res += test_word("XMAS", index_data, data, range);
            }
        } else if data_point == 'S' {
            'range: for range in [1, row_size - 1, row_size, row_size + 1] {
                res += test_word("SAMX", index_data, data, range);
            }
        }
    }
    println!("{:?}", res);
}
