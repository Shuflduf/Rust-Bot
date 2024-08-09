use ask_gemini::Gemini;

use crate::secrets;

pub async fn ask(prompt: &str) -> String {

    let final_response: String;

    let model = Gemini::new(Some(secrets::AI_TOKEN), None);


    match model.ask(prompt).await {
        Ok(response) => final_response = response[0].clone(),
        Err(e) => final_response = format!("Error: {}", e),
    }
    

    return final_response

}


