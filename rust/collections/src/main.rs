use std::collections::HashMap;

fn main() {
    let list_of_numbers = vec![32,55,2,2,93,54,112,3234,17,32,99,2,112,22223];

    let mut median_copy = list_of_numbers.clone();
    let median = get_median(&mut median_copy);
    println!("Median of {:?} is {median}", list_of_numbers);

    let mode = get_mode(&list_of_numbers);
    println!("Mode of {:?} is {mode}", list_of_numbers);

    let apple = "apple";
    let apple_in_pig_latin = get_word_in_pig_latin(apple);
    println!("{apple} in pig latin is {apple_in_pig_latin}");

    let giraffe = "giraffe";
    let giraffe_in_pig_latin = get_word_in_pig_latin(giraffe);
    println!("{giraffe} in pig latin is {giraffe_in_pig_latin}");
}

fn get_word_in_pig_latin(word: &str) -> String{
    let vowels = ['a','e','i','o','u'];

    let Some(first_letter) = word.chars().nth(0) else {panic!("No letters")};
    let word = String::from(word);
    
    match vowels.contains(&first_letter) {
        false => {
            let remaining_word: &str = &word[first_letter.len_utf8()..];
            format!("{remaining_word}-{first_letter}ay")
        }
        true => {
            format!("{word}-hay")
        }
    }

}

fn get_median(numbers: &mut Vec<i32>) -> i32{
    numbers.sort();
    let length = numbers.len();
    match &length % 2 == 0 {
       true => {
           numbers[&length/2 - 1] + numbers[&length/2] / 2
       }
       false => {
           numbers[(&length - 1)/2]
       }
    } 
}

fn get_mode(numbers: &Vec<i32>) -> i32 {
    let mut count_hash: HashMap<i32, i32> = HashMap::new();
    for i in numbers {
        let count = count_hash.entry(*i).or_insert(0);
        *count += 1;
    }

    match count_hash.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|(k,_v)|k) {
        None => 0,
        Some(i) => *i
    }
}
