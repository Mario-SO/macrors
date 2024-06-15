use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use std::env;

pub fn meal_prep(
    protein_grams: f32,
    fat_grams: f32,
    carbs_grams: f32,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());

    let req = ChatCompletionRequest::new(
        chat_completion::GPT4.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: format!("Based on this macros, create a 5-day meal prep using International System quantities:\nProteins: {} grams\nFats: {} grams\nCarbs: {} grams", protein_grams, fat_grams, carbs_grams),
            name: None,
            function_call: None,
        }],
    );

    let diet = client.chat_completion(req)?;

    Ok(diet.choices[0]
        .message
        .content
        .as_ref()
        .unwrap()
        .trim()
        .to_owned())
}
