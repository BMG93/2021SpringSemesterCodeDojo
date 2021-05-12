use std::io;
use rand::Rng;
fn main() {
    
    println!("Let's play a game!");
    let code = vec!(random_color(), random_color(), random_color(), random_color());
    println!("{:?}",code);

    for guess in (1..13).rev() {
        println!("Guess the code!");
        println!(" Remaining Guess(es): {}", guess);
        let mut rawguessinput = String::new();
        let mut guess_vec = Vec::new();

            io::stdin()
            .read_line(&mut rawguessinput)
            .expect("Failed to read line");
        
        for character in rawguessinput.chars() {
            if let Some(digit) = character.to_digit(10) {
                guess_vec.push(digit);
            }
        }
        println!("{:?} is the guess_vec", guess_vec);
        let mut colors_in_right_spot:u32 = 0;
        let mut colors_in_wrong_spot:u32 = 0;
        let mut counter = 1;
        while counter < 4 {
            counter += 1;
            if guess_vec[counter] == code[counter] {
                colors_in_right_spot += 1;
            };
        };
        for n in guess_vec {
            for i in &code {
                if n == *i {
                    colors_in_wrong_spot += 1;
                };
            };
        };
        colors_in_wrong_spot -= colors_in_right_spot; 
        println!("hint1: Matching colors in matching spots: {}", colors_in_right_spot);
        println!("hint2: Matching colors in wrong spot: {}", colors_in_wrong_spot);
        if colors_in_right_spot == 4 {
            println!("You Win!")
        };
    }
}
fn random_color() -> u32 {
    let a_random_color = rand::thread_rng().gen_range(1..6);
    a_random_color
    }


//TODO: put numbers into an array that I can remove certain wrong answers from
//TODO: save correct guess information to increase chances of finding right answer
//TODO: can I win without a single random guess?
