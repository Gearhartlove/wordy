use std::collections::{BTreeMap, HashMap};
use std::ops::Rem;
use std::sync::{Arc, mpsc};
use std::thread;

pub const PUNC: &str = ".,?\"\n\t:;'{}()-";

pub fn get_text(text: &str) -> String {
    use std::fs;

    let file_path = format!("assets/{}.txt", text.clone());
    fs::read_to_string(file_path)
        .expect(format!("The file does not exist").as_str())
}

pub fn setup_text(text: &str) -> Arc<Vec<String>> {
    let text: Arc<Vec<String>> =
        Arc::new(
            get_text(text)
                .split(|c| c == ' ' || c == '\n')
                .filter(|s| s.len() != 0)
                .map(|s: &str|
                    s
                        .to_lowercase()
                        .rmv_punc()
                )
                .filter(|s| s.len() != 0) // removes any remaining "", couldo: improve to just one call
                .collect()
        );
    return text;
}

pub fn start_punc(word: &String) -> bool {
    let first = word.chars().nth(0).unwrap();
    if PUNC.contains(first) {
        return true;
    }
    return false;
}

pub fn end_punc(word: &String) -> bool {
    let len = word.len();
    let last = word.chars().nth(len - 1).unwrap();
    if PUNC.contains(last) {
        return true;
    }
    return false;
}

pub trait RemovePuctuation {
    fn rmv_punc(&mut self) -> String;
}

impl RemovePuctuation for String {
    // Removes punctuation from the start and end of the word
    fn rmv_punc(&mut self) -> String {
        // if the word consisted of only punctuation
        if self.len() == 0 {
            return "".to_string();
        }

        // if the word starts or ends with punctuation, cut it off and repeat this process
        if start_punc(self) {
            let rmv = self.remove(0);

            #[cfg(debug_assertions)]
            println!("removed: {rmv} from:{self}");

            return self.rmv_punc();
        } else if end_punc(self) {
            let last = self.len() - 1;
            let rmv = self.remove(last);

            #[cfg(debug_assertions)]
            println!("removed: {rmv} from:{self}");

            return self.rmv_punc();
        }
        return self.to_string();
    }
}

pub fn pretty_print(map: HashMap<i32, Vec<String>>) {
    let mut keys: Vec<i32> = vec![];

    for key in map.keys() {
        keys.push(key.clone())
    }

    keys.sort();

    for key in keys.iter() {
        println!("\n-------------------------------------------------------------");
        println!("Word Frequency: {}", key);
        let mut count = 0;
        let words = map.get(key).unwrap();

        for word in words.iter() {
            count += 1;
            if count % 9 == 0 {
                println!("{} ", word)
            } else {
                print!("{} ", word)
            }
        }
    }
}