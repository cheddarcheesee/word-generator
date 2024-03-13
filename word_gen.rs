use rand::Rng;
use std::io;
// crates for input/output and randomizations
fn main() {
  loop {
      println!("Enter a sentence containing a singular underscore or minus sign.");
    let mut sample_sentence = String::new();
    io::stdin().read_line(&mut sample_sentence).expect("Failed to read line");
    // the items in the words and random responses list are FULLY EDITABLE. they only change the OUTPUT. the ORIGINAL project has misc words added by friends. i removed them with more basic words.
    let words = vec!["apples", "oranges", "pears", "bananas", "grapes", "strawberries"];
    let responses = vec!["yes", "no", "maybe", "heck yeah", "heck nah", "i don't know", "ask again later", "i'm not sure", "go do something else"];
    // generating a random word & response (will change depending on whether you use an underscore or minus sign)
    let random_word = words[rand::thread_rng().gen_range(0..words.len())];
    let gen_response = responses[rand::thread_rng().gen_range(0..responses.len())];
    // checking if the input contains an underscore or minus sign
    for letters in sample_sentence.chars() {
      if letters == '_' {
        let result = str::replace(&sample_sentence, "_", &random_word);
        println!("{}", result)
      }
      else if letters == '-' {
        let result = str::replace(&sample_sentence, "-", &gen_response);
        println!("{}", result)
      }
      }  
  } 
} 
// end of code
