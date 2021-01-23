use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct ChampionInfo {
    #[serde(alias = "maxNewPlayerLevel")]
    pub max_new_player_level: u32,
    #[serde(alias = "freeChampionIdsForNewPlayers")]
    pub free_champion_ids_for_new_players: Vec<u32>,
    #[serde(alias = "freeChampionIds")]
    pub free_champion_ids: Vec<u32>,
}

/// Lifetime annotated wrapper for "search for champion rotations"
pub async fn champion_rotations<'a>(
    region: &'a str,
    api_key: &'a str,
) -> Result<CurrentGameInfo, reqwest::Error> {
    let url = format!(
        "https://{}.api.riotgames.com/lol/platform/v3/champion-rotations",
        region
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

    let champ_info: ChampionInfo = serde_json::from_str(&resp).unwrap();
    Ok(champ_info)
}
