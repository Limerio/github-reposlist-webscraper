use sqlx::{Executor, FromRow, Pool, Sqlite};

#[derive(Clone, FromRow, Debug, PartialEq)]
pub struct Repo {
    pub id: String,
    pub link: String,
    pub main_lang: String,
    pub status: String,
}

impl Repo {
    pub async fn get_all(conn: Pool<Sqlite>) -> Vec<Self> {
        sqlx::query_as::<_, Self>("SELECT * FROM repos")
            .fetch_all(&conn)
            .await
            .unwrap()
    }

    pub async fn update(self, conn: Pool<Sqlite>) {
        let formated = format!(
            "INSERT INTO repos (id, link, main_lang, status) VALUES ('{}', '{}', '{}', '{}') ",
            self.id, self.link, self.main_lang, self.status
        );

        conn.execute(&*formated).await.unwrap();
    }
}
