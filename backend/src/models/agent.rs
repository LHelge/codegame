use mlua::Lua;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("Agent name is required.")]
    NameEmpty,

    #[error("Agent name must be at most 50 characters.")]
    NameTooLong,

    #[error("Agent name can only contain letters, numbers, spaces, hyphens, and underscores.")]
    NameInvalidCharacters,

    #[error("Agent code is required.")]
    CodeEmpty,

    #[error("Invalid Lua syntax: {0}")]
    InvalidLuaSyntax(String),
}

type Result<T> = std::result::Result<T, AgentError>;

/// Validates an agent name.
pub fn validate_agent_name(name: &str) -> Result<()> {
    let name = name.trim();

    if name.is_empty() {
        return Err(AgentError::NameEmpty);
    }

    if name.len() > 50 {
        return Err(AgentError::NameTooLong);
    }

    // Allow alphanumeric, spaces, hyphens, and underscores
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == ' ' || c == '-' || c == '_')
    {
        return Err(AgentError::NameInvalidCharacters);
    }

    Ok(())
}

/// Validates agent code - must not be empty and must be valid Lua syntax.
pub fn validate_agent_code(code: &str) -> Result<()> {
    if code.trim().is_empty() {
        return Err(AgentError::CodeEmpty);
    }

    // Validate Lua syntax by attempting to load the code
    let lua = Lua::new();
    lua.load(code).exec().map_err(|e| {
        // Extract the error message, removing the "[string \"...\"]:line:" prefix if present
        let msg = e.to_string();
        let clean_msg = if let Some(pos) = msg.find("]:") {
            msg[pos + 2..].trim().to_string()
        } else {
            msg
        };
        AgentError::InvalidLuaSyntax(clean_msg)
    })?;

    Ok(())
}

/// Represents a user's AI agent for a specific game.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Agent {
    pub id: i64,
    pub user_id: i64,
    pub game_id: i64,
    pub name: String,
    pub code: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Request payload for creating a new agent.
#[derive(Debug, Deserialize)]
pub struct CreateAgentRequest {
    pub game_id: i64,
    pub name: String,
    #[serde(default)]
    pub code: String,
}

/// Request payload for updating an existing agent.
#[derive(Debug, Deserialize)]
pub struct UpdateAgentRequest {
    pub name: Option<String>,
    pub code: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_accepts_simple_name() {
        assert!(validate_agent_name("My Agent").is_ok());
    }

    #[test]
    fn validate_accepts_name_with_numbers() {
        assert!(validate_agent_name("Agent 123").is_ok());
    }

    #[test]
    fn validate_accepts_name_with_hyphens_and_underscores() {
        assert!(validate_agent_name("my-agent_v2").is_ok());
    }

    #[test]
    fn validate_accepts_single_character() {
        assert!(validate_agent_name("A").is_ok());
    }

    #[test]
    fn validate_accepts_50_characters() {
        let name = "a".repeat(50);
        assert!(validate_agent_name(&name).is_ok());
    }

    #[test]
    fn validate_rejects_empty_name() {
        assert!(matches!(
            validate_agent_name(""),
            Err(AgentError::NameEmpty)
        ));
    }

    #[test]
    fn validate_rejects_whitespace_only() {
        assert!(matches!(
            validate_agent_name("   "),
            Err(AgentError::NameEmpty)
        ));
    }

    #[test]
    fn validate_rejects_name_over_50_characters() {
        let name = "a".repeat(51);
        assert!(matches!(
            validate_agent_name(&name),
            Err(AgentError::NameTooLong)
        ));
    }

    #[test]
    fn validate_rejects_special_characters() {
        assert!(matches!(
            validate_agent_name("Agent!"),
            Err(AgentError::NameInvalidCharacters)
        ));
    }

    #[test]
    fn validate_rejects_emoji() {
        assert!(matches!(
            validate_agent_name("Agent 🤖"),
            Err(AgentError::NameInvalidCharacters)
        ));
    }

    #[test]
    fn validate_rejects_newlines() {
        assert!(matches!(
            validate_agent_name("Agent\nName"),
            Err(AgentError::NameInvalidCharacters)
        ));
    }

    #[test]
    fn validate_code_accepts_valid_code() {
        assert!(validate_agent_code("-- Lua comment").is_ok());
    }

    #[test]
    fn validate_code_accepts_valid_lua_function() {
        let code = r#"
            function think()
                local x = 10
                return x + 5
            end
        "#;
        assert!(validate_agent_code(code).is_ok());
    }

    #[test]
    fn validate_code_accepts_valid_lua_with_loops() {
        let code = r#"
            for i = 1, 10 do
                print(i)
            end
        "#;
        assert!(validate_agent_code(code).is_ok());
    }

    #[test]
    fn validate_code_rejects_empty() {
        assert!(matches!(
            validate_agent_code(""),
            Err(AgentError::CodeEmpty)
        ));
    }

    #[test]
    fn validate_code_rejects_whitespace_only() {
        assert!(matches!(
            validate_agent_code("   \n\t  "),
            Err(AgentError::CodeEmpty)
        ));
    }

    #[test]
    fn validate_code_rejects_invalid_syntax() {
        // Missing 'end' keyword
        let code = "function broken()";
        assert!(matches!(
            validate_agent_code(code),
            Err(AgentError::InvalidLuaSyntax(_))
        ));
    }

    #[test]
    fn validate_code_rejects_syntax_error() {
        // Invalid Lua syntax
        let code = "if x then print(x)"; // missing 'end'
        assert!(matches!(
            validate_agent_code(code),
            Err(AgentError::InvalidLuaSyntax(_))
        ));
    }

    #[test]
    fn validate_code_rejects_malformed_expression() {
        let code = "local x = 5 +"; // incomplete expression
        assert!(matches!(
            validate_agent_code(code),
            Err(AgentError::InvalidLuaSyntax(_))
        ));
    }
}
