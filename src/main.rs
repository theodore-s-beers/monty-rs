#![warn(clippy::pedantic, clippy::nursery)]

use num_format::{Locale, ToFormattedString};
use rand::distr::Uniform;
use rand::prelude::*;

fn main() {
    // Set up rng
    let mut rng = rand::rng();

    // Create a set of door numbers from which to sample
    let doors = Uniform::new_inclusive(1, 3).unwrap();

    // We'll increment correct guesses as they occur
    let mut correct_guesses = 0;

    for _ in 0..10_000 {
        let true_door = doors.sample(&mut rng);

        let guess = doors.sample(&mut rng);

        if guess == true_door {
            correct_guesses += 1;
        }
    }

    println!(
        "Correct guesses out of 10,000 (control): {}",
        correct_guesses.to_formatted_string(&Locale::en)
    );

    // Starting over...
    correct_guesses = 0;

    for _ in 0..10_000 {
        let true_door = doors.sample(&mut rng);

        let guess = doors.sample(&mut rng);

        // Prepare to eliminate a false door
        // This will do nothing here, of course; just coding for thought process
        let _eliminated_door: u8;

        loop {
            // Pick a random door. If the initial guess was correct, there are
            // two possibilities for which door to eliminate at this stage.
            // We should choose between them at random to keep things legit.
            let possible_elimination = doors.sample(&mut rng);

            // If the random door we just picked is neither the one guessed nor the
            // true door, select it for elimination and break the loop.
            if possible_elimination != true_door && possible_elimination != guess {
                _eliminated_door = possible_elimination;
                break;
            }
        }

        // Imagine that the offer to switch was declined...

        if guess == true_door {
            correct_guesses += 1;
        }
    }

    println!(
        "Correct guesses out of 10,000, after declining to switch: {}",
        correct_guesses.to_formatted_string(&Locale::en)
    );

    // Starting over...
    correct_guesses = 0;

    for _ in 0..10_000 {
        let true_door = doors.sample(&mut rng);

        let mut guess = doors.sample(&mut rng);

        // Prepare to eliminate a false door
        let eliminated_door: u8;

        loop {
            // Pick a random door. If the initial guess was correct, there are
            // two possibilities for which door to eliminate at this stage.
            // We should choose between them at random to keep things legit.
            let possible_elimination = doors.sample(&mut rng);

            // If the random door we just picked is neither the one guessed nor the
            // true door, select it for elimination and break the loop.
            if possible_elimination != true_door && possible_elimination != guess {
                eliminated_door = possible_elimination;
                break;
            }
        }

        // Now we just switch to whichever door was not the original guess, and
        // was not eliminated. There's only one option...
        for i in 1..=3 {
            if i != guess && i != eliminated_door {
                guess = i;
                break;
            }
        }

        if guess == true_door {
            correct_guesses += 1;
        }
    }

    println!(
        "Correct guesses out of 10,000, after switching: {}",
        correct_guesses.to_formatted_string(&Locale::en)
    );
}
