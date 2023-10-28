use crate::enums::{Exercise, Gender, Goal};
use dialoguer::{theme::ColorfulTheme, Input, Select};

pub fn get_weight() -> f32 {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your weight in kg")
        .interact()
        .unwrap()
}

pub fn get_height() -> f32 {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your height in cm")
        .interact()
        .unwrap()
}

pub fn get_age() -> f32 {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your age")
        .interact()
        .unwrap()
}

pub fn get_gender() -> Gender {
    let gender_selection = &["Male", "Female"];
    let gender_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select your gender")
        .default(0)
        .items(&gender_selection[..])
        .interact()
        .unwrap();
    match gender_choice {
        0 => Gender::Male,
        1 => Gender::Female,
        _ => unreachable!(),
    }
}

pub fn get_exercise_level() -> Exercise {
    let exercise_selections = &["Sedentary", "Light", "Moderate", "Heavy"];
    let exercise_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select your exercise level")
        .default(0)
        .items(&exercise_selections[..])
        .interact()
        .unwrap();
    match exercise_choice {
        0 => Exercise::Sedentary,
        1 => Exercise::Light,
        2 => Exercise::Moderate,
        3 => Exercise::Heavy,
        _ => unreachable!(),
    }
}

pub fn get_goal() -> Goal {
    let goal_selections = &["Lose weight", "Maintain weight", "Gain weight"];
    let goal_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your goal?")
        .default(0)
        .items(&goal_selections[..])
        .interact()
        .unwrap();
    match goal_choice {
        0 => Goal::Lose,
        1 => Goal::Maintain,
        2 => Goal::Gain,
        _ => unreachable!(),
    }
}

pub fn want_meal_prep() -> bool {
    let meal_prep_selections = &["Yes", "No"];
    let meal_prep_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to see meal prep ideas? (requires OpenAI API key)")
        .default(0)
        .items(&meal_prep_selections[..])
        .interact()
        .unwrap();
    match meal_prep_choice {
        0 => true,
        1 => false,
        _ => unreachable!(),
    }
}
