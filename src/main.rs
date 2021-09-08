use data::Config;
use client::GitLabApi;

mod data;
mod client;


#[tokio::main]
async fn main() {
    let token = "MkCQJsuPywMmnjYAaXwm";  // Todo read from stdin
    let config = Config::new(token);
    let api_client = GitLabApi::new(config);

    let resp = api_client.create_snippet("Testing").await.unwrap();
    println!("{:#?}", resp);

    let resp = api_client.snippet_upload(resp.id, "foo_bar.md", "# Lorem ipsum" ).await.unwrap();
    println!("{:#?}", resp)
}
