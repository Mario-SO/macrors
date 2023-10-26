mod cli;
mod enums;
mod macro_calc;
mod meal_prep;

extern crate console;
use console::{style, Term};

fn main() {
    let term = Term::stdout();
    term.clear_screen().unwrap();

    // Header
    println!("{}", style("Macro-Diet by mariodev").bold().underlined());
    println!();

    let weight = cli::get_weight();
    let exercise = cli::get_exercise_level();
    let goal = cli::get_goal();

    let calories = macro_calc::calculate_calories(weight, exercise, goal);
    println!(
        "\n{}",
        style(format!(
            "Your maximun calorie intake should be: {:.2}",
            calories
        ))
        .green()
        .bold()
    );

    let (protein_grams, protein_calories, fat_grams, fat_calories, carbs_grams, carbs_calories) =
        macro_calc::calculate_macros(weight, calories);

    // Macros output
    println!(
        "\n{}",
        style("=================== Your Macros ===================").bold()
    );
    println!(
        "{}: {:>5.1} grams | {:>5.1} calories",
        style("Protein").blue().bold(),
        protein_grams,
        protein_calories
    );
    println!(
        "{}:     {:>5.1} grams | {:>5.1} calories",
        style("Fat").red().bold(),
        fat_grams,
        fat_calories
    );
    println!(
        "{}:   {:>5.1} grams | {:>5.1} calories",
        style("Carbs").yellow().bold(),
        carbs_grams,
        carbs_calories
    );
    println!(
        "{}",
        style("===================================================").bold()
    );
    println!();

    if cli::want_meal_prep() {
        let result = meal_prep::meal_prep(protein_grams, fat_grams, carbs_grams);

        match result {
            Ok(diet_plan) => {
                println!(
                    "{}",
                    style("Recommended 5-day meal prep").bold().underlined()
                );
                println!();
                println!("{}", diet_plan);
            }
            Err(e) => {
                eprintln!("Error occurred: {}", e); // Prints to standard error
            }
        }
    } else {
        println!();
        println!(
            "{}",
            style("Thank you for using Macro-Diet!").bold().underlined()
        );
    }
}
