use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct BannedChampion {
    #[serde(alias = "pickTurn")]
    pub pick_turn: u32,
    #[serde(alias = "championId")]
    pub champion_id: u64,
    #[serde(alias = "teamId")]
    pub team_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Observer {
    #[serde(alias = "encryptionKey")]
    pub encryption_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct Perks {
    #[serde(alias = "perkIds")]
    pub perk_ids: Vec<u64>,
    #[serde(alias = "perkStyle")]
    pub perk_style: u64,
    #[serde(alias = "perkSubStyle")]
    pub perk_sub_style: u64,
}

#[derive(Serialize, Deserialize)]
pub struct GameCustomizationObject {
    #[serde(alias = "category")]
    pub category: String,
    #[serde(alias = "content")]
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct CurrentGameParticipant {
    #[serde(alias = "championId")]
    pub champion_id: u64,
    #[serde(alias = "perks")]
    pub perks: Perks,
    #[serde(alias = "profileIconId")]
    pub profile_icon_id: u64,
    #[serde(alias = "bot")]
    pub bot: bool,
    #[serde(alias = "teamId")]
    pub team_id: u64,
    #[serde(alias = "summonerName")]
    pub summoner_name: String,
    #[serde(alias = "summonerId")]
    pub summoner_id: String,
    #[serde(alias = "spell1Id")]
    pub spell_1_id: u64,
    #[serde(alias = "spell2Id")]
    pub spell_2_id: u64,
    #[serde(alias = "gameCustomizationObjects")]
    pub game_customization_objects: Vec<GameCustomizationObject>,
}

#[derive(Serialize, Deserialize)]
pub struct CurrentGameInfo {
    #[serde(alias = "gameId")]
    pub game_id: u64,
    #[serde(alias = "gameType")]
    pub game_type: String,
    #[serde(alias = "gameStartTime")]
    pub game_start_time: u64,
    #[serde(alias = "mapId")]
    pub map_id: u64,
    #[serde(alias = "gameLength")]
    pub game_length: u64,
    #[serde(alias = "platformID")]
    pub platform_id: String,
    #[serde(alias = "gameMode")]
    pub game_mode: String,
    #[serde(alias = "bannedChampions")]
    pub banned_champions: Vec<BannedChampion>,
    #[serde(alias = "gameQueueConfigId")]
    pub game_queue_config_id: u64,
    #[serde(alias = "observers")]
    pub observers: Observer,
    #[serde(alias = "participants")]
    pub participants: Vec<CurrentGameParticipant>,
}

#[derive(Serialize, Deserialize)]
pub struct Participant {
    #[serde(alias = "bot")]
    pub bot: bool,
    #[serde(alias = "spell1Id")]
    pub spell_1_id: u64,
    #[serde(alias = "spell2Id")]
    pub spell_2_id: u64,
    #[serde(alias = "profileIconId")]
    pub profile_icon_id: u64,
    #[serde(alias = "summonerName")]
    pub summoner_name: String,
    #[serde(alias = "championId")]
    pub champion_id: u64,
    #[serde(alias = "teamId")]
    pub team_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct FeaturedGameInfo {
    #[serde(alias = "gameId")]
    pub game_id: u64,
    #[serde(alias = "gameType")]
    pub game_type: String,
    #[serde(alias = "gameStartTime")]
    pub game_start_time: u64,
    #[serde(alias = "mapId")]
    pub map_id: u64,
    #[serde(alias = "gameLength")]
    pub game_length: u64,
    #[serde(alias = "platformID")]
    pub platform_id: String,
    #[serde(alias = "gameMode")]
    pub game_mode: String,
    #[serde(alias = "bannedChampions")]
    pub banned_champions: Vec<BannedChampion>,
    #[serde(alias = "gameQueueConfigId")]
    pub game_queue_config_id: u64,
    #[serde(alias = "observers")]
    pub observers: Observer,
    #[serde(alias = "participants")]
    pub participants: Vec<Participant>,
}

#[derive(Serialize, Deserialize)]
pub struct FeaturedGames {
    #[serde(alias = "gameList")]
    pub game_list: Vec<FeaturedGameInfo>,
    #[serde(alias = "clientRefreshInterval")]
    pub client_refresh_interval: u64,
}

/// Lifetime annotated wrapper for "search for active games by summoner"
pub async fn active_games_by_summoner<'a>(
    region: &'a str,
    api_key: &'a str,
    encrypted_summoner_id: &'a str,
) -> Result<CurrentGameInfo, reqwest::Error> {
    let url = format!(
        "https://{}.api.riotgames.com/lol/spectator/v4/active-games/by-summoner/{}",
        region, encrypted_summoner_id
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

    let game_info: CurrentGameInfo = serde_json::from_str(&resp).unwrap();
    Ok(game_info)
}

/// Lifetime annotated wrapper for "search for featured games"
pub async fn featured_games<'a>(
    region: &'a str,
    api_key: &'a str,
) -> Result<FeaturedGameInfo, reqwest::Error> {
    let url = format!(
        "https://{}.api.riotgames.com/lol/spectator/v4/featured-games",
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

    let game_info: FeaturedGames = serde_json::from_str(&resp).unwrap();
    Ok(game_info)
}
