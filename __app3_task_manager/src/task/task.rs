use crate::task::console_task_manager::ConsoleManager;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High
}

impl Priority {
   pub fn to_string(&self) -> String {
        match self {
            Priority::Low => "Low".to_owned(),
            Priority::Medium => "Medium".to_owned(),
            Priority::High => "High".to_owned()
        }
    }
}
//.................................................................................................
#[derive(Serialize, Deserialize)]
pub struct Task {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) priority: Priority,
    add_time: DateTime<Local>
}

impl Task {
    pub fn new(name: String, description: String, priority: Priority) -> Self {
        Self{ name, description, priority, add_time: Local::now()}
    }

    pub fn new_from_console() -> Self {
        let name = ConsoleManager::input("Enter new task name: ")
            .expect("Error getting user input");
        let description = ConsoleManager::input("Enter new task description: ")
            .expect("Error getting user input");
        let priority = match ConsoleManager::input("Enter new task priority: ")
            .expect("Error getting user input")
            .to_lowercase()
            .as_str() {
            "low" => Priority::Low,
            "medium" => Priority::Medium,
            "high" => Priority::High,
            _ => {
                println!("Invalid priority, setting to low");
                Priority::Low
            }
        };

        Self::new(name, description, priority)
    }

   pub fn print_task(&self) {
        println!(
            "  {}  |  {}  | {}\ndescription: \"{}\"\n",
            self.name,
            self.priority.to_string(),
            self.add_time.format("%d-%m-%Y %H:%M:%S"),
            self.description,
        );
    }
}