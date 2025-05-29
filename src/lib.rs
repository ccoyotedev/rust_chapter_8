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
    use std::collections::HashMap;

    pub struct Employee {
        name: String,
        department: String,
    }

    pub enum Prompt {
        Add(Employee),
        Get(Option<String>),
    }

    pub fn build_employee(name: String, department: String) -> Employee {
        Employee { name, department }
    }

    pub struct DB {
        dictionary: HashMap<String, Vec<String>>,
    }

    impl DB {
        pub fn handle_database_prompt(&mut self, prompt: Prompt) {
            match prompt {
                Prompt::Add(employee) => self.add_to_database(employee),
                Prompt::Get(department) => self.retrieve_from_database(department),
            };
        }
        fn add_to_database(&mut self, employee: Employee) {
            let employees = self
                .dictionary
                .entry(employee.department)
                .or_insert(Vec::new());
            employees.push(employee.name);
        }
        fn retrieve_from_database(&mut self, department: Option<String>) {
            println!("================================");
            match department {
                Some(dep) => {
                    let employees = self.dictionary.get_mut(&dep);
                    match employees {
                        Some(emps) => {
                            println!("{}", dep.to_uppercase());
                            println!("--------------------------------");
                            emps.sort();
                            for employee in emps {
                                println!("{}", employee)
                            }
                        }
                        None => println!("Department does not exist"),
                    }
                }
                None => println!("{:?}", self.dictionary),
            }
            println!("================================");
        }
    }

    pub fn build_db() -> DB {
        DB {
            dictionary: HashMap::new(),
        }
    }
}
