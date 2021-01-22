use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Account {
    #[serde(alias = "puuid")]
    pub puiid: String,
    #[serde(alias = "gameName")]
    pub game_name: String,
    #[serde(alias = "tagLine")]
    pub tag_line: String,
}

pub async fn accounts_by_puuid<'a>(
    puuid: &'a str,
    region: &'a str,
    api_key: &'a str,
) -> Result<Account, reqwest::Error> {
    let mut headers = HeaderMap::new();
    let client = reqwest::Client::new();
    headers.insert("X-Riot-Token", HeaderValue::from_str(api_key).unwrap());

    let url = format!(
        "https://{}.api.riotgames.com/riot/account/v1/accounts/by-puuid/{}",
        region, puuid
    );

    let res = client
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(serde_json::from_str(&res).unwrap())
}

pub async fn account_by_riot_id<'a>(
    riot_id: &'a str,
    tag_line: &'a str,
    api_key: &'a str,
) -> Result<Account, reqwest::Error> {
    let mut headers = HeaderMap::new();
    let client = reqwest::Client::new();
    headers.insert("X-Riot-Token", HeaderValue::from_str(api_key).unwrap());

    let url = format!(
        "https://americas.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}",
        riot_id, tag_line
    );

    let res = client
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(serde_json::from_str(&res).unwrap())
}
