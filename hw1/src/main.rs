/*
//Declare a constant for the freezing point of water in Fahrenheit (32°F).
const FREEZE_POINT: f64 = 32.0;

//Implement two functions fahrenheit_to_celsiu
fn fahrenheit_to_celsius(f: f64) -> f64{
    (f - FREEZE_POINT) * 5.0 / 9.0
}

//Converts Celsius to Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64{
    (c * 9.0/5.0) + FREEZE_POINT
}

fn main (){
    //Declare a mutable variable with a temperature in Fahrenheit
    let mut temp_f: f64 = FREEZE_POINT;

    //Convert it to Celsius using your function and print the result
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{temp_f} F = {temp_c:.2} C ");

    //loop to convert and print the next 5 integer temperatures
    for _ in 0..5 {
        temp_f += 1.0;
        let temp_c = fahrenheit_to_celsius(temp_f);
        println!("{temp_f} F = {temp_c:.2} C ");
    }
}



        Assignment 2

fn main() {
    // Array of 10 integers
    let numbers = [3, 5, 8, 12, 15, 7, 20, 1, 6, 10];

    // For loop: check each number
    for &num in numbers.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else if is_even(num) {
            println!("{}: Even", num);
        } else {
            println!("{}: Odd", num);
        }
    }

// Function to check if a number is even
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// While loop: compute sum of all numbers
    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of all numbers: {}", sum);

    // Loop to find the largest number
    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest);
}
*/

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    // Hard-coded secret number
    let secret_number: i32 = 7;

    // Variable to track number of guesses
    let mut attempts = 0;

    // Mutable guess variable (simulating user input)
    let mut guess = 0;

    loop {
        attempts += 1; // Increment attempts each iteration

        // Simulate a guess 
        guess += 1; //start at 0 and increment until correct

        // Check the guess
        let result = check_guess(guess, secret_number);

        // Print feedback
        if result == 0 {
            println!("Guess {} is correct!", guess);
            break; // Exit loop if correct
        } else if result == 1 {
            println!("Guess {} is too high.", guess);
        } else {
            println!("Guess {} is too low.", guess);
        }
    }

    
    println!("It took {} guesses to find the secret number.", attempts);
}
