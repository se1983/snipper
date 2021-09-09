use clap::Clap;

use client::GitLabApi;

mod data;
mod client;

#[derive(Clap, Debug, Clone)]
pub struct Opts {
    pub token: String,
    pub mode: String,
    pub url: String,
    #[clap(long)]
    pub id: Option<usize>,
    #[clap(long)]
    pub title: Option<String>,
    #[clap(long)]
    pub file_path: Option<String>,
    pub file_content: Option<String>,
}

#[tokio::main]
async fn main() {
    let config: Opts = Opts::parse();
    let api = GitLabApi::new(config.clone());

    let resp = match config.mode.as_str() {
        "Create" => {
            api.do_post()
                .await
                .unwrap_or_else(|err| panic!("Could not create the Snippet: [{}]", err));
        }
        "Update" => {
            api.do_put()
                .await
                .unwrap_or_else(|err| panic!("Could not update the Snippet: [{}]", err));
        }
        _ => panic!("mode not accepted!")
    };

    let response_text = serde_json::to_string_pretty(&resp).unwrap();
    println!("{}", response_text);
}


