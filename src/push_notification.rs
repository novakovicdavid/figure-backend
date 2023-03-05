use onesignal_rust_api::*;
use onesignal_rust_api::apis::configuration::Configuration;
use onesignal_rust_api::models::{Notification, StringMap};

#[derive(Debug)]
pub struct PushNotificationService {
    app_id: String,
    config: Configuration,
}

pub fn create_push_notification_service(onesignal_appid: String, onesignal_appkey_token: String, onesignal_userkey_token: String) -> PushNotificationService {
    let mut configuration = Configuration::new();
    configuration.app_key_token = Some(onesignal_appkey_token);
    configuration.user_key_token = Some(onesignal_userkey_token);
    PushNotificationService {
        app_id: onesignal_appid,
        config: configuration
    }
}

impl PushNotificationService {
    fn create_notification(&self) -> Box<Notification> {
        let mut notification = Notification::new(self.app_id.clone());
        let mut string_map = StringMap::new();

        string_map.en = Some(String::from("New Figure got posted!"));
        notification.contents = Some(Box::new(string_map));
        notification.is_chrome_web = Some(true);
        notification.is_any_web = Some(true);
        notification.included_segments = Some(vec![String::from("Subscribed Users")]);

        Box::new(notification)
    }

    pub async fn push_message(&self) {
        let notification = self.create_notification();

        // Send notification to the server
        let create_notification_response = apis::default_api::create_notification(&self.config, *notification).await;

        // Check the result
        if let Ok(ref created_notification) = create_notification_response {
            println!("Created notification id: {}", created_notification.id);
        }

        if let Err(ref created_notification_error) = create_notification_response {
            println!("Created notification error: {}", created_notification_error);
        }
    }
}