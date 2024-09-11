pub mod requests {
    use reqwest::header::CONTENT_TYPE;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize, Debug)]
    struct GETAPIResponse {
        origin: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct JSONResponse {
        json: HashMap<String, String>,
    }

    pub async fn get_resp() -> Result<String, Box<dyn std::error::Error>> {
        let client: reqwest::Client = reqwest::Client::new();
        let resp200: GETAPIResponse = client
            .get("https://httpbin.org/ip")
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await?
            .json::<GETAPIResponse>()
            .await?;

        Ok(resp200.origin)
    }
}
