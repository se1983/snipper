use std::error::Error;
use std::time::Duration;

use reqwest::{Client, header};

use crate::data::{req_body, resp_body};
use crate::Opts;

fn http_client_factory(config: &Opts) -> Result<Client, Box<dyn Error>, > {
    let mut headers = header::HeaderMap::new();
    let mut access_token = header::HeaderValue::from_str(
        config.token.as_str()
    )?;
    access_token.set_sensitive(true);
    headers.insert("PRIVATE-TOKEN", access_token);
    headers.insert(
        "Content-Type",
        header::HeaderValue::from_static("application/json"),
    );

    let client = Client::builder()
            .user_agent("AWSME Snipper")
            .default_headers(headers)
            .timeout(Duration::from_secs(10))
            .build()?;
    Ok(client)
}


#[derive(Debug)]
pub struct GitLabApi {
    config: Opts,
    client: Client,
}

impl GitLabApi {

    pub(crate) fn new(config: Opts) -> GitLabApi {
        let client = http_client_factory(&config).unwrap();
        GitLabApi { config, client }
    }

    pub async fn do_post(&self) -> Result<resp_body::SnippetResponse, Box<dyn Error>> {
        let body = req_body::CreateSnippet::new(&self.config.snippet_title.as_ref().unwrap());
        let body = serde_json::to_string(&body)?;

        let resp = self.client
            .post(&self.config.snippet_url)
            .body(body)
            .send()
            .await?;

        Ok(resp.json().await?)

    }
    pub async fn do_put(&self) -> Result<resp_body::SnippetResponse, Box<dyn Error>> {
        let snippet_id = self.config.snippet_id.unwrap();
        let file_name = self.config.file_path.as_ref().unwrap();
        let file_content = self.config.file_content.as_ref().unwrap();
        let file_content = file_content.as_str();

        let url = format!("{}{}", self.config.snippet_url, snippet_id);
        let body = req_body::Update::new(&file_name, file_content);
        let body = serde_json::to_string(&body)?;

        let resp = self.client
            .put(url)
            .body(body)
            .send()
            .await?;

        Ok(resp.json().await?)

    }


}
