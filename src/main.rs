#[allow(unused_imports)]  // bring the trait into scope so method Opts::parse() works;
use clap::Clap;
use snipper::Mode::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = snipper::Api::new();
    let snippet = match api.config.mode {
        Get => api.get_snippet().await?,
        Create => api
            .get_snippet().await
            .unwrap_or(api.create_snippet().await?),
        Update => {
            let snippet = api.get_snippet().await?;
            api.upload_file(snippet.id).await?
        },
    };
    println!("{}", serde_json::to_string_pretty(&snippet)?);

    Ok(())
}
