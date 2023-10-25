use crate::enums::{Exercise, Goal};

pub fn calculate_calories(weight: f32, exercise: Exercise, goal: Goal) -> f32 {
    let calories = match exercise {
        Exercise::Sedentary => weight * 22.0 * 1.2,
        Exercise::Light => weight * 22.0 * 1.4,
        Exercise::Moderate => weight * 22.0 * 1.6,
        Exercise::Heavy => weight * 22.0 * 1.8,
    };

    match goal {
        Goal::Lose => calories - (calories * 0.2),
        Goal::Maintain => calories,
        Goal::Gain => calories + (calories * 0.2),
    }
}

pub fn calculate_macros(weight: f32, calories: f32) -> (f32, f32, f32, f32, f32, f32) {
    let protein_grams = weight * 2.5;
    let protein_calories = protein_grams * 4.0;

    let fat_grams = weight;
    let fat_calories = fat_grams * 9.0;

    let carbs_calories = calories - (protein_calories + fat_calories);
    let carbs_grams = carbs_calories / 4.0;

    (
        protein_grams,
        protein_calories,
        fat_grams,
        fat_calories,
        carbs_grams,
        carbs_calories,
    )
}
