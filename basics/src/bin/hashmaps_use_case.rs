//! This program will go through a sentence and generate a 
//! HashMap where each word will be mapped to its count in the sentence
//! Basically a frequency hash map

use std::collections::HashMap;

fn main(){
    let sentence = "Hello beautiful world it's so happy and simple and deppresed and sad here lollll";
    
    let mut hm = HashMap::new();

    for word in sentence.split_whitespace() {
        let count = hm.entry(word).or_insert(0);
        *count += 1;
    };

}