use std::time::Duration;

use crate::game_state::Move;

use model::{Spaces, TurnRequest, TurnResponse};

pub struct Ai {
    host: String,
    port: u16,
}

impl Ai {
    pub fn from_arg(arg: &str) -> Result<Self, String> {
        let arg = if arg.parse::<u16>().is_ok() {
            format!("localhost:{}", arg)
        } else {
            arg.to_string()
        };

        if !arg.contains(':') {
            return Err(format!(
                "Argument '{}' is not properly formatted. Expected 'hostname:port' or 'port'.",
                arg
            ));
        }

        let parts: Vec<&str> = arg.split(':').collect();
        if parts.len() != 2 || parts[1].parse::<u16>().is_err() {
            return Err(format!(
                "Argument '{}' is not properly formatted. Expected 'hostname:port' or 'port'.",
                arg
            ));
        }

        let host = parts[0].to_string();
        let port = parts[1]
            .parse::<u16>()
            .map_err(|_| format!("Invalid port number in argument '{}'.", arg))?;

        Ok(Self { host, port })
    }

    pub async fn make_move(
        &self,
        reqwest_client: &reqwest::Client,
        turn: usize,
        spaces: &Spaces,
        player_id: String,
        game_id: String,
    ) -> Option<Move> {
        let request_body = TurnRequest {
            player_id: player_id.clone(),
            game_id,
            turn,
            spaces: spaces.clone(),
        };
        let response: Option<TurnResponse> = reqwest_client
            .post(format!("http://{}:{}", self.host, self.port))
            .timeout(Duration::from_millis(500))
            .json(&request_body)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        response.map(|r| Move {
            owner: player_id,
            units: spaces[r.from.x][r.from.y].get_units(),
            from: r.from,
            to: r.to,
        })
    }
}
