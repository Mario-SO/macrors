extern crate dialoguer;
use dialoguer::{theme::ColorfulTheme, Input, Select};

pub enum Exercise {
    Sedentary,
    Light,
    Moderate,
    Heavy,
}

pub enum Goal {
    Lose,
    Maintain,
    Gain,
}

fn main() {
    // clear the terminal using the clear command from linux
    std::process::Command::new("clear").status().unwrap();

    println!("===========================================");
    println!("============= Macro Calculator ============");
    println!("=============  Made by Mario   ============");
    println!("===========================================");

    println!("");

    let weight: f32 = Input::new()
        .with_prompt("Enter your weight in kg")
        .interact()
        .unwrap();

    let exercise_selections = &["Sedentary", "Light", "Moderate", "Heavy"];
    let exercise_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select your exercise level")
        .default(0)
        .items(&exercise_selections[..])
        .interact()
        .unwrap();
    let exercise = match exercise_choice {
        0 => Exercise::Sedentary,
        1 => Exercise::Light,
        2 => Exercise::Moderate,
        3 => Exercise::Heavy,
        _ => unreachable!(),
    };

    let goal_selections = &["Lose weight", "Maintain weight", "Gain weight"];
    let goal_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your goal?")
        .default(0)
        .items(&goal_selections[..])
        .interact()
        .unwrap();
    let goal = match goal_choice {
        0 => Goal::Lose,
        1 => Goal::Maintain,
        2 => Goal::Gain,
        _ => unreachable!(),
    };

    let calories = match exercise {
        Exercise::Sedentary => weight * 22.0 * 1.2,
        Exercise::Light => weight * 22.0 * 1.4,
        Exercise::Moderate => weight * 22.0 * 1.6,
        Exercise::Heavy => weight * 22.0 * 1.8,
    };

    let calories = match goal {
        Goal::Lose => calories - (calories * 0.2),
        Goal::Maintain => calories,
        Goal::Gain => calories + (calories * 0.2),
    };

    println!("Your max calorie intake should be: {:.1}", calories);

    let protein_grams = weight * 2.5;
    let protein_calories = protein_grams * 4.0;

    let fat_grams = weight;
    let fat_calories = fat_grams * 9.0;

    let carbs_calories = calories - (protein_calories + fat_calories);
    let carbs_grams = carbs_calories / 4.0;

    println!("=================== Your Macros ===================");
    println!(
        "Protein: {:>5.1} grams | {:>5.1} calories",
        protein_grams, protein_calories
    );
    println!(
        "Fat:     {:>5.1} grams | {:>5.1} calories",
        fat_grams, fat_calories
    );
    println!(
        "Carbs:   {:>5.1} grams | {:>5.1} calories",
        carbs_grams, carbs_calories
    );
    println!("===================================================");
}
