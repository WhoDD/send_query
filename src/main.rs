use serde::{Deserialize, Serialize};
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
enum Role {
    Tank,
    Healer,
    Damager,
    Others,
}

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    id: u32,
    role: Role,
    queue_time: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let players = vec![
        Player { id: 1, role: Role::Tank, queue_time: 3 },
        Player { id: 2, role: Role::Damager, queue_time: 8 },
        Player { id: 3, role: Role::Healer, queue_time: 2 },
        Player { id: 4, role: Role::Others, queue_time: 7 },
        Player { id: 5, role: Role::Damager, queue_time: 1 },
        Player { id: 6, role: Role::Tank, queue_time: 10 },
        Player { id: 7, role: Role::Healer, queue_time: 6 },
        Player { id: 8, role: Role::Others, queue_time: 3 },
    ];

    let json_data = serde_json::to_value(&players)?;

    let client = reqwest::Client::new();
    let res = client.post("http://127.0.0.1:80/mm")
        .body(json_data.to_string())
        .send()
        .await?;

    println!("{:?}", res.text().await?);

    Ok(())
}