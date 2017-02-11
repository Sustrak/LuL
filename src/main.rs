extern crate hyper;
extern crate hyper_native_tls;
extern crate serde_json;

use hyper::Client;
use std::io;
use std::io::Read;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_json::Value;

struct League {
    solo: String,
    flex:  String
}

fn main() {
    let token = "";
    let token2 = "";
    println!("HI INTRODUCE YOUR SUMMONERS NAME (undercase)");
    let mut summ_name = String::new();
    io::stdin().read_line(&mut summ_name);
    let n = summ_name.len();
    summ_name.truncate(n -2);

    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    //Get summoner ID
    let mut url = build_url(&summ_name, token, "_", false);
    let (ret, json) = get(url, &client);
    if !ret{
        println!("User not found, please press enter to exit ...");
        let mut aux = String::new();
        io::stdin().read_line(&mut aux);
        std::process::exit(1);
    }
    let id = json[&summ_name]["id"].to_string();

    //Get players ID
    url = build_url(&summ_name, token, &id, false);
    let (ret, json) = get(url, &client);
    if !ret {
        println!("The user is not in a game, press enter to exit ...");
        let mut aux = String::new();
        io::stdin().read_line(&mut aux);
        std::process::exit(1);
    }
    let mut summ_ids: Vec<String> = Vec::new();
    let mut summ_names: Vec<String> = Vec::new();
    let mut summ_ids2: Vec<String> = Vec::new();
    let mut summ_names2: Vec<String> = Vec::new();
    for i in 0..10 {
        if json["participants"][i]["teamId"].to_string() == "100" {
            summ_ids.push(json["participants"][i]["summonerId"].to_string());
            summ_names.push(json["participants"][i]["summonerName"].to_string());
        }
        else {
            summ_ids2.push(json["participants"][i]["summonerId"].to_string());
            summ_names2.push(json["participants"][i]["summonerName"].to_string());
        }
    }

    //Get league for each player
    let mut leagues: Vec<League> = Vec::new();
    let mut leagues2: Vec<League> = Vec::new();
    for i in summ_ids {
        url = build_url(&summ_name, token2, &i, true);
        let (ret, json) = get(url, &client);
        let mut league = League {solo:"none".to_string(), flex: "none".to_string()};
        for j in 0..2 {
            if j==0 {league.solo = json[&i][j]["tier"].to_string();}
            else{league.flex = json[&i][j]["tier"].to_string();}
        }
        leagues.push(league);
    }
    for i in summ_ids2 {
        url = build_url(&summ_name, token2, &i, true);
        let (ret, json) = get(url, &client);
        let mut league = League {solo:"none".to_string(), flex: "none".to_string()};
        for j in 0..2 {
            if j==0 {league.solo = json[&i][j]["tier"].to_string();}
            else{league.flex = json[&i][j]["tier"].to_string();}
        }
        leagues2.push(league);
    }

    //Print results
    let mut i: usize = 0;
    println!("YOUR TEAM:");
    while i < 5 {
        println!("           {}", summ_names[i]);
        println!("SOLO:      {}", leagues[i].solo);
        println!("FLEX:      {}\n", leagues[i].flex);
        i += 1;
    }
    i = 0;
    println!("\n ---------------------------- \n ENEMY TEAM:");
    while i < 5 {
        println!("           {}", summ_names2[i]);
        println!("SOLO:      {}", leagues2[i].solo);
        println!("FLEX:      {}\n", leagues2[i].flex);
        i += 1;
    }

    println!("Press enter to exit...");
    let mut aux: String = String::new();
    io::stdin().read_line(&mut aux);
}

fn get(url: String, client: &Client) -> (bool, Value) {
    let mut json = String::new();
    let mut ret = client.get(&url).send().unwrap();
    ret.read_to_string(&mut json).unwrap();
    (ret.status.is_success(), serde_json::from_str(&json).unwrap())
}

fn build_url(name: &String, token: &str, id: &str, league: bool) -> String {
    let mut url = String::new();
    if id == "_" {
        //Id of the summoner
        url = "https://euw.api.pvp.net/api/lol/euw/v1.4/summoner/by-name/".to_string()
              + name + "?api_key=" + &token;
    }
    else if league {
        //League info of the summoner
        url = "https://euw.api.pvp.net/api/lol/euw/v2.5/league/by-summoner/".to_string()
              + id + "?api_key=" + &token;
    }
    else {
        //Info of the game
        url = "https://euw.api.pvp.net/observer-mode/rest/consumer/getSpectatorGameInfo/EUW1/".to_string()
              + id + "?api_key=" + &token;
    }
    return url;
}
