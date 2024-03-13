use crate::task::task::Task;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct TasksManager {
    pub(crate) tasks: Vec<Task>
}

impl TasksManager {
    pub fn new() -> Self {
        Self {
            tasks: vec![]
        }
    }

    pub(crate) fn print_tasks(&self) {
        for task in &self.tasks {
            task.print_task();
        }
    }

    pub(crate) fn add_task(&mut self, task: Task) {
        self.tasks.push(task)
    }

    pub(crate) fn remove_task(&mut self, name: &str) -> Result<String, String> {
        if let Some(index) = self.find_task(name) {
            self.tasks.remove(index);
            Ok(format!("task \"{}\" removed successfully", name))
        } else {
            Err(format!("task with name \"{}\" doesnt exist", name))
        }
    }

    pub(crate) fn find_task(&self, name: &str) -> Option<usize>{
        self.tasks.iter().position(|task | task.name == name)
    }

    pub(crate) fn edit_task(&mut self, name: &str, updated_task: Task) -> Result<String, String>{
        if let Some(index) = self.find_task(name) {
            match self.tasks.get_mut(index) {
                None => Err("Error borrowing task".to_owned()),
                Some(task) => {
                    task.name = updated_task.name;
                    task.description = updated_task.description;
                    task.priority = updated_task.priority;

                    Ok(format!("task \"{}\" updated successfully", name))
                }
            }
        } else {
            Err(format!("task with name \"{}\" doesnt exist", name))
        }
    }

    pub(crate) fn store_to_file(&self, filename: &str) -> Result<String, String> {
        if !Path::new(filename).exists() {
            let file = match File::create(filename) {
                Ok(file) => file,
                Err(err) => return Err(format!("Error creating file: {err}"))
            };
            match serde_json::to_writer(&file, &self.tasks) {
                Ok(_) => Ok("Data stored successfully".to_owned()),
                Err(err) => Err(format!("Error saving data: {err}"))
            }
        } else {
            Err("File \"{filename}\" already exist".to_owned())
        }
    }

    pub(crate) fn read_from_file(&mut self, filename: &str) -> Result<String, String> {
        if Path::new(filename).exists() {
            let file = match File::open(filename) {
                Ok(file) => file,
                Err(err) => return Err(format!("Error opening file: {err}"))
            };

            let reader = BufReader::new(file);
            self.tasks = match serde_json::from_reader(reader) {
                Ok(data) => data,
                Err(err) => {
                    return  Err(format!("Error reading data: {err}"))
                }
            };
            Ok("Data read successfully".to_owned())
        } else {
            Err("File \"{filename}\" doesnt exist".to_owned())
        }
    }
}