# wordy
A Multi-threaded word frequency counter.

```
-------------------------------------------------------------
Word Frequency: 1
compare i day art lovely temperate rough winds do 
shake darling buds may lease hath all short date 
hot eye heaven shines often is gold dimm'd complexion 
every from declines by chance nature's changing course untrimm'd 
but thy summer not fade lose possession that ow'st 
death brag wander'st shade when lines time grow'st as 
men breathe eyes see lives gives life 
-------------------------------------------------------------
Word Frequency: 2
thee a summer's more the too sometime his or 
eternal nor in so long can this 
-------------------------------------------------------------
Word Frequency: 3
shall to of fair 
-------------------------------------------------------------
Word Frequency: 4
thou 
-------------------------------------------------------------
Word Frequency: 5
and 
```

## Genesis
Concurrency is a topic I've always seemed to brush by. I have encountered it with java during University, 
but I've never dug into it myself. This small project is significant because it is my first advent / self 
exploration into concurrent and parallel programming using Rust. 

Disclaimer. The intent of this project is educational, not to count words the fastest.

At a high level, wordy uses a **multi-producer single-consumer** (mpsc) channel to send information from any number
of transmitters to a receiver, which ultimately processes the result. 

Important moments during this program were: 
* Creating an **Atomic Reference Counter** (Arc) to immutably share text among threads
* Spawning threads 
* Joining threads together to wait for program to finish
* Deciding NOT to use Mutexes

Things to improve on: 
* Integrate Tests into development process
* Compare Performance out of curiosity
