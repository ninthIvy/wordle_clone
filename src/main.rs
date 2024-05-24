use std::io::{self};

fn main() 
{
    let mut correct_letters = String::from("");
    let answer = String::from("word");
    let mut guess = String::new();
    let mut hangman = String::from("____");
    loop 
    {
        println!("Enter your guess: ");
        io::stdin().read_line(&mut guess).expect("input failure");
        guess = guess.trim().to_string();
        check_letters(&guess, &answer, &mut correct_letters, &mut hangman);
        if guess == answer
        {
            println!("You win");
            break;
        }
        else 
        {
            println!("correct letters: {}",correct_letters);
            
        }
        println!("hangman: {}", hangman);
        guess.clear()
    }
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
