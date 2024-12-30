use std::usize;

fn test_word(word: &str, index_data: usize, data: &str, range: usize) -> bool {
    for (index_word, letter) in word.chars().enumerate() {
        let offset = index_data + index_word * range;
        if data.chars().nth(offset) != Some(letter) {
            return false;
        }
    }
    true
}

fn main() {
    let data = include_str!("../input.txt");
    let mut res = 0;
    let row_size = data.lines().next().unwrap().len() + 1;
    for (index_data, data_point) in data.chars().enumerate() {
        if data_point == 'M' {
            if test_word("MAS", index_data, data, row_size + 1)
                && test_word("SAM", index_data + 2, data, row_size - 1)
                || test_word("MAS", index_data, data, row_size + 1)
                    && test_word("MAS", index_data + 2, data, row_size - 1)
            {
                res += 1;
            }
        } else if data_point == 'S' {
            if test_word("SAM", index_data, data, row_size + 1)
                && test_word("MAS", index_data + 2, data, row_size - 1)
                || test_word("SAM", index_data, data, row_size + 1)
                    && test_word("SAM", index_data + 2, data, row_size - 1)
            {
                res += 1
            }
        }
    }
    println!("{:?}", res);
}
