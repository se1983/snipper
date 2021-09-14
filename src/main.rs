#[allow(unused_imports)]  // bring the trait into scope so method Opts::parse() works;
use clap::Clap;
use snipper::Mode::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = snipper::Api::new();

    let snippet = match api.config.mode {
        Create => match api.get_snippet().await? {
                Some(snippet) => snippet,
                None => api.create_snippet().await?
            }

        Update => match api.get_snippet().await?{
                Some(snippet) => api.upload_file(snippet.id).await?,
                None => panic!("Could not find snippet: {}", api.config.title)

        }
        Get => match api.get_snippet().await?{
            Some(snippet) => snippet,
            None => panic!("Could not find snippet: {}", api.config.title)
        }
    };
    println!("{}", serde_json::to_string_pretty(&snippet)?);

    Ok(())
}
