use kalosm::{*, language::*};

#[tokio::main]
async fn main() {
    let llm = Llama::builder()
    .with_source(LlamaSource::llama_3_2_1b_chat())
    .build().await.unwrap();
    
    let mut chat = Chat::builder(llm)
    .with_system_prompt("").build();

    let mut response_stream= chat.add_message("Hello");
    response_stream.to_std_out().await.unwrap();

}