use serde::Deserialize;

#[derive(Deserialize)]
struct Response {
    teams: Vec<Team>
}

#[derive(Deserialize)]
struct Team {
    fileCode: String
}

pub(crate) fn get_team_code(player_id: i32) -> String {
    let res: Response = reqwest::blocking::get(format!("https://statsapi.mlb.com/api/v1/people/{}/stats", player_id)).unwrap().json().unwrap();
    res.teams[0].fileCode.to_string()
}