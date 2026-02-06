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
            SELECT id as "id!", name, display_name
            FROM games
            ORDER BY name
            "#,
        )
        .fetch_all(self.db)
        .await?;

        Ok(games)
    }

    /// Find a game by its unique name.
    pub async fn find_by_name(&self, name: &str) -> Result<Option<Game>> {
        let game = sqlx::query_as!(
            Game,
            r#"
            SELECT id as "id!", name, display_name
            FROM games
            WHERE name = ?
            "#,
            name,
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
    async fn test_find_by_name() {
        let pool = setup_test_db().await;
        let repo = GameRepository::new(&pool);

        let game = repo
            .find_by_name("robotsumo")
            .await
            .expect("Failed to find game");
        assert!(game.is_some());
        assert_eq!(game.unwrap().display_name, "Robot Sumo");

        let game = repo
            .find_by_name("nonexistent")
            .await
            .expect("Failed to find game");
        assert!(game.is_none());
    }
}
