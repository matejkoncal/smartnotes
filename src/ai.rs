use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};

pub async fn transform_to_md(text: &str) -> String {
    let client = Client::new();

    let template_message = ChatCompletionRequestMessageArgs::default()
        .role(Role::System)
        .content("V sprave dostanes poznamky. Naformatuj ich a vrat vo formate markdown. Zachovaj jazyk v ktorom poznamky su! Ak riadok v poznamkach zacina @ai vnimaj to ako pokyny pre formatovanie. Riad sa nimi ale nezahrn ich do vysledneho suboru.")
        .build()
        .unwrap();

    let message_from_pdf = ChatCompletionRequestMessageArgs::default()
        .role(Role::User)
        .content(text)
        .build()
        .unwrap();

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .temperature(0.0)
        .messages([template_message, message_from_pdf])
        .build()
        .unwrap();

    client.chat().create(request).await.unwrap().choices[0]
        .message
        .content
        .to_owned()
}
