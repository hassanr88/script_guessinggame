/*
By: Hassan Rana
Date: 2026-10-07
Program Details: a guessing game that picks a random number between 1 and 100 and lets you keep guessing until you get it right!
You have two decisions to make. The first one tells you whether your guess was TOO HIGH! or TOO LOW!.
The second decision structure should tell you whether you are FREEZING (more than 50 away, or COLD (more than 25 away), or COOL (more than 15 away), or WARM (more than 10 away), or HOT (more than 5 away), or BOILING (between 1 and 5 numbers away).
It prints a message when you get it right, too and allows you to play a new game.
*/

use rand::Rng;
use std::io;

fn diff(n: i32, m: i32) {
    let difference = (n - m).abs();
                if difference > 50 {
                    println!("You are FREEZING!");
                } else if difference > 25 {
                    println!("You are COLD!");
                } else if difference > 15 {
                    println!("You are COOL!");
                } else if difference > 10 {
                    println!("You are WARM!");
                } else if difference > 5 {
                    println!("You are HOT!");
                } else {
                    println!("You are BOILING!");
                }
}

fn main() {
    println!("Welcome to the Guessing Game! Guess a number between 1 and 100. The closer you get, the hotter you are");
    let mut num = rand::rng();
    let random_num: i32 = num.random_range(1..=100);
    loop {
        let mut input = String::new();
        println!("Enter your Guess:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let guess = input.trim().parse::<i32>();
        if let Ok(guess) = guess {
            if guess < 1 || guess > 100 {
                println!("Please enter a number between 1 and 100.");
                continue;
            } else if guess > random_num {
                println!("TOO HIGH!");
                diff(guess, random_num);
            } else if guess < random_num {
                println!("TOO LOW!");
                diff(guess, random_num);
            } else {
                println!("Congratulations! You guessed the number {}!", random_num);
                break;
            }
        } else {
            println!("Please enter a valid number, decimals are not accepted.");
        }
    }
}
