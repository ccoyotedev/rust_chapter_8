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
