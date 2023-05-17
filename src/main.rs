mod db;
mod github;

use db::Repo;
use dotenv::dotenv;
use github::Github;
use scraper::{Html, Selector};
use sqlx::{migrate::MigrateDatabase, Executor, Sqlite, SqlitePool};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let username = std::env::var("USERNAME").unwrap();
    let db_url = std::env::var("DB_URL").unwrap();

    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        match Sqlite::create_database(&db_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    }

    let conn = SqlitePool::connect(&db_url).await.unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS repos (id VARCHAR(50) PRIMARY KEY NOT NULL, link VARCHAR(250) NOT NULL, main_lang VARCHAR(15) NOT NULL, status VARCHAR(15) NOT NULL)"
    ).await.unwrap();

    let body = Github::get_page(&username).await;

    let document = Html::parse_document(&body);

    let selector_repos_list = Selector::parse("div[id=\"user-repositories-list\"] > ul").unwrap();
    let selector_title = Selector::parse("h3").unwrap();
    let selector_anchor = Selector::parse("a").unwrap();
    let selector_span = Selector::parse("span").unwrap();
    let selector_prog_lang = Selector::parse("span[itemprop=\"programmingLanguage\"]").unwrap();

    let repos_query = Repo::get_all(conn.clone()).await;

    for repos in document.select(&selector_repos_list) {
        for (i, title) in repos.select(&selector_title).enumerate() {
            let repo_lang = {
                let value = repos.select(&selector_prog_lang).nth(i);

                if value.is_none() {
                    "Unknown".to_string()
                } else {
                    value.unwrap().text().collect::<String>()
                }
            };

            let repo_id = title
                .select(&selector_anchor)
                .last()
                .unwrap()
                .text()
                .collect::<String>();

            let repo_status = title
                .select(&selector_span)
                .last()
                .unwrap()
                .text()
                .collect::<String>();

            let repo_data = Repo {
                id: repo_id.trim().to_string(),
                link: format!(
                    "{}/{}",
                    Github::get_url(&username, false),
                    repo_id.trim().to_string()
                ),
                main_lang: repo_lang,
                status: repo_status,
            };

            if !repos_query.contains(&repo_data) {
                repo_data.update(conn.clone()).await;
            }
        }
    }

    let repos = Repo::get_all(conn.clone()).await;

    for repo in repos {
        println!("{:#?}", repo);
    }
}
