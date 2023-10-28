use crate::enums::{Exercise, Gender, Goal};

pub fn calculate_calories(
    weight: f32,
    height: f32,
    age: f32,
    gender: Gender,
    exercise: Exercise,
    goal: Goal,
) -> f32 {
    let ree_male = 10.0 * weight + 6.25 * height - 5.0 * age + 5.0;
    let ree_female = 10.0 * weight + 6.25 * height - 5.0 * age - 161.0;

    let calories = if matches!(gender, Gender::Male) {
        match exercise {
            Exercise::Sedentary => ree_male * 1.2,
            Exercise::Light => ree_male * 1.375,
            Exercise::Moderate => ree_male * 1.55,
            Exercise::Heavy => ree_male * 1.725,
        }
    } else {
        match exercise {
            Exercise::Sedentary => ree_female * 1.2,
            Exercise::Light => ree_female * 1.375,
            Exercise::Moderate => ree_female * 1.55,
            Exercise::Heavy => ree_female * 1.725,
        }
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
