use sqlx::SqlitePool;

use crate::models::User;
use crate::prelude::*;

/// Repository for user database operations.
/// Uses the repository pattern to abstract database access.
pub struct UserRepository<'a> {
    db: &'a SqlitePool,
}

impl<'a> UserRepository<'a> {
    /// Create a new UserRepository with a database connection pool.
    pub fn new(db: &'a SqlitePool) -> Self {
        Self { db }
    }

    /// Create a new user in the database.
    pub async fn create(&self, username: &str, password: &str, admin: bool) -> Result<User> {
        let user = User::new(0, username, password, admin)?;

        let result = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO users (username, password_hash, admin)
            VALUES (?, ?, ?)
            RETURNING id
            "#,
        )
        .bind(&user.username)
        .bind(&user.password_hash)
        .bind(admin)
        .fetch_one(self.db)
        .await?;

        Ok(User {
            id: result,
            username: user.username,
            password_hash: user.password_hash,
            admin,
        })
    }

    /// Find a user by their ID.
    pub async fn find_by_id(&self, id: i64) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, username, password_hash, admin
            FROM users
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(self.db)
        .await?;

        Ok(user)
    }

    /// Find a user by their username.
    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, username, password_hash, admin
            FROM users
            WHERE username = ?
            "#,
        )
        .bind(username)
        .fetch_optional(self.db)
        .await?;

        Ok(user)
    }

    /// Get all users from the database.
    pub async fn find_all(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT id, username, password_hash, admin
            FROM users
            ORDER BY id
            "#,
        )
        .fetch_all(self.db)
        .await?;

        Ok(users)
    }

    /// Update a user's information.
    pub async fn update(&self, id: i64, username: &str, admin: bool) -> Result<Option<User>> {
        let result = sqlx::query(
            r#"
            UPDATE users
            SET username = ?, admin = ?, updated_at = datetime('now')
            WHERE id = ?
            "#,
        )
        .bind(username)
        .bind(admin)
        .bind(id)
        .execute(self.db)
        .await?;

        if result.rows_affected() == 0 {
            return Ok(None);
        }

        self.find_by_id(id).await
    }

    /// Update a user's password.
    pub async fn update_password(&self, id: i64, password: &str) -> Result<bool> {
        // Create a temporary user to hash the password
        let temp_user = User::new(0, "", password, false)?;

        let result = sqlx::query(
            r#"
            UPDATE users
            SET password_hash = ?, updated_at = datetime('now')
            WHERE id = ?
            "#,
        )
        .bind(&temp_user.password_hash)
        .bind(id)
        .execute(self.db)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Delete a user by their ID.
    pub async fn delete(&self, id: i64) -> Result<bool> {
        let result = sqlx::query(
            r#"
            DELETE FROM users
            WHERE id = ?
            "#,
        )
        .bind(id)
        .execute(self.db)
        .await?;

        Ok(result.rows_affected() > 0)
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
    async fn test_create_user() {
        let pool = setup_test_db().await;
        let repo = UserRepository::new(&pool);

        let user = repo.create("testuser", "password123", false).await.unwrap();

        assert_eq!(user.username, "testuser");
        assert!(!user.admin);
        assert!(user.id > 0);
    }

    #[tokio::test]
    async fn test_find_by_id() {
        let pool = setup_test_db().await;
        let repo = UserRepository::new(&pool);

        let created = repo.create("testuser", "password123", false).await.unwrap();
        let found = repo.find_by_id(created.id).await.unwrap();

        assert!(found.is_some());
        assert_eq!(found.unwrap().username, "testuser");
    }

    #[tokio::test]
    async fn test_find_by_username() {
        let pool = setup_test_db().await;
        let repo = UserRepository::new(&pool);

        repo.create("testuser", "password123", false).await.unwrap();
        let found = repo.find_by_username("testuser").await.unwrap();

        assert!(found.is_some());
        assert_eq!(found.unwrap().username, "testuser");
    }

    #[tokio::test]
    async fn test_update_user() {
        let pool = setup_test_db().await;
        let repo = UserRepository::new(&pool);

        let created = repo.create("testuser", "password123", false).await.unwrap();
        let updated = repo.update(created.id, "newusername", true).await.unwrap();

        assert!(updated.is_some());
        let updated = updated.unwrap();
        assert_eq!(updated.username, "newusername");
        assert!(updated.admin);
    }

    #[tokio::test]
    async fn test_delete_user() {
        let pool = setup_test_db().await;
        let repo = UserRepository::new(&pool);

        let created = repo.create("testuser", "password123", false).await.unwrap();
        let deleted = repo.delete(created.id).await.unwrap();

        assert!(deleted);

        let found = repo.find_by_id(created.id).await.unwrap();
        assert!(found.is_none());
    }
}
