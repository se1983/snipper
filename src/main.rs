use clap::Clap;

use client::GitLabApi;

mod data;
mod client;

#[derive(Clap, Debug, Clone)]
pub struct Opts {
    pub token: String,
    pub mode: String,
    pub snippet_url: String,
    #[clap(long)]
    pub snippet_id: Option<usize>,
    #[clap(long)]
    pub snippet_title: Option<String>,
    #[clap(long)]
    pub file_path: Option<String>,
    pub file_content: Option<String>,
}

#[tokio::main]
async fn main() {
    let config: Opts = Opts::parse();
    let api_client = GitLabApi::new(config.clone());

    let resp = match config.mode.as_str() {
        "Create" => {
            let title = config.snippet_title.unwrap();
            api_client.create_snippet(title.as_str()).await.unwrap()
        }
        "Update" => {
            let snippet_id = config.snippet_id.unwrap();
            let file_name = config.file_path.unwrap();
            let file_name = file_name.as_str();
            let file_content = config.file_content.unwrap();
            let file_content = file_content.as_str();
            api_client.snippet_upload(
                snippet_id,
                file_name,
                file_content,
            ).await.unwrap()
        }
        _ => panic!("mode not accepted!")
    };

    let response_text = serde_json::to_string_pretty(&resp).unwrap();
    println!("{}", response_text);
}


