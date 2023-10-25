mod cli;
mod enums;
mod macro_calc;

extern crate console;
use console::{style, Term};

fn main() {
    let term = Term::stdout();
    term.clear_screen().unwrap();

    // Header
    println!("{}", style("Macro Calculator by Mario").bold().underlined());
    println!();

    let weight = cli::get_weight();
    let exercise = cli::get_exercise_level();
    let goal = cli::get_goal();

    let calories = macro_calc::calculate_calories(weight, exercise, goal);
    println!(
        "\n{}",
        style(format!(
            "Your max calorie intake should be: {:.2}",
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
}
