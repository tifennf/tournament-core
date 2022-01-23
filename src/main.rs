use tournament_core::ressources::player::RiotPlayer;

#[tokio::main]
async fn main() {
    let api_key = "RGAPI-c1cf5b58-fa9d-44fc-9041-142b7b5aa151";

    let url = format!(
        "https://euw1.api.riotgames.com/tft/summoner/v1/summoners/by-name/daddykebab?api_key={}",
        api_key
    );

    let res = reqwest::get(url).await.unwrap();

    let res = res.json::<RiotPlayer>().await.unwrap();

    println!("{:?}", res);
}
