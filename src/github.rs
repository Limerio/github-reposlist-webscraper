pub struct Github {}

impl Github {
    pub fn get_url(username: &str, tab: bool) -> String {
        format!(
            "https://github.com/{}{}",
            username,
            if tab {
                "?tab=repositories".to_string()
            } else {
                "".to_string()
            }
        )
    }

    pub async fn get_page(username: &str) -> String {
        reqwest::get(Github::get_url(username, true))
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    }
}
