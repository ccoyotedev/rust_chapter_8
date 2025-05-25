extern crate std;

pub mod averages {
    pub fn get_median(arr: &Vec<u32>) -> Option<u32> {
        let mut arr = arr.clone();
        let length = arr.len();
        match length {
            0 => None,
            1 => Some(arr[0]),
            _ => get_median_value(&mut arr, length),
        }
    }

    fn get_median_value(arr: &mut Vec<u32>, length: usize) -> Option<u32> {
        arr.sort();

        if length % 2 == 0 {
            let length = length / 2;

            let x = &arr[..length][length - 1];
            let y = &arr[length..][0];

            Some((x + y) / 2)
        } else {
            arr.get(length / 2).copied()
        }
    }

    pub fn get_mode(arr: &Vec<u32>) -> Option<u32> {
        let mut map = std::collections::HashMap::new();

        for number in arr {
            let count = map.entry(number).or_insert(0);
            *count += 1;
        }

        let max_value = map.iter().max_by_key(|entry| entry.1).unwrap();
        return Some(**max_value.0);
    }
}

pub mod pig_latin {
    pub fn translate_string(str: String) -> String {
        let words: Vec<&str> = str.split_whitespace().collect();

        let mut translated_string = String::new();
        for word in words {
            let translated_word = translate_word(word);
            translated_string = format!("{translated_string}{translated_word} ");
        }

        return translated_string;
    }

    fn translate_word(word: &str) -> String {
        let mut chars = word.chars();
        let first_char = chars.nth(0).unwrap();

        if is_vowel(&first_char) {
            let translated_word = format!("{}-hey", word);
            return translated_word;
        } else {
            let start = &word[1..];
            let end = format!("{}ay", first_char);
            let translated_word = format!("{start}-{end}");
            return translated_word;
        }
    }

    fn is_vowel(char: &char) -> bool {
        let vowels = ['a', 'e', 'i', 'u', 'o'];

        vowels.contains(char)
    }
}

pub mod employee_database {
    pub struct Employee {
        name: String,
        department: String,
    }
    pub enum Prompt {
        Add(Employee),
        Remove(Employee),
        Get(Option<String>),
    }

    pub fn build_employee(name: String, department: String) -> Employee {
        Employee { name, department }
    }

    pub fn handle_database_prompt(prompt: Prompt) {
        match prompt {
            Prompt::Add(employee) => add_to_database(employee),
            Prompt::Remove(employee) => remove_from_database(employee),
            Prompt::Get(department) => retrieve_from_database(department),
        }
    }

    fn add_to_database(employee: Employee) {}

    fn remove_from_database(employee: Employee) {}

    fn retrieve_from_database(department: Option<String>) {}
}
