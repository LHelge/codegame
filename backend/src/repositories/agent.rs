use crate::models::Agent;
use crate::prelude::*;
use sqlx::SqlitePool;

/// Repository for agent database operations.
pub struct AgentRepository<'a> {
    db: &'a SqlitePool,
}

impl<'a> AgentRepository<'a> {
    /// Create a new AgentRepository with a database connection pool.
    pub fn new(db: &'a SqlitePool) -> Self {
        Self { db }
    }

    /// Create a new agent for a user.
    pub async fn create(
        &self,
        user_id: i64,
        game_id: i64,
        name: &str,
        code: &str,
    ) -> Result<Agent> {
        let agent = sqlx::query_as!(
            Agent,
            r#"
            INSERT INTO agents (user_id, game_id, name, code)
            VALUES (?, ?, ?, ?)
            RETURNING 
                id as "id!",
                user_id as "user_id!",
                game_id as "game_id!",
                name,
                code,
                created_at,
                updated_at
            "#,
            user_id,
            game_id,
            name,
            code,
        )
        .fetch_one(self.db)
        .await?;

        Ok(agent)
    }

    /// Find an agent by ID, only if it belongs to the specified user.
    pub async fn find_by_id(&self, id: i64, user_id: i64) -> Result<Option<Agent>> {
        let agent = sqlx::query_as!(
            Agent,
            r#"
            SELECT 
                id as "id!",
                user_id as "user_id!",
                game_id as "game_id!",
                name,
                code,
                created_at,
                updated_at
            FROM agents
            WHERE id = ? AND user_id = ?
            "#,
            id,
            user_id,
        )
        .fetch_optional(self.db)
        .await?;

        Ok(agent)
    }

    /// Find all agents for a user in a specific game.
    pub async fn find_by_user_and_game(&self, user_id: i64, game_id: i64) -> Result<Vec<Agent>> {
        let agents = sqlx::query_as!(
            Agent,
            r#"
            SELECT 
                id as "id!",
                user_id as "user_id!",
                game_id as "game_id!",
                name,
                code,
                created_at,
                updated_at
            FROM agents
            WHERE user_id = ? AND game_id = ?
            ORDER BY name
            "#,
            user_id,
            game_id,
        )
        .fetch_all(self.db)
        .await?;

        Ok(agents)
    }

    /// Update an agent's name and/or code.
    pub async fn update(
        &self,
        id: i64,
        user_id: i64,
        name: Option<&str>,
        code: Option<&str>,
    ) -> Result<Option<Agent>> {
        // First check the agent exists and belongs to user
        let existing = self.find_by_id(id, user_id).await?;
        if existing.is_none() {
            return Ok(None);
        }
        let existing = existing.unwrap();

        // Determine new values
        let new_name = name.unwrap_or(&existing.name);
        let new_code = code.unwrap_or(&existing.code);

        let agent = sqlx::query_as!(
            Agent,
            r#"
            UPDATE agents
            SET name = ?, code = ?, updated_at = datetime('now')
            WHERE id = ? AND user_id = ?
            RETURNING 
                id as "id!",
                user_id as "user_id!",
                game_id as "game_id!",
                name,
                code,
                created_at,
                updated_at
            "#,
            new_name,
            new_code,
            id,
            user_id,
        )
        .fetch_one(self.db)
        .await?;

        Ok(Some(agent))
    }

    /// Delete an agent by ID, only if it belongs to the specified user.
    pub async fn delete(&self, id: i64, user_id: i64) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM agents
            WHERE id = ? AND user_id = ?
            "#,
            id,
            user_id,
        )
        .execute(self.db)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{GameRepository, UserRepository};
    use sqlx::SqlitePool;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();
        pool
    }

    async fn create_test_user(pool: &SqlitePool) -> i64 {
        let repo = UserRepository::new(pool);
        let user = repo
            .create("testuser", "TestPass123!", false)
            .await
            .expect("Failed to create user");
        user.id
    }

    /// Get the robotsumo game ID (seeded by migrations)
    async fn get_test_game_id(pool: &SqlitePool) -> i64 {
        let repo = GameRepository::new(pool);
        let game = repo
            .find_by_name("robotsumo")
            .await
            .expect("Failed to query game")
            .expect("robotsumo should exist");
        game.id
    }

    #[tokio::test]
    async fn test_create_agent() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let game_id = get_test_game_id(&pool).await;

        let repo = AgentRepository::new(&pool);
        let agent = repo
            .create(user_id, game_id, "My Agent", "-- Lua code")
            .await
            .expect("Failed to create agent");

        assert_eq!(agent.name, "My Agent");
        assert_eq!(agent.code, "-- Lua code");
        assert_eq!(agent.user_id, user_id);
        assert_eq!(agent.game_id, game_id);
    }

    #[tokio::test]
    async fn test_find_by_id() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let game_id = get_test_game_id(&pool).await;

        let repo = AgentRepository::new(&pool);
        let created = repo
            .create(user_id, game_id, "Agent 1", "code")
            .await
            .unwrap();

        // Should find with correct user_id
        let found = repo.find_by_id(created.id, user_id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Agent 1");

        // Should not find with wrong user_id
        let not_found = repo.find_by_id(created.id, user_id + 999).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_find_by_user_and_game() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let game_id = get_test_game_id(&pool).await;

        let repo = AgentRepository::new(&pool);
        repo.create(user_id, game_id, "Agent A", "code a")
            .await
            .unwrap();
        repo.create(user_id, game_id, "Agent B", "code b")
            .await
            .unwrap();

        let agents = repo.find_by_user_and_game(user_id, game_id).await.unwrap();
        assert_eq!(agents.len(), 2);
        assert_eq!(agents[0].name, "Agent A");
        assert_eq!(agents[1].name, "Agent B");
    }

    #[tokio::test]
    async fn test_update_agent() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let game_id = get_test_game_id(&pool).await;

        let repo = AgentRepository::new(&pool);
        let created = repo
            .create(user_id, game_id, "Original", "original code")
            .await
            .unwrap();

        // Update name only
        let updated = repo
            .update(created.id, user_id, Some("Renamed"), None)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(updated.name, "Renamed");
        assert_eq!(updated.code, "original code");

        // Update code
        let updated = repo
            .update(created.id, user_id, None, Some("new code"))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(updated.code, "new code");
    }

    #[tokio::test]
    async fn test_delete_agent() {
        let pool = setup_test_db().await;
        let user_id = create_test_user(&pool).await;
        let game_id = get_test_game_id(&pool).await;

        let repo = AgentRepository::new(&pool);
        let created = repo
            .create(user_id, game_id, "To Delete", "code")
            .await
            .unwrap();

        // Delete with correct user
        let deleted = repo.delete(created.id, user_id).await.unwrap();
        assert!(deleted);

        // Verify it's gone
        let found = repo.find_by_id(created.id, user_id).await.unwrap();
        assert!(found.is_none());

        // Delete with wrong user should return false
        let created2 = repo
            .create(user_id, game_id, "Another", "code")
            .await
            .unwrap();
        let not_deleted = repo.delete(created2.id, user_id + 999).await.unwrap();
        assert!(!not_deleted);
    }
}
