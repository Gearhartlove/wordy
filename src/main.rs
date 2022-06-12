use std::collections::{BTreeMap, HashMap};
use std::ops::Rem;
use std::sync::{Arc, mpsc};
use std::thread;

const PUNC: &str = ".,?\"\n\t:;'{}()";

fn count_word_frequencies() {
    let text: Arc<Vec<String>> =
        Arc::new(
            get_text()
                .split(|c| c == ' ' || c == '\n')
                .filter(|s| s.len() != 0)
                .map(|s: &str|
                    s
                        .to_lowercase()
                        .rmv_punc()
                )
                .collect()
        );

    // create channel
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    // no thread will spawn before this index; records what words have been considered
    let mut prgm_indx = 0;
    // all words seen in the text
    let mut words: Vec<String> = vec!();

    // while the program has not considered every word, continue
    while prgm_indx != text.len() {
        let next_word: String = text.get(prgm_indx).unwrap().clone();
        if !words.contains(&next_word) {
            words.push(next_word.clone());

            // create variables for thread
            let text = Arc::clone(&text); // text reference
            let mut ti = prgm_indx.clone(); // thread index
            let tx = tx.clone();
            let mut wc = 0; // word counter

            let handle = thread::spawn(move || {
                let target = next_word;
                // Note: Difference between iter and into_iter;
                // > into_iter consumes every one of the values and renders the vector unusable
                for word in text.iter().skip(ti) {
                    if *word == target {
                        wc += 1
                    }
                    ti += 1
                }
                // todo: change to return a tuple
                // tx.send(format!("thread({target}): {wc}"))
                tx.send((target, wc));
            });
            handles.push(handle)
        }
        // update world index
        prgm_indx += 1;

        if prgm_indx % 1000 == 0 {
            println!("{prgm_indx}");
        }
    }
    // after every word is considered, exit the loop

    // wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    drop(tx);

    // todo print in nice format
    let mut word_freq_map = BTreeMap::<String, i32>::new();
    for recieved in rx {
        word_freq_map.insert(recieved.0, recieved.1);
    }

    println!("{:?}", word_freq_map);
}

fn start_punc(word: &String) -> bool {
    let first = word.chars().nth(0).unwrap();
    if PUNC.contains(first) {
        return true;
    }
    return false;
}

fn end_punc(word: &String) -> bool {
    let len = word.len();
    let last = word.chars().nth(len - 1).unwrap();
    if PUNC.contains(last) {
        return true;
    }
    return false;
}

trait RemovePuctuation {
    fn rmv_punc(&mut self) -> String;
}

impl RemovePuctuation for String {
    // Removes punctuation from the start and end of the word
    fn rmv_punc(&mut self) -> String {
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

fn remove_numbers() {
    unimplemented!()
}

fn get_text() -> String {
    use std::fs;

    let file_path = "assets/odyssey.txt";
    fs::read_to_string(file_path)
        .expect(format!("The file {} does not exist", file_path).as_str())
}

fn main() {
    count_word_frequencies()
}