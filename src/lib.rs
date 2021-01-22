#![allow(dead_code)]
use serde::Deserialize;
use slurp::read_all_to_string;

// import modules
mod accountv1;
mod summonerv4;
/**
 *
 * ApiKey is a shortcut to deserializing the config. Other options may be added in the future, but for now it's just the key
 *
 *
 */
#[derive(Deserialize)]
struct ApiKey {
    api_key: String,
}

fn regenerate_api_key() -> String {
    let config: String = read_all_to_string("config.toml").unwrap();
    let regen_key: ApiKey = toml::from_str(&config).expect("failed to parse toml");
    print!("");
    return regen_key.api_key;
}

#[cfg(test)]
mod tests {
    use super::*;
    use accountv1::accountv1::accounts_by_puuid;
    use accountv1::accountv1::Account;
    use summonerv4::summonerv4::{summoners_by_summoner_name, Summoner};
    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn request_by_username() {
        let my_api_key = regenerate_api_key();
        let my_summoner = "faker";
        let region: &str = "na1";

        let values: Summoner =
            aw!(summoners_by_summoner_name(region, &my_api_key, my_summoner)).unwrap();

        assert_eq!(values.name.to_lowercase(), my_summoner);
    }
    #[test]
    fn request_by_puuid() {
        let my_api_key = regenerate_api_key();
        let my_summoner = "faker";
        let region: &str = "americas";
        let server_region: &str = "na1";

        let values: Summoner = aw!(summoners_by_summoner_name(
            server_region,
            &my_api_key,
            my_summoner
        ))
        .unwrap();

        assert_eq!(values.name.to_lowercase(), my_summoner);

        let puuid = &values.puuid.clone();
        let result_by_puuid: Account =
            aw!(accounts_by_puuid(puuid, "americas", &my_api_key)).unwrap();

        assert!(
            result_by_puuid.puiid
                == "cmK75yiGPkl0dA9HDsuq0RY17Jwe1TUhTNJnYI-StvseI-eyMuYIUodCoc6dZBhbaUxoS33RuA-OyA"
                    .to_owned()
        );

        assert!(result_by_puuid.game_name.to_lowercase() == "faker");
        assert!(result_by_puuid.tag_line.to_lowercase() == "na1");
    }
    #[test]
    fn request_by_riot_id() {
        let riot_id = "Faker";
        let tag_line = "na1";
        let api_key = regenerate_api_key();

        let values: Account = aw!(accountv1::accountv1::account_by_riot_id(
            riot_id, tag_line, &api_key
        ))
        .unwrap();

        assert!(
            values.puiid
                == "cmK75yiGPkl0dA9HDsuq0RY17Jwe1TUhTNJnYI-StvseI-eyMuYIUodCoc6dZBhbaUxoS33RuA-OyA"
                    .to_owned()
        );

        assert!(values.game_name.to_lowercase() == "faker");
        assert!(values.tag_line.to_lowercase() == "na1");
    }
}
