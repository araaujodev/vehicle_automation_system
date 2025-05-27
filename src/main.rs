//! Vehicle Automation System
//! 
//! This system provides a modular architecture for controlling vehicle
//! functions through custom commands. Initially designed to work with
//! terminal input, it can be extended to use GPIO for real hardware integration.

mod command;
mod vehicle;
mod input;
mod output;
mod error;

use std::io;
use command::CommandProcessor;
use vehicle::VehicleController;
use input::InputHandler;
use output::OutputManager;

fn main() -> Result<(), error::SystemError> {
    // Initialize system components
    let mut vehicle = VehicleController::new();
    let command_processor = CommandProcessor::new();
    let input_handler = InputHandler::new();
    let output_manager = OutputManager::new();

    output_manager.print_welcome();

    // Main system loop
    loop {
        // Get command from input
        let input = match input_handler.get_command() {
            Ok(input) => input,
            Err(e) => {
                output_manager.print_error(&format!("Input error: {}", e));
                continue;
            }
        };

        // Check for exit command
        if input.trim() == "exit" {
            output_manager.print_info("Shutting down system...");
            break;
        }

        // Process command
        match command_processor.process(&input) {
            Ok(cmd) => {
                output_manager.print_command_received(&cmd.name);
                
                // Execute vehicle action based on command
                match vehicle.execute_command(&cmd) {
                    Ok(msg) => output_manager.print_success(&msg),
                    Err(e) => output_manager.print_error(&format!("Execution error: {}", e)),
                }
            },
            Err(e) => {
                output_manager.print_error(&format!("Command error: {}", e));
            }
        }
    }

    Ok(())
}