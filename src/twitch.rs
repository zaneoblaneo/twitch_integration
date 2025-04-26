use crate::secrets::{ 
    CLIENT_SECRET, 
    CLIENT_ID, 
    ACCESS_TOKEN, 
    USER_ID
};

use crate::game::User;
use raylib::prelude::*;

use crate::error::Error;

use reqwest::{ 
    Client, 
    Request, 
    Response, 
    header::{
        HeaderMap
    }, 
    Url,
    Method,
    RequestBuilder
};

use serde_json::Value;

use std::collections::HashMap;

pub async fn make_chatters_req() -> Result<Response, Error> {
    let uri: String = format!("https://api.twitch.tv/helix/chat/chatters?broadcaster_id={}&moderator_id={}", USER_ID, USER_ID);
    let bearer: String = format!("Bearer {}", ACCESS_TOKEN);
    let client: Client = Client::new();
    let mut req: Request = Request::new(Method::GET, Url::parse(uri.as_str())?);
    let headers: &mut HeaderMap = req.headers_mut();
    headers.append("Authorization", bearer.as_str().parse()?);
    headers.append("Client-Id", CLIENT_ID.parse()?);
    let req_builder: RequestBuilder = RequestBuilder::from_parts(client, req);
    Ok(req_builder.send().await?)
}

fn parse_color(s: String) -> Result<Color, Error> {
    if !s.starts_with("#") {
        return Err(Error::InvalidColor);
    }
    let tmp = s.replace("#", "");
    let chars: Vec<char> = tmp.chars().collect();
    if chars.len() != 6 {
        return Err(Error::InvalidColor);
    }
    let mut conv_map: HashMap<char, u8> = HashMap::new();
    conv_map.insert('0', 0x0);
    conv_map.insert('1', 0x1);
    conv_map.insert('2', 0x2);
    conv_map.insert('3', 0x3);
    conv_map.insert('4', 0x4);
    conv_map.insert('5', 0x5);
    conv_map.insert('6', 0x6);
    conv_map.insert('7', 0x7);
    conv_map.insert('8', 0x8);
    conv_map.insert('9', 0x9);
    conv_map.insert('a', 0xa);
    conv_map.insert('b', 0xb);
    conv_map.insert('c', 0xc);
    conv_map.insert('d', 0xd);
    conv_map.insert('e', 0xe);
    conv_map.insert('f', 0xf);
    conv_map.insert('A', 0xa);
    conv_map.insert('B', 0xb);
    conv_map.insert('C', 0xc);
    conv_map.insert('D', 0xd);
    conv_map.insert('E', 0xe);
    conv_map.insert('F', 0xf);

    Ok(Color {
        r: conv_map.get(&chars[0]).ok_or(Error::InvalidColor)? << 4 | conv_map.get(&chars[1]).ok_or(Error::InvalidColor)?,
        g: conv_map.get(&chars[2]).ok_or(Error::InvalidColor)? << 4 | conv_map.get(&chars[3]).ok_or(Error::InvalidColor)?,
        b: conv_map.get(&chars[4]).ok_or(Error::InvalidColor)? << 4 | conv_map.get(&chars[5]).ok_or(Error::InvalidColor)?,
        a: 0xff,
    })
}

pub async fn get_chat_colors(users: &mut HashMap<String, User>) -> Result<(), Error> {
    let user_ids: Vec<String> = users.keys().map( |x| -> String { x.to_owned() }).collect();
    let batches = (user_ids.len() + 98) / 99usize;

    for batch in 0..batches {
        let mut uri: String = format!("https://api.twitch.tv/helix/chat/color?user_id={}", USER_ID);
        for user in batch * 99..(std::cmp::min((batch + 1) * 99, users.len())) {
            uri.push_str(format!("&user_id={}", user_ids[user]).as_str());
        }
        let bearer: String = format!("Bearer {}", ACCESS_TOKEN);
        let client: Client = Client::new();
        let mut req: Request = Request::new(Method::GET, Url::parse(uri.as_str())?);
        let headers: &mut HeaderMap = req.headers_mut();
        headers.append("Authorization", bearer.as_str().parse()?);
        headers.append("Client-Id", CLIENT_ID.parse()?);
        let req_builder: RequestBuilder = RequestBuilder::from_parts(client, req);
        let resp: Response = req_builder.send().await?;
        let resp_parsed: Value = serde_json::from_str(&resp.text().await?)?;
        if let Value::Array(data) = &resp_parsed["data"] {
            for i in 0..data.len() {
                let user_id: String = data[i]["user_id"].to_string().replace("\"", "");
                let color: Color = parse_color(data[i]["color"].to_string().replace("\"", ""))?;
                users.get_mut(&user_id).ok_or(Error::InvalidKey)?.user_color = Some(color);
            }
        };
    }
    Ok(())
}

pub async fn get_moderators(users: &mut HashMap<String, User>) -> Result<(), Error> {
    let user_ids: Vec<String> = users.keys().map( |x| -> String { x.to_owned() }).collect();
    let batches = (users.len() + 99) / 100usize;

    for batch in 0..batches {
        let mut uri: String = format!("https://api.twitch.tv/helix/moderation/moderators?broadcaster_id={}", USER_ID);
        for user in (batch * 100)..std::cmp::min((batch + 1) * 100, users.len()) {
            uri.push_str(format!("&user_id={}", user_ids[user]).as_str());
        }
        let bearer: String = format!("Bearer {}", ACCESS_TOKEN);
        let client: Client = Client::new();
        let mut req: Request = Request::new(Method::GET, Url::parse(uri.as_str())?);
        let headers: &mut HeaderMap = req.headers_mut();
        headers.append("Authorization", bearer.as_str().parse()?);
        headers.append("Client-Id", CLIENT_ID.parse()?);
        let req_builder: RequestBuilder = RequestBuilder::from_parts(client, req);
        let resp: Response = req_builder.send().await?;
        let resp_parsed: Value = serde_json::from_str(&resp.text().await?)?;
        if let Value::Array(data) = &resp_parsed["data"] {
            for i in 0..data.len() {
                let user_id: String = data[i]["user_id"].to_string().replace("\"", "");
                users.get_mut(user_id.as_str()).ok_or(Error::InvalidKey)?.mod_flag = true;
            }
        } else {
            panic!("tried to get moderators, but data wasn't a Value::Array");
        }
    }

    Ok(())
}

pub async fn get_users() -> Result<HashMap<String, User>, Error> {
    let mut out: HashMap<String, User> = HashMap::new();
    let chatters = make_chatters_req().await?;
    let v: Value = serde_json::from_str(&chatters.text().await?)?;
    let total = &v["total"];
    let users = &v["data"];
    for i in 0..(total.as_u64()).ok_or(Error::Nan)? {
        let user = &users[i as usize];
        out.insert(user["user_id"].as_str().ok_or(Error::NotAString)?.to_owned(), 
            User {
                user_name: user["user_name"].as_str().ok_or(Error::NotAString)?.to_owned(),
                mod_flag: false,
                debug_flag: false,
                user_color: None,
                pos: None,
                vel: None,
                acc: None,
            }
        );
    }
    Ok(out)
}

pub async fn update_users() -> Result<HashMap<String, User>, Error> {
    let mut users = get_users().await?;
    get_moderators(&mut users).await?;
    get_chat_colors(&mut users).await?;
    Ok(users)
}
