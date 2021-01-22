use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Summoner {
    #[serde(alias = "id")]
    pub id: String,
    #[serde(alias = "accountId")]
    pub account_id: String,
    #[serde(alias = "puuid")]
    pub puuid: String,
    #[serde(alias = "name")]
    pub name: String,
    #[serde(alias = "profileIconId")]
    pub profile_icon_id: u32,
    #[serde(alias = "revisionDate")]
    pub revision_date: u64,
    #[serde(alias = "summonerLevel")]
    pub summoner_level: u32,
}

impl Summoner {
    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn to_summoner(json: String) -> Self {
        serde_json::from_str(&json).unwrap()
    }
}
/// Lifetime annotated wrapper for "search for summoner by summoner name"
pub async fn summoners_by_summoner_name<'a>(
    region: &'a str,
    api_key: &'a str,
    summoner_name: &'a str,
) -> Result<Summoner, reqwest::Error> {
    
    let url = format!(
        "https://{}.api.riotgames.com/lol/summoner/v4/summoners/by-name/{}",
        region, summoner_name
    );

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();

    headers.insert("X-Riot-Token", HeaderValue::from_str(api_key).unwrap());
    let resp = client
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    let summoner: Summoner = serde_json::from_str(&resp).unwrap();
    Ok(summoner)
}
