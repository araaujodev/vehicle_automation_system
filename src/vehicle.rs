//! Vehicle controller module
//!
//! Responsible for maintaining vehicle state and executing commands.
//! In a real implementation, this would interface with vehicle hardware through GPIO.

use std::{thread, time::Duration};
use crate::command::Command;
use crate::error::SystemError;

/// Represents the state of the vehicle
#[derive(Debug)]
pub struct VehicleState {
    /// Whether the vehicle is running
    pub engine_on: bool,
    /// Whether the headlights are on
    pub headlights_on: bool,
    /// Whether the trunk is open
    pub trunk_open: bool,
}

/// Controls vehicle operations and maintains state
pub struct VehicleController {
    /// Current state of the vehicle
    state: VehicleState,
}

impl VehicleController {
    /// Create a new vehicle controller with initial state
    pub fn new() -> Self {
        VehicleController {
            state: VehicleState {
                engine_on: false,
                headlights_on: false,
                trunk_open: false,
            },
        }
    }
    
    /// Execute a command on the vehicle
    pub fn execute_command(&mut self, command: &Command) -> Result<String, SystemError> {
        match command.name.as_str() {
            "ligar" => self.turn_on(),
            "desligar" => self.turn_off(),
            "acender_farol" => self.headlights_on(),
            "apagar_farol" => self.headlights_off(),
            "abrir_porta_malas" => self.open_trunk(),
            "buzinar" => {
                // Check for optional duration parameter
                let duration = if !command.params.is_empty() {
                    match command.params[0].parse::<u64>() {
                        Ok(secs) if secs > 0 && secs <= 10 => secs,
                        _ => return Err(SystemError::InvalidCommand(
                            "Buzina: duração deve ser entre 1 e 10 segundos".to_string()
                        )),
                    }
                } else {
                    1 // Default to 1 second
                };
                
                self.honk(duration)
            },
            _ => Err(SystemError::InvalidCommand(format!("Command not implemented: {}", command.name))),
        }
    }
    
    /// Turn on the vehicle
    /// 
    /// GPIO implementation would activate the ignition relay on pin 17
    fn turn_on(&mut self) -> Result<String, SystemError> {
        if self.state.engine_on {
            return Err(SystemError::VehicleOperation("O veículo já está ligado".to_string()));
        }
        
        // In a real implementation, this would activate GPIO pin 17
        println!("Simulation: Activating ignition relay on GPIO pin 17");
        
        self.state.engine_on = true;
        Ok("Veículo ligado com sucesso".to_string())
    }
    
    /// Turn off the vehicle
    /// 
    /// GPIO implementation would deactivate the ignition relay on pin 17
    fn turn_off(&mut self) -> Result<String, SystemError> {
        if !self.state.engine_on {
            return Err(SystemError::VehicleOperation("O veículo já está desligado".to_string()));
        }
        
        // In a real implementation, this would deactivate GPIO pin 17
        println!("Simulation: Deactivating ignition relay on GPIO pin 17");
        
        self.state.engine_on = false;
        
        // Also turn off headlights when turning off the vehicle
        if self.state.headlights_on {
            self.state.headlights_on = false;
            println!("Simulation: Automatically turning off headlights on GPIO pin 18");
        }
        
        Ok("Veículo desligado com sucesso".to_string())
    }
    
    /// Turn on the headlights
    /// 
    /// GPIO implementation would activate the headlight circuit on pin 18
    fn headlights_on(&mut self) -> Result<String, SystemError> {
        if !self.state.engine_on {
            return Err(SystemError::VehicleOperation(
                "Não é possível acender os faróis com o veículo desligado".to_string()
            ));
        }
        
        if self.state.headlights_on {
            return Err(SystemError::VehicleOperation("Os faróis já estão acesos".to_string()));
        }
        
        // In a real implementation, this would activate GPIO pin 18
        println!("Simulation: Activating headlight circuit on GPIO pin 18");
        
        self.state.headlights_on = true;
        Ok("Faróis acesos com sucesso".to_string())
    }
    
    /// Turn off the headlights
    /// 
    /// GPIO implementation would deactivate the headlight circuit on pin 18
    fn headlights_off(&mut self) -> Result<String, SystemError> {
        if !self.state.engine_on {
            return Err(SystemError::VehicleOperation(
                "Não é possível apagar os faróis com o veículo desligado".to_string()
            ));
        }
        
        if !self.state.headlights_on {
            return Err(SystemError::VehicleOperation("Os faróis já estão apagados".to_string()));
        }
        
        // In a real implementation, this would deactivate GPIO pin 18
        println!("Simulation: Deactivating headlight circuit on GPIO pin 18");
        
        self.state.headlights_on = false;
        Ok("Faróis apagados com sucesso".to_string())
    }
    
    /// Open the trunk
    /// 
    /// GPIO implementation would activate the trunk actuator on pin 22 for a short duration
    fn open_trunk(&mut self) -> Result<String, SystemError> {
        if self.state.trunk_open {
            return Err(SystemError::VehicleOperation("O porta-malas já está aberto".to_string()));
        }
        
        // In a real implementation, this would activate GPIO pin 22
        println!("Simulation: Activating trunk actuator on GPIO pin 22");
        
        // In a real scenario, we would activate the actuator for a short time
        // then deactivate it, as the actuator just needs a pulse to trigger
        println!("Simulation: Waiting 1 second for actuator");
        thread::sleep(Duration::from_secs(1));
        println!("Simulation: Deactivating trunk actuator");
        
        self.state.trunk_open = true;
        Ok("Porta-malas aberto com sucesso".to_string())
    }
    
    /// Honk the horn for the specified duration in seconds
    /// 
    /// GPIO implementation would activate the horn relay on pin 23
    fn honk(&self, duration: u64) -> Result<String, SystemError> {
        if !self.state.engine_on {
            return Err(SystemError::VehicleOperation(
                "Não é possível buzinar com o veículo desligado".to_string()
            ));
        }
        
        // In a real implementation, this would activate GPIO pin 23
        println!("Simulation: Activating horn on GPIO pin 23");
        
        // Simulate a delay for the horn duration
        println!("Simulation: Horn active for {} seconds", duration);
        thread::sleep(Duration::from_secs(duration));
        
        // In a real implementation, this would deactivate GPIO pin 23
        println!("Simulation: Deactivating horn");
        
        Ok(format!("Buzina acionada por {} segundos", duration))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::Command;

    #[test]
    fn test_turn_on() {
        let mut controller = VehicleController::new();
        
        // Should be able to turn on when off
        assert!(controller.turn_on().is_ok());
        assert!(controller.state.engine_on);
        
        // Should fail when already on
        assert!(controller.turn_on().is_err());
    }
    
    #[test]
    fn test_turn_off() {
        let mut controller = VehicleController::new();
        
        // Should fail when already off
        assert!(controller.turn_off().is_err());
        
        // Turn on first
        controller.turn_on().unwrap();
        
        // Now should be able to turn off
        assert!(controller.turn_off().is_ok());
        assert!(!controller.state.engine_on);
    }
    
    #[test]
    fn test_headlights() {
        let mut controller = VehicleController::new();
        
        // Can't use headlights when vehicle is off
        assert!(controller.headlights_on().is_err());
        
        // Turn on vehicle
        controller.turn_on().unwrap();
        
        // Should be able to turn on headlights now
        assert!(controller.headlights_on().is_ok());
        assert!(controller.state.headlights_on);
        
        // Should fail when already on
        assert!(controller.headlights_on().is_err());
        
        // Should be able to turn off
        assert!(controller.headlights_off().is_ok());
        assert!(!controller.state.headlights_on);
        
        // Should fail when already off
        assert!(controller.headlights_off().is_err());
        
        // Turning off vehicle should also turn off headlights
        controller.headlights_on().unwrap();
        assert!(controller.state.headlights_on);
        controller.turn_off().unwrap();
        assert!(!controller.state.headlights_on);
    }
    
    #[test]
    fn test_trunk() {
        let mut controller = VehicleController::new();
        
        // Should be able to open trunk regardless of engine state
        assert!(controller.open_trunk().is_ok());
        assert!(controller.state.trunk_open);
        
        // Should fail when already open
        assert!(controller.open_trunk().is_err());
    }
    
    #[test]
    fn test_honk() {
        let mut controller = VehicleController::new();
        
        // Can't honk when vehicle is off
        assert!(controller.honk(1).is_err());
        
        // Turn on vehicle
        controller.turn_on().unwrap();
        
        // Should be able to honk now
        assert!(controller.honk(1).is_ok());
        
        // Try with invalid duration via command
        let cmd = Command::new("buzinar".to_string(), vec!["0".to_string()]);
        assert!(controller.execute_command(&cmd).is_err());
        
        let cmd = Command::new("buzinar".to_string(), vec!["11".to_string()]);
        assert!(controller.execute_command(&cmd).is_err());
    }
}