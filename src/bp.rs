pub mod stats {
    use reqwest::{self, Error};
    use serde::{Deserialize, Serialize};
    use serde_json;

    static ALL_PLAYER_STATS_URL: &'static str = "https://breakingpoint.gg/api/trpc/playerStats.getAggregatedOrderedPlayerStats?input=%7B%22json%22%3A%7B%22eventType%22%3A%5B%5D%2C%22mapId%22%3A%5B38%2C21%2C59%2C46%2C53%2C47%2C27%2C76%2C29%2C20%2C80%2C67%2C66%2C18%2C45%2C68%2C28%2C10%2C11%2C15%2C69%2C23%2C70%2C60%2C8%2C54%2C79%2C16%2C25%2C61%2C78%2C55%2C48%2C44%2C56%2C49%2C32%2C9%2C13%2C72%2C30%2C31%2C62%2C12%2C22%2C17%2C57%2C50%2C81%2C41%2C19%2C73%2C51%2C40%2C75%2C39%2C36%2C63%2C77%2C74%2C58%2C33%2C42%2C52%2C24%2C35%2C34%2C71%2C26%2C64%2C65%2C43%2C37%5D%2C%22modeId%22%3A%5B1%2C2%2C3%2C4%2C5%5D%2C%22eventId%22%3A%5B%5D%2C%22teamId%22%3A%5B%5D%2C%22onlyCDLStats%22%3Atrue%2C%22onlyChallengersStats%22%3Afalse%2C%22seasonId%22%3A2026%2C%22startAt%22%3Anull%2C%22endAt%22%3Anull%2C%22aggregateMatchStats%22%3Atrue%7D%2C%22meta%22%3A%7B%22values%22%3A%7B%22startAt%22%3A%5B%22undefined%22%5D%2C%22endAt%22%3A%5B%22undefined%22%5D%7D%7D%7D";

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Json {
        pub hp_map_wins: u64,
        pub snd_map_wins: u64,
        pub ctl_map_wins: u64,
        pub ovl_map_wins: u64,
        pub match_wins: u64,
        pub matches_played: u64,
        pub map_wins: u64,
        pub max_hp_kd: f64,
        pub max_snd_kd: f64,
        pub max_ctl_kd: u64,
        pub max_ovl_kd: f64,
        pub max_match_kills: u64,
        pub max_match_kd: f64,
        pub max_hp_kills: u64,
        pub max_snd_kills: u64,
        pub max_ctl_kills: u64,
        pub max_ovl_kills: u64,
        pub max_ovl_overloads: u64,
        pub points_earned: u64,
        pub player_id: u64,
        pub player_tag: String,
        pub hp_game_count: u64,
        pub snd_game_count: u64,
        pub ctl_game_count: u64,
        pub dom_game_count: u64,
        pub ovl_game_count: u64,
        pub game_count: u64,
        pub assists: u64,
        pub ctl_bp_rating: u64,
        pub contested_hill_time: u64,
        pub damage: u64,
        pub deaths: u64,
        pub defuse_count: u64,
        pub first_blood_count: u64,
        pub first_death_count: u64,
        pub gametime_min: u64,
        pub gametime_sec: u64,
        pub hp_bp_rating: f64,
        pub highest_streak: u64,
        pub hill_time: u64,
        pub kills: u64,
        pub non_traded_kills: u64,
        pub one_v_four_win_count: u64,
        pub one_v_one_win_count: u64,
        pub one_v_three_win_count: u64,
        pub one_v_two_win_count: u64,
        pub overloads: u64,
        pub plant_count: u64,
        pub snd_bp_rating: f64,
        pub snd_rounds: u64,
        pub snipe_count: u64,
        pub zone_capture_count: u64,
        pub zone_tier_capture_count: u64,
        pub ctl_attack_rounds: u64,
        pub hp_kills: u64,
        pub hp_assists: u64,
        pub hp_non_traded_kills: u64,
        pub snd_kills: u64,
        pub snd_assists: u64,
        pub snd_non_traded_kills: u64,
        pub ctl_kills: u64,
        pub ctl_assists: u64,
        pub ctl_non_traded_kills: u64,
        pub ctl_ticks: u64,
        pub dom_kills: u64,
        pub dom_assists: u64,
        pub dom_non_traded_kills: u64,
        pub ovl_kills: u64,
        pub ovl_assists: u64,
        pub ovl_non_traded_kills: u64,
        pub hp_damage: u64,
        pub hp_deaths: u64,
        pub snd_damage: u64,
        pub snd_deaths: u64,
        pub ctl_damage: u64,
        pub ctl_deaths: u64,
        pub dom_damage: u64,
        pub dom_deaths: u64,
        pub ovl_damage: u64,
        pub ovl_deaths: u64,
        pub hp_gametime: f64,
        pub ctl_gametime: u64,
        pub dom_gametime: u64,
        pub dom_caps: u64,
        pub ovl_gametime: f64,
        pub ovl_overloads: u64,
        pub bp_rating: f64,
        pub slayer_rating: f64,
        pub dmg_per_min: f64,
        pub first_blood_percentage: f64,
        pub kd: f64,
        pub hp_dmg_10m: f64,
        pub hp_kd: f64,
        pub hp_k_10m: f64,
        pub hp_d_10m: f64,
        pub hp_a_10m: f64,
        pub hp_ntk_pct: f64,
        pub hp_obj_10m: f64,
        pub k_p_10m: u64,
        pub snd_damage_per_round: f64,
        pub snd_kd: f64,
        pub snd_kpr: f64,
        pub snd_dpr: f64,
        pub snd_apr: f64,
        pub snd_ntk_pct: f64,
        pub snd_plants_defuses_per_round: f64,
        pub snd_plants_defuses: u64,
        pub ctl_kd: u64,
        pub ctl_k_10m: u64,
        pub ctl_dmg_10m: u64,
        pub ctl_obj_10m: u64,
        pub hp_eng_10m: f64,
        pub ctl_eng_10m: u64,
        pub tes: f64,
        pub ctl_t_atrd: u64,
        pub hp_dmg: u64,
        pub hp_obj_time: u64,
        pub snd_dmg: u64,
        pub snd_odw: u64,
        pub snd_clutch_wins: u64,
        pub snd_adr: f64,
        pub snd_odw_pct: f64,
        pub ctl_dmg: u64,
        pub dom_kd: u64,
        pub dom_k_10m: u64,
        pub dom_d_10m: u64,
        pub dom_caps_10m: u64,
        pub ovl_kd: f64,
        pub ovl_k_10m: f64,
        pub ovl_d_10m: f64,
        pub ovl_a_10m: f64,
        pub ovl_dmg_10m: f64,
        pub ovl_goals_10m: f64,
        pub ovl_eng_10m: f64,
        pub ovl_ntk_pct: f64,
        pub ovl_overloads_per_game: f64,
        pub ovl_dmg: u64,
        pub dmg: u64,
        pub ntk_pct: f64,
        pub dmg_rating: f64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Data {
        pub json: Vec<Json>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Result {
        pub data: Data,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PlayersStats {
        pub result: Result,
    }

    impl Default for PlayersStats {
        fn default() -> Self {
            PlayersStats {
                result: Result {
                    data: Data { json: Vec::new() },
                },
            }
        }
    }

    impl Default for Json {
        fn default() -> Self {
            Json {
                hp_map_wins: 0,
                snd_map_wins: 0,
                ctl_map_wins: 0,
                ovl_map_wins: 0,
                match_wins: 0,
                matches_played: 0,
                map_wins: 0,
                max_hp_kd: 0.0,
                max_snd_kd: 0.0,
                max_ctl_kd: 0,
                max_ovl_kd: 0.0,
                max_match_kills: 0,
                max_match_kd: 0.0,
                max_hp_kills: 0,
                max_snd_kills: 0,
                max_ctl_kills: 0,
                max_ovl_kills: 0,
                max_ovl_overloads: 0,
                points_earned: 0,
                player_id: 0,
                player_tag: String::new(),
                hp_game_count: 0,
                snd_game_count: 0,
                ctl_game_count: 0,
                dom_game_count: 0,
                ovl_game_count: 0,
                game_count: 0,
                assists: 0,
                ctl_bp_rating: 0,
                contested_hill_time: 0,
                damage: 0,
                deaths: 0,
                defuse_count: 0,
                first_blood_count: 0,
                first_death_count: 0,
                gametime_min: 0,
                gametime_sec: 0,
                hp_bp_rating: 0.0,
                highest_streak: 0,
                hill_time: 0,
                kills: 0,
                non_traded_kills: 0,
                one_v_four_win_count: 0,
                one_v_one_win_count: 0,
                one_v_three_win_count: 0,
                one_v_two_win_count: 0,
                overloads: 0,
                plant_count: 0,
                snd_bp_rating: 0.0,
                snd_rounds: 0,
                snipe_count: 0,
                zone_capture_count: 0,
                zone_tier_capture_count: 0,
                ctl_attack_rounds: 0,
                hp_kills: 0,
                hp_assists: 0,
                hp_non_traded_kills: 0,
                snd_kills: 0,
                snd_assists: 0,
                snd_non_traded_kills: 0,
                ctl_kills: 0,
                ctl_assists: 0,
                ctl_non_traded_kills: 0,
                ctl_ticks: 0,
                dom_kills: 0,
                dom_assists: 0,
                dom_non_traded_kills: 0,
                ovl_kills: 0,
                ovl_assists: 0,
                ovl_non_traded_kills: 0,
                hp_damage: 0,
                hp_deaths: 0,
                snd_damage: 0,
                snd_deaths: 0,
                ctl_damage: 0,
                ctl_deaths: 0,
                dom_damage: 0,
                dom_deaths: 0,
                ovl_damage: 0,
                ovl_deaths: 0,
                hp_gametime: 0.0,
                ctl_gametime: 0,
                dom_gametime: 0,
                dom_caps: 0,
                ovl_gametime: 0.0,
                ovl_overloads: 0,
                bp_rating: 0.0,
                slayer_rating: 0.0,
                dmg_per_min: 0.0,
                first_blood_percentage: 0.0,
                kd: 0.0,
                hp_dmg_10m: 0.0,
                hp_kd: 0.0,
                hp_k_10m: 0.0,
                hp_d_10m: 0.0,
                hp_a_10m: 0.0,
                hp_ntk_pct: 0.0,
                hp_obj_10m: 0.0,
                k_p_10m: 0,
                snd_damage_per_round: 0.0,
                snd_kd: 0.0,
                snd_kpr: 0.0,
                snd_dpr: 0.0,
                snd_apr: 0.0,
                snd_ntk_pct: 0.0,
                snd_plants_defuses_per_round: 0.0,
                snd_plants_defuses: 0,
                ctl_kd: 0,
                ctl_k_10m: 0,
                ctl_dmg_10m: 0,
                ctl_obj_10m: 0,
                hp_eng_10m: 0.0,
                ctl_eng_10m: 0,
                tes: 0.0,
                ctl_t_atrd: 0,
                hp_dmg: 0,
                hp_obj_time: 0,
                snd_dmg: 0,
                snd_odw: 0,
                snd_clutch_wins: 0,
                snd_adr: 0.0,
                snd_odw_pct: 0.0,
                ctl_dmg: 0,
                dom_kd: 0,
                dom_k_10m: 0,
                dom_d_10m: 0,
                dom_caps_10m: 0,
                ovl_kd: 0.0,
                ovl_k_10m: 0.0,
                ovl_d_10m: 0.0,
                ovl_a_10m: 0.0,
                ovl_dmg_10m: 0.0,
                ovl_goals_10m: 0.0,
                ovl_eng_10m: 0.0,
                ovl_ntk_pct: 0.0,
                ovl_overloads_per_game: 0.0,
                ovl_dmg: 0,
                dmg: 0,
                ntk_pct: 0.0,
                dmg_rating: 0.0,
            }
        }
    }

    pub async fn get_all_player_stats() -> core::result::Result<String, Error> {
        let body: String = reqwest::get(ALL_PLAYER_STATS_URL).await?.text().await?;

        Ok(body)
    }

    pub fn stats_to_players(
        stats: String,
    ) -> core::result::Result<PlayersStats, serde_json::Error> {
        let player_stats: PlayersStats = serde_json::from_str(&stats)?;

        Ok(player_stats)
    }

    pub fn print_player_tags(players_stats: &PlayersStats) {
        for player in players_stats.result.data.json.iter() {
            println!("Player tag: {}", player.player_tag);
        }
    }

    pub fn get_player_tags(players_stats: &PlayersStats) -> Vec<String> {
        let mut tags: Vec<String> = Vec::new();

        for player in players_stats.result.data.json.iter() {
            tags.push(player.player_tag.clone());
        }

        tags
    }

    pub fn print_players_stats(tag: String, players_stats: &PlayersStats) {
        for player in players_stats.result.data.json.iter() {
            if tag == player.player_tag {
                println!("{:?}", player);
            }
        }
    }

    pub fn get_players_stats(
        tag: String,
        players_stats: &PlayersStats,
    ) -> std::result::Result<Json, Error> {
        let mut player_stats = Json::default();
        for player in players_stats.result.data.json.iter() {
            if tag == player.player_tag {
                player_stats = player.clone();
            }
        }
        Ok(player_stats)
    }

    // TODO Implement get fns or he rest of stats that I  care about
    // kd, hp kd, hp k 10m, snd kd, ovl kd, ovl k 10m
    // calculate snd kills per game, take kpr (kills per round) * 6 round minium. Would this be good?

    pub fn get_kd(tag: String, players_stats: &PlayersStats) -> std::result::Result<f64, Error> {
        let mut kd: f64 = 0.0;
        for player in players_stats.result.data.json.iter() {
            if tag == player.player_tag {
                kd = player.kd;
            }
        }

        Ok(kd)
    }

    pub fn get_hp_kd(tag: String, players_stats: &PlayersStats) -> std::result::Result<f64, Error> {
        let mut hp_kd = 0.0;
        for player in players_stats.result.data.json.iter() {
            if tag == player.player_tag {
                hp_kd = player.hp_kd
            }
        }

        Ok(hp_kd)
    }

    pub fn get_hp_k_10m(
        tag: String,
        players_stats: &PlayersStats,
    ) -> std::result::Result<f64, Error> {
        let mut hp_k_10m = 0.0;
        for player in players_stats.result.data.json.iter() {
            if tag == player.player_tag {
                hp_k_10m = player.hp_k_10m
            }
        }
        Ok(hp_k_10m)
    }

    pub fn get_snd_kd(
        tag: String,
        players_stats: &PlayersStats,
    ) -> std::result::Result<f64, Error> {
        let mut snd_kd = 0.0;
        for player in players_stats.result.data.json.iter() {
            if tag == player.player_tag {
                snd_kd = player.snd_kd
            }
        }

        Ok(snd_kd)
    }

    pub fn get_ovl_kd(
        tag: String,
        players_stats: &PlayersStats,
    ) -> std::result::Result<f64, Error> {
        let mut ovl_kd = 0.0;
        for player in players_stats.result.data.json.iter() {
            if tag == player.player_tag {
                ovl_kd = player.ovl_kd
            }
        }

        Ok(ovl_kd)
    }

    pub fn get_ovl_k_10m(
        tag: String,
        players_stats: &PlayersStats,
    ) -> std::result::Result<f64, Error> {
        let mut ovl_k_10m = 0.0;
        for player in players_stats.result.data.json.iter() {
            if tag == player.player_tag {
                ovl_k_10m = player.ovl_k_10m
            }
        }

        Ok(ovl_k_10m)
    }

    pub fn get_snd_kills(
        tag: String,
        players_stats: &PlayersStats,
    ) -> std::result::Result<f64, Error> {
        let mut snd_kills = 0.0;
        for player in players_stats.result.data.json.iter() {
            if tag == player.player_tag {
                snd_kills = player.snd_kpr * 6.0;
            }
        }

        Ok(snd_kills)
    }
}
