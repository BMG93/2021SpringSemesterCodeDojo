//use std::io;
use rand::Rng;
/* I am not sure yet of the best way to create a list of all guesses that is easily accessible still I'm wondering if I should treat each location in the code as a separate value and if that would be best for a struct. 
 */
fn main() {
    
    println!("Let's play a game!");
    let code = vec!(random_color(), random_color(), random_color(), random_color());
    println!("{:?}",code);
    for guess in (1..13).rev() {
        println!("Guess the code!");
        println!(" Remaining Guess(es): {}", guess);
        //let mut rawguessinput = String::new();
        let mut guess_vec = Vec::new();
/* no need for io when I am not inputting the guesses
            io::stdin()
            .read_line(&mut rawguessinput)
            .expect("Failed to read line");
        
        for character in rawguessinput.chars() {
            if let Some(digit) = character.to_digit(10) {
                guess_vec.push(digit);
            }
        } */
        println!("{:?} is the guess_vec", guess_vec);
        let mut colors_in_right_spot:u32 = 0;
        let mut colors_in_wrong_spot:u32 = 0;
        let mut counter = 0;
        while counter <= 3 {
            if guess_vec[counter] == code[counter] {
                colors_in_right_spot += 1;
            }
            else if code.contains(&guess_vec[counter]) && guess_vec[counter] != code[counter] {
                colors_in_wrong_spot += 1;
            }
            counter += 1
        }

        println!("hint1: Matching colors in matching spots: {}", colors_in_right_spot);
/*         if guess_vec != code {
            println!("You didn't win!");
            // remove the guess_vec from the list of possible guess vec's or remove them earlier as you try them if you are just using possible correct answers.
        }; */
        if guess_vec == code {
            println!("You win!");
        };
        println!("hint2: Matching colors in wrong spot: {}", colors_in_wrong_spot);
    }
}
fn random_color() -> u32 {
    let a_random_color = rand::thread_rng().gen_range(1..6);
    a_random_color
    }
