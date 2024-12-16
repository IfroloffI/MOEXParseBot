use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct MOEXResponse {
    pub securities: MOEXSecurities,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MOEXSecurities {
    pub columns: Vec<String>,
    pub data: Vec<Vec<String>>,
}

pub async fn get_moex_data() -> Result<MOEXResponse> {
    let client = reqwest::Client::new();
    let resp = client
    .get("https://iss.moex.com/iss/engines/currency/markets/selt/boards/united/securities.json")
    .send()
    .await?
    .json::<MOEXResponse>()
    .await?;
    Ok(resp)
}