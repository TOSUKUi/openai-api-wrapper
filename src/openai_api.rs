pub mod openai_api {
    use core::fmt;

    use actix_web::{http::Error, web::Bytes};
    use futures::{stream::BoxStream, Stream, future::ok};
    use http::{response, StatusCode};
    use reqwest::{self, Body, Response, RequestBuilder};
    use serde::{Deserialize, Serialize};
    use serde_json::{self, Value};

    #[derive(Serialize, Deserialize)]
    pub struct ChatCompletionRequestBody {
        pub model: String,
        pub messages: Vec<ChatCompletionRequestMessage>,
        pub stream: bool
    }

    #[derive(Serialize, Deserialize)]
    pub struct ChatCompletionRequestMessage {
        pub role: String,
        pub content: String,
    }

    const BASE_URL: &str = "https://api.openai.com/v1/chat/completions";

    pub fn chat_complesion_stream_request(body: &ChatCompletionRequestBody) ->  RequestBuilder {
        let openai_api_key = std::env::var("OPENAI_API_KEY").unwrap();
        let client = reqwest::Client::new();
        client
            .post(BASE_URL)
            .body(serde_json::to_string(&body).unwrap())
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", openai_api_key))
    }
    #[derive(Debug)]
    enum OpenAIAPIError {
        BadRequest
    }

    impl std::error::Error for OpenAIAPIError {}

    impl fmt::Display for OpenAIAPIError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
            let error_message = match self {
                OpenAIAPIError::BadRequest => "Bad request",
            };
            write!(f, "{}", error_message)
        }
    }

}
