mod util;

use std::collections::{BTreeMap, HashMap};
use std::ops::Rem;
use std::sync::{Arc, mpsc};
use std::thread;
use util::*;

/// Counts the frequency of words in a text document using a channel. A new thread is spawned
/// any time a new undiscovered word is considered. That thread then looks for that specific word
/// throughout the rest of the document. Once the thread has considered every word, it's
/// transmitter sends a message containing the target word and frequency. This message is
/// received by a receiver, which collects the data into a hash map.
fn count_word_frequencies(text: &str) {
    let text: Arc<Vec<String>> = setup_text(text);
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
                tx.send((target, wc));
            });
            handles.push(handle)
        }
        // update world index
        prgm_indx += 1;

        # [cfg(debug_assertions)]
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
    println!("{}", word_freq_map.len());
}

/// "Standard" word frequency word counter. Looks at every word in a text document. If the word
/// has not yet been considered, add it to the hash map. Otherwise, increment the word's value
/// in the hashmap.
fn hash_freq_word_count(text: &str) {
    let text: Arc<Vec<String>> = setup_text(text);

    # [cfg(debug_assertions)]
    let mut prgm_indx = 0;

    let mut hash = HashMap::<String, i32>::new();

    // Consider each word in the text document
    for word in text.iter() {
        if hash.contains_key(word) {
            *hash.get_mut(word).unwrap() += 1;
        } else {
            hash.insert(word.clone(), 0);
        }

        # [cfg(debug_assertions)]
            {
                prgm_indx += 1;
                if prgm_indx % 1000 == 0 {
                    println!("{prgm_indx}");
                }
            }

    }
    println!("{:?}", hash);
    println!("{}", hash.len());
}


// todo: compare performance beetween conventional counter and multithreading counter
fn main() {
    //count_word_frequencies("odyssey");
    hash_freq_word_count("odyssey")
}