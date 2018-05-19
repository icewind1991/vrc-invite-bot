extern crate hyper;
extern crate restson;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate log;
extern crate simple_logger;

use hyper::header::Cookie;
use restson::{Error, RestClient, RestPath};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum NotificationType {
    All,
    Message,
    FriendRequest,
    Invite,
    VoteToKick,
    Help,
    Hidden,
    RequestInvite
}

impl ToString for NotificationType {
    fn to_string(&self) -> String {
        match *self {
            NotificationType::All => "all".to_string(),
            NotificationType::Message => "message".to_string(),
            NotificationType::FriendRequest => "friendrequest".to_string(),
            NotificationType::Invite => "invitemessage".to_string(),
            NotificationType::VoteToKick => "votetokick".to_string(),
            NotificationType::Help => "help".to_string(),
            NotificationType::Hidden => "hidden".to_string(),
            NotificationType::RequestInvite => "requestinvite".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct NotificationResponse {
    id: String,
    #[serde(rename = "senderUserId")]
    sender_user_id: String,
    #[serde(rename = "senderUsername")]
    sender_user_name: String,
    #[serde(rename = "type")]
    message_type: NotificationType,
    message: String,
    details: String,
    seen: bool,
    created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum NotificationList {
    Array(Vec<NotificationResponse>)
}

impl RestPath<()> for NotificationList {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(format!("/api/1/auth/user/notifications"))
    }
}

#[derive(Serialize, Deserialize)]
struct AcceptFriendRequest {}

impl RestPath<String> for AcceptFriendRequest {
    fn get_path(notification_id: String) -> Result<String, Error> { Ok(format!("/api/1/auth/user/notifications/{}/accept", notification_id)) }
}

fn main() {
//    simple_logger::init().unwrap();
    let mut client = RestClient::new("https://vrchat.com").unwrap();
    let mut api_key_cookie = Cookie::new();
    api_key_cookie.set("apiKey", "JlE5Jldo5Jibnk5O5hTx6XVqsJu4WJ26");
    client.set_auth("", "");
    client.set_header(api_key_cookie);
    let result: Result<NotificationList,restson::Error> = client.get(());
    match result {
        Err(err) => println!("{:?}", err),
        Ok(notifications) => println!("{:?}", notifications)
    }
}
