//! Command processing module
//!
//! Handles the parsing, validation, and processing of user commands.

use crate::error::SystemError;

/// Represents a processed command with its name and parameters
#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub params: Vec<String>,
}

impl Command {
    /// Create a new command
    pub fn new(name: String, params: Vec<String>) -> Self {
        Command { name, params }
    }
}

/// Processes raw input into structured commands
pub struct CommandProcessor {}

impl CommandProcessor {
    /// Create a new command processor
    pub fn new() -> Self {
        CommandProcessor {}
    }

    /// Process a raw input string into a Command
    pub fn process(&self, input: &str) -> Result<Command, SystemError> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        
        if parts.is_empty() {
            return Err(SystemError::InvalidCommand("Empty command".to_string()));
        }
        
        let name = parts[0].to_lowercase();
        
        // Validate command name
        match name.as_str() {
            "ligar" | "desligar" | "acender_farol" | "apagar_farol" | 
            "abrir_porta_malas" | "buzinar" => {
                // Extract parameters (if any)
                let params = parts[1..]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                
                Ok(Command::new(name, params))
            },
            _ => Err(SystemError::InvalidCommand(format!("Unknown command: {}", name))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_commands() {
        let processor = CommandProcessor::new();
        
        let commands = vec![
            "ligar",
            "desligar",
            "acender_farol",
            "apagar_farol",
            "abrir_porta_malas",
            "buzinar 2",
        ];
        
        for cmd in commands {
            assert!(processor.process(cmd).is_ok());
        }
    }

    #[test]
    fn test_invalid_commands() {
        let processor = CommandProcessor::new();
        
        let commands = vec![
            "",
            "   ",
            "unknown_command",
            "acelerar", // Not implemented yet
        ];
        
        for cmd in commands {
            assert!(processor.process(cmd).is_err());
        }
    }
}