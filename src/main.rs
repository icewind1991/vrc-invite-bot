extern crate hyper;
extern crate log;
extern crate restson;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate simple_logger;
extern crate openssl_probe;

use hyper::header::Cookie;
use restson::{Error, RestClient, RestPath};
use std::{thread, time};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum NotificationType {
    All,
    Message,
    #[serde(rename = "friendRequest")]
    FriendRequest,
    Invite,
    VoteToKick,
    Help,
    Hidden,
    RequestInvite,
}

impl ToString for NotificationType {
    fn to_string(&self) -> String {
        match *self {
            NotificationType::All => "all".to_string(),
            NotificationType::Message => "message".to_string(),
            NotificationType::FriendRequest => "friendRequest".to_string(),
            NotificationType::Invite => "invite".to_string(),
            NotificationType::VoteToKick => "votetokick".to_string(),
            NotificationType::Help => "help".to_string(),
            NotificationType::Hidden => "hidden".to_string(),
            NotificationType::RequestInvite => "requestinvite".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct InstanceId {
    world: String,
    instance: String,
}

impl ToString for InstanceId {
    fn to_string(&self) -> String {
        format!("{}:{}", self.world, self.instance)
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

#[derive(Serialize, Deserialize, Debug)]
struct Notification {
    #[serde(rename = "type")]
    message_type: NotificationType,
    details: String,
    message: String,
}

impl RestPath<String> for Notification {
    fn get_path(target_user_id: String) -> Result<String, Error> {
        Ok(format!("/api/1/auth/user/{}/notification", target_user_id))
    }
}

#[derive(Serialize, Deserialize)]
struct AcceptFriendRequest {}

impl RestPath<String> for AcceptFriendRequest {
    fn get_path(notification_id: String) -> Result<String, Error> { Ok(format!("/api/1/auth/user/notifications/{}/accept", notification_id)) }
}

#[derive(Serialize, Deserialize)]
struct HideNotification {}

impl RestPath<String> for HideNotification {
    fn get_path(notification_id: String) -> Result<String, Error> { Ok(format!("/api/1/auth/user/notifications/{}/hide", notification_id)) }
}

#[derive(Serialize, Deserialize)]
struct InviteNotification {
    #[serde(rename = "worldId")]
    world_id: String
}

struct VrcApi {
    client: RestClient
}

impl VrcApi {
    pub fn new(api_key: &str, username: &str, password: &str) -> Result<VrcApi, Error> {
        let mut client = RestClient::new("https://vrchat.com")?;
        let mut api_key_cookie = Cookie::new();
        api_key_cookie.set("apiKey", api_key.to_string());
        client.set_auth(username, password);
        client.set_header(api_key_cookie);

        Ok(VrcApi {
            client
        })
    }

    pub fn get_notifications(&mut self, notification_type: NotificationType) -> Result<NotificationList, Error> {
        match notification_type {
            NotificationType::All => self.client.get(()),
            _ => self.client.get_with((), &[("type", &notification_type.to_string())])
        }
    }

    pub fn accept_friend_request(&mut self, notification_id: String) -> Result<(), Error> {
        self.client.put(notification_id, &AcceptFriendRequest {})
    }

    pub fn hide_notification(&mut self, notification_id: String) -> Result<(), Error> {
        self.client.put(notification_id, &HideNotification {})
    }

    pub fn invite_user(&mut self, user_id: String, instance: InstanceId, message: String) -> Result<(), Error> {
        let invite = InviteNotification {
            world_id: instance.to_string()
        };
        self.send_notification(user_id, NotificationType::Invite, &invite, message)
    }

    fn send_notification<T: ?Sized>(&mut self, user_id: String, message_type: NotificationType, details: &T, message: String) -> Result<(), Error>
        where
            T: serde::Serialize
    {
        self.client.post(user_id, &Notification {
            message_type,
            details: serde_json::to_string(details).map_err(|_| Error::ParseError)?,
            message,
        })
    }
}

fn accept_all(api: &mut VrcApi) -> Result<(), Error> {
    let result = api.get_notifications(NotificationType::FriendRequest)?;
    match result {
        NotificationList::Array(requests) => for request in requests {
            println!("accepting friend request from {}", request.sender_user_name);
            api.accept_friend_request(request.id)?
        }
    }
    Ok(())
}

fn handle_invite_requests(api: &mut VrcApi) -> Result<(), Error> {
    let result = api.get_notifications(NotificationType::RequestInvite)?;
    match result {
        NotificationList::Array(requests) => for request in requests {
            println!("handling invite request from {}", request.sender_user_name);
            api.hide_notification(request.id)?;
            let instance: InstanceId = serde_json::from_str(&request.details).map_err(|_| Error::ParseError)?;
            api.invite_user(request.sender_user_id, instance, request.message)?
        }
    }
    Ok(())
}

fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 5 {
        println!("Usage {} <api_key> <username> <password> <mode>", args[0])
    } else {
        let mut api = VrcApi::new(&args[1], &args[2], &args[3]).unwrap();

        let mode = args[4].to_string();

        let five_seconds = time::Duration::from_secs(5);

        match &*mode {
            "accept" => {
                loop {
                    let result = accept_all(&mut api);
                    match result {
                        Err(e) => println!("error while running loop {:?}", e),
                        Ok(_) => {}
                    }
                    thread::sleep(five_seconds);
                }
            }
            "invite" => {
                loop {
                    let result = handle_invite_requests(&mut api);
                    match result {
                        Err(e) => println!("error while running loop {:?}", e),
                        Ok(_) => {}
                    }
                    thread::sleep(five_seconds);
                }
            }
            mode => println!("unrecognized mode {}, supported modes: accept, invite", mode)
        }
    }
}
