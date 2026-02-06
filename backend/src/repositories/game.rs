use crate::models::Game;
use crate::prelude::*;
use sqlx::SqlitePool;

/// Repository for game database operations.
pub struct GameRepository<'a> {
    db: &'a SqlitePool,
}

impl<'a> GameRepository<'a> {
    /// Create a new GameRepository with a database connection pool.
    pub fn new(db: &'a SqlitePool) -> Self {
        Self { db }
    }

    /// Get all games from the database.
    pub async fn find_all(&self) -> Result<Vec<Game>> {
        let games = sqlx::query_as!(
            Game,
            r#"
            SELECT id as "id!", name, wasm_filename
            FROM games
            ORDER BY name
            "#,
        )
        .fetch_all(self.db)
        .await?;

        Ok(games)
    }

    /// Find a game by its ID.
    pub async fn find_by_id(&self, id: i64) -> Result<Option<Game>> {
        let game = sqlx::query_as!(
            Game,
            r#"
            SELECT id as "id!", name, wasm_filename
            FROM games
            WHERE id = ?
            "#,
            id,
        )
        .fetch_optional(self.db)
        .await?;

        Ok(game)
    }

    /// Find a game by its WASM filename.
    pub async fn find_by_wasm_filename(&self, wasm_filename: &str) -> Result<Option<Game>> {
        let game = sqlx::query_as!(
            Game,
            r#"
            SELECT id as "id!", name, wasm_filename
            FROM games
            WHERE wasm_filename = ?
            "#,
            wasm_filename,
        )
        .fetch_optional(self.db)
        .await?;

        Ok(game)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create test database");

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        pool
    }

    #[tokio::test]
    async fn test_find_all_returns_seeded_games() {
        let pool = setup_test_db().await;
        let repo = GameRepository::new(&pool);

        let games = repo.find_all().await.expect("Failed to find games");

        assert_eq!(games.len(), 2);
        assert!(games.iter().any(|g| g.name == "robotsumo"));
        assert!(games.iter().any(|g| g.name == "snake"));
    }

    #[tokio::test]
    async fn test_find_by_id() {
        let pool = setup_test_db().await;
        let repo = GameRepository::new(&pool);

        let game = repo.find_by_id(1).await.expect("Failed to find game");
        assert!(game.is_some());

        let game = repo.find_by_id(999).await.expect("Failed to find game");
        assert!(game.is_none());
    }

    #[tokio::test]
    async fn test_find_by_wasm_filename() {
        let pool = setup_test_db().await;
        let repo = GameRepository::new(&pool);

        let game = repo
            .find_by_wasm_filename("robotsumo")
            .await
            .expect("Failed to find game");
        assert!(game.is_some());
        assert_eq!(game.unwrap().name, "robotsumo");

        let game = repo
            .find_by_wasm_filename("nonexistent")
            .await
            .expect("Failed to find game");
        assert!(game.is_none());
    }
}
