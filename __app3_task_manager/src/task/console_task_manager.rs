use crate::task::task_manager::TasksManager;
use crate::task::task::Task;

use std::io::Write;


pub struct ConsoleManager {
    tasks_manager: TasksManager,
    menu_options: Vec<String>,
}

impl ConsoleManager {
    pub fn new() -> Self {
        Self {
            tasks_manager: TasksManager::new(),
            menu_options: vec![
                "Add task".to_owned(),
                "Find task".to_owned(),
                "Edit task".to_owned(),
                "Remove task".to_owned(),
                "Print task".to_owned(),
                "Store tasks to file".to_owned(),
                "Read tasks from file".to_owned(),
            ]
        }
    }

    pub fn print_menu(&self) {
        for (index, menu_option) in self.menu_options.iter().enumerate() {
            println!("{}. {}", index + 1, menu_option);
        }
    }

   pub fn input(query: &str) -> std::io::Result<String> {
        print!("{query}");
        std::io::stdout().flush()?; //return result

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;

        Ok(buffer.trim().to_owned())
    }

   pub fn process_command(&mut self) {
        match Self::input("Enter command [1..7] or 0 for back to Menu: ") {
            Ok(command) => {
                match command.as_str() {
                    "1" => {
                        self.tasks_manager.add_task(Task::new_from_console());
                    }

                    "2" => {
                        let name = match Self::input("Enter task name to find: ") {
                            Ok(name) => name,
                            Err(err) => {
                                println!("Error getting user input: {}", err);
                                return;
                            }
                        };

                        match self.tasks_manager.find_task(name.as_str()) {
                            None => println!("task with name \"{}\" doesn't exist", name),
                            Some(index) => {
                                println!("task found!");
                                self.tasks_manager.tasks.get(index).unwrap().print_task();
                            }
                        }
                    }

                    "3" => {
                        let name = match Self::input("Enter task name to edit: ") {
                            Ok(name) => name,
                            Err(err) => {
                                println!("Error getting user input: {}", err);
                                return;
                            }
                        };

                        match self.tasks_manager.edit_task(name.as_str(), Task::new_from_console()) {
                            Ok(msg) => println!("{}", msg),
                            Err(msg) => println!("{}", msg),
                        }
                    }

                    "4" => {
                        let name = match Self::input("Enter task name to remove: ") {
                            Ok(name) => name,
                            Err(err) => {
                                println!("Error getting user input: {}", err);
                                return;
                            }
                        };

                        match self.tasks_manager.remove_task(name.as_str()) {
                            Ok(msg) => println!("{}", msg),
                            Err(msg) => println!("{}", msg),
                        }
                    }

                    "5" => {
                        self.tasks_manager.print_tasks();
                    }

                    "6" => {
                        let filename = match Self::input("Enter file name to store: ") {
                            Ok(filename) => filename,
                            Err(err) => {
                                println!("Error getting file name input: {}", err);
                                return;
                            }
                        };
                        match self.tasks_manager.store_to_file(filename.as_str()) {
                            Ok(msg) => println!("{}", msg),
                            Err(msg) => println!("{}", msg),
                        }
                    },

                    "7" => {
                        let filename = match Self::input("Enter file name to read: ") {
                            Ok(filename) => filename,
                            Err(err) => {
                                println!("Error getting file name input: {}", err);
                                return;
                            }
                        };
                        match self.tasks_manager.read_from_file(filename.as_str()) {
                            Ok(msg) => println!("{}", msg),
                            Err(msg) => println!("{}", msg),
                        }
                    }
                    "0" => {
                        self.print_menu();
                    }

                    &_ => println!("Unexpect command :D")
                }
            }
            Err(err) => println!("Error getting user input: {err}")
        }
    }
}