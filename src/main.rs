use std::sync::{Arc, mpsc};
use std::thread;

fn count_word_frequencies() {
    let text: Arc<Vec<String>> =
        Arc::new(
            get_text()
                .split(" ")
                .map(|s| {
                    s.rmv_punc();
                    s.to_lowercase();
                })
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
                tx.send(format!("thread({target}): {wc}"))
            });
            handles.push(handle)
        }
        // update world index
        prgm_indx += 1
    }
    // after every word is considered, exit the loop

    // wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // todo change to returning a hash map to main
    for recieved in rx {
        println!("{recieved}")
    }
}

// todo: add error for file not found
// todo: add default texts for practice
fn get_text() -> String {
    use std::fs;

    let file_path = "assets/default.txt";
    fs::read_to_string(file_path)
        .expect(format!("The file {} does not exist", file_path).as_str())
}

// Removes punctuation from the start and end of the word
fn rmv_punc(word: &str) -> String {
    todo!()
}

fn remove_numbers() {
    unimplemented!()
}


fn main() {
    count_word_frequencies()
}