use std::io::{self, Write};
//use rand::seq::SliceRandom;
//use std::collections::HashMap;
//TODO: bucketing approach
fn main() 
{   let mut win_status: u8;
    let mut buffer = String::from("");
    let mut difficulty: u8;
    loop 
    {
       // difficulty = get_difficulty();
        win_status = game_loop();

        if win_status == 1
        {
            println!("You Win!");
        }
        else
        {
            println!("You Lost!");
        }

        println!("Play again? [Y/n]");
        io::stdin().read_line(&mut buffer).expect("input failure");
        buffer = buffer.trim().to_string().to_uppercase();
        if buffer != "Y"
        {
            println!("Bye!");
            break;
        }
    }
}

fn get_difficulty() -> u8
{
   let mut buffer = String::from("");
   let word_length: u8;      
   print!("Enter difficulty: ");
   match io::stdout().flush()
   {
        Ok(()) => {}
        Err(err) => eprintln!("Error flushing stdout: {}", err),
   }
    io::stdin().read_line(&mut buffer).expect("input failure");
    buffer = buffer.trim().to_string();
    word_length = string_to_u8(&buffer);
    return word_length;
}
fn string_to_u8(input: &String) -> u8
{
    match input.parse::<u8>()
    {
        Ok(u8_variable) =>
        {
            return u8_variable;
        }
        Err(_) =>
        {
            println!("non number passed to string_to_u8()");
            return 255;
        }
    }
}
// 1 = success, 2 = fail
fn game_loop() -> u8
{
    let mut correct_letters = String::from("");
    let answer = String::from("word");
    let mut guess = String::new();
    let mut hangman = String::from("____");
    let mut number_of_tries: u8 = 3;
    let exit_condition: u8;
    loop  
    {
        println!("Enter your guess: ");
        io::stdin().read_line(&mut guess).expect("input failure");
        guess = guess.trim().to_string();
        if guess == answer
        {
            exit_condition = 1;
            break;
        }
            
        check_letters(&guess, &answer, &mut correct_letters, &mut hangman);
        guess.clear();
        number_of_tries -= 1;
        println!("correct letters: {}",correct_letters);
        println!("hangman: {} \ntries left: {}", hangman, number_of_tries);

        if number_of_tries == 0
        {
            exit_condition = 2;
            break;
        }
    }

    return exit_condition;

}

fn check_letters(guess: &String, answer: &String, correct_letters: &mut String, hangman: &mut String)  
{
    for character in guess.chars()
    {
        if answer.contains(character) && !correct_letters.contains(character) 
        {
            correct_letters.push(character);
        }
    }

    for (index, (guess_char, answer_char)) in guess.chars().zip(answer.chars()).enumerate()
    {
        if guess_char == answer_char
        {
            hangman.remove(index);
            hangman.insert(index, guess_char);
        }
    }
}
