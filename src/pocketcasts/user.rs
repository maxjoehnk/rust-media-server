use pocketcasts::PocketcastPodcast;

use reqwest::Client;
use reqwest::header;

#[derive(Deserialize)]
pub struct PocketcastUser {
    email: String,
    password: String,
    pub session: Option<String>
}

impl PocketcastUser {
    pub fn new(email: &'static str, password: &'static str) -> PocketcastUser {
        PocketcastUser {
            email: email.to_string(),
            password: password.to_string(),
            session: None
        }
    }

    pub fn login(&mut self) {
        let uri = "https://play.pocketcasts.com/users/sign_in";
        let body = [
            ("[user]email", self.email.as_str()),
            ("[user]password", self.password.as_str())
        ];

        let client = Client::new();
        let res = client.post(uri)
            .form(&body)
            .send()
            .unwrap();

        let cookies = res.headers().get::<header::SetCookie>().unwrap();
    }

    pub fn get_subscriptions(&self) -> Option<Vec<PocketcastPodcast>> {
        let uri = "https://play.pocketcasts.com/web/podcasts/all.json";
        let client = Client::new();
        let session = self.session.clone().expect("Login first");
        let mut cookies = header::Cookie::new();
        cookies.set("_social_session", session);
        let mut res = client.post(uri)
            .header(cookies)
            .send()
            .unwrap();

        if !res.status().is_success() {
            return None;
        }

        let res: SubscriptionsResponse = res.json().unwrap();

        Some(res.podcasts)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SubscriptionsResponse {
    podcasts: Vec<PocketcastPodcast>
}