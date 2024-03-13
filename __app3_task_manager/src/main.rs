mod task;
use crate::task::console_task_manager::ConsoleManager;


/*
// #[derive(Serialize, Deserialize)]
// enum Priority {
//     Low,
//     Medium,
//     High
// }
//
// impl Priority {
//     fn to_string(&self) -> String {
//         match self {
//             Priority::Low => "Low".to_owned(),
//             Priority::Medium => "Medium".to_owned(),
//             Priority::High => "High".to_owned()
//         }
//     }
// }
// //.................................................................................................
// #[derive(Serialize, Deserialize)]
// struct task {
//     name: String,
//     description: String,
//     priority: Priority,
//     add_time: DateTime<Local>
// }
//
// impl task {
//     fn new(name: String, description: String, priority: Priority) -> Self {
//         Self{ name, description, priority, add_time: Local::now()}
//     }
//
//     fn new_from_console() -> Self {
//         let name = ConsoleManager::input("Enter new task name: ")
//             .expect("Error getting user input");
//         let description = ConsoleManager::input("Enter new task description: ")
//             .expect("Error getting user input");
//         let priority = match ConsoleManager::input("Enter new task priority: ")
//             .expect("Error getting user input")
//             .to_lowercase()
//             .as_str() {
//             "low" => Priority::Low,
//             "medium" => Priority::Medium,
//             "high" => Priority::High,
//             _ => {
//                 println!("Invalid priority, setting to low");
//                 Priority::Low
//             }
//         };
//
//        Self::new(name, description, priority)
//     }
//
//     fn print_task(&self) {
//         println!(
//             "  {}  |  {}  | {}\ndescription: \"{}\"\n",
//             self.name,
//             self.priority.to_string(),
//             self.add_time.format("%d-%m-%Y %H:%M:%S"),
//             self.description,
//         );
//     }
// }
//.......................................................................................
// struct TasksManager {
//     tasks: Vec<task>
// }
//
// impl TasksManager {
//     fn new() -> Self {
//         Self {
//             tasks: vec![]
//         }
//     }
//
//     fn print_tasks(&self) {
//         for task in &self.tasks {
//             task.print_task();
//         }
//     }
//
//     fn add_task(&mut self, task: task) {
//         self.tasks.push(task)
//     }
//
//     fn remove_task(&mut self, name: &str) -> Result<String, String> {
//         if let Some(index) = self.find_task(name) {
//             self.tasks.remove(index);
//             Ok(format!("task \"{}\" removed successfully", name))
//         } else {
//             Err(format!("task with name \"{}\" doesnt exist", name))
//         }
//     }
//
//     fn find_task(&self, name: &str) -> Option<usize>{
//         self.tasks.iter().position(|task | task.name == name)
//     }
//
//     fn edit_task(&mut self, name: &str, updated_task: task) -> Result<String, String>{
//         if let Some(index) = self.find_task(name) {
//             match self.tasks.get_mut(index) {
//                 None => Err("Error borrowing task".to_owned()),
//                 Some(task) => {
//                     task.name = updated_task.name;
//                     task.description = updated_task.description;
//                     task.priority = updated_task.priority;
//
//                     Ok(format!("task \"{}\" updated successfully", name))
//                 }
//             }
//         } else {
//             Err(format!("task with name \"{}\" doesnt exist", name))
//         }
//     }
//
//     fn store_to_file(&self, filename: &str) -> Result<String, String> {
//         if !Path::new(filename).exists() {
//             let file = match File::create(filename) {
//                 Ok(file) => file,
//                 Err(err) => return Err(format!("Error creating file: {err}"))
//             };
//             match serde_json::to_writer(&file, &self.tasks) {
//                 Ok(_) => Ok("Data stored successfully".to_owned()),
//                 Err(err) => Err(format!("Error saving data: {err}"))
//             }
//         } else {
//             Err("File \"{filename}\" already exist".to_owned())
//         }
//     }
//
//     fn read_from_file(&mut self, filename: &str) -> Result<String, String> {
//         if Path::new(filename).exists() {
//             let file = match File::open(filename) {
//                 Ok(file) => file,
//                 Err(err) => return Err(format!("Error opening file: {err}"))
//             };
//
//             let reader = BufReader::new(file);
//             self.tasks = match serde_json::from_reader(reader) {
//                 Ok(data) => data,
//                 Err(err) => {
//                     return  Err(format!("Error reading data: {err}"))
//                 }
//             };
//             Ok("Data read successfully".to_owned())
//         } else {
//             Err("File \"{filename}\" doesnt exist".to_owned())
//         }
//     }
// }
//........................................................................................................
// struct ConsoleManager {
//     tasks_manager: TasksManager,
//     menu_options: Vec<String>,
// }
//
// impl ConsoleManager {
//     fn new() -> Self {
//         Self {
//             tasks_manager: TasksManager::new(),
//             menu_options: vec![
//                 "Add task".to_owned(),
//                 "Find task".to_owned(),
//                 "Edit task".to_owned(),
//                 "Remove task".to_owned(),
//                 "Print task".to_owned(),
//                 "Store tasks to file".to_owned(),
//                 "Read tasks from file".to_owned(),
//             ]
//         }
//     }
//
//     fn print_menu(&self) {
//         for (index, menu_option) in self.menu_options.iter().enumerate() {
//             println!("{}. {}", index + 1, menu_option);
//         }
//     }
//
//     fn input(query: &str) -> std::io::Result<String> {
//         print!("{query}");
//         std::io::stdout().flush()?; //return result
//
//         let mut buffer = String::new();
//         std::io::stdin().read_line(&mut buffer)?;
//
//         Ok(buffer.trim().to_owned())
//     }
//
//     fn process_command(&mut self) {
//         match Self::input("Enter command [1..7]: ") {
//             Ok(command) => {
//                 match command.as_str() {
//                     "1" => {
//                         self.tasks_manager.add_task(task::new_from_console());
//                     }
//
//                     "2" => {
//                         let name = match Self::input("Enter task name to find: ") {
//                             Ok(name) => name,
//                             Err(err) => {
//                                 println!("Error getting user input: {}", err);
//                                 return;
//                             }
//                         };
//
//                         match self.tasks_manager.find_task(name.as_str()) {
//                             None => println!("task with name \"{}\" doesn't exist", name),
//                             Some(index) => {
//                                 println!("task found!");
//                                 self.tasks_manager.tasks.get(index).unwrap().print_task();
//                             }
//                         }
//                     }
//
//                     "3" => {
//                         let name = match Self::input("Enter task name to edit: ") {
//                             Ok(name) => name,
//                             Err(err) => {
//                                 println!("Error getting user input: {}", err);
//                                 return;
//                             }
//                         };
//
//                         match self.tasks_manager.edit_task(name.as_str(), task::new_from_console()) {
//                             Ok(msg) => println!("{}", msg),
//                             Err(msg) => println!("{}", msg),
//                         }
//                     }
//
//                     "4" => {
//                         let name = match Self::input("Enter task name to remove: ") {
//                             Ok(name) => name,
//                             Err(err) => {
//                                 println!("Error getting user input: {}", err);
//                                 return;
//                             }
//                         };
//
//                         match self.tasks_manager.remove_task(name.as_str()) {
//                             Ok(msg) => println!("{}", msg),
//                             Err(msg) => println!("{}", msg),
//                         }
//                     }
//
//                     "5" => {
//                         self.tasks_manager.print_tasks();
//                     }
//
//                     "6" => {
//                         let filename = match Self::input("Enter file name to store: ") {
//                             Ok(filename) => filename,
//                             Err(err) => {
//                                 println!("Error getting file name input: {}", err);
//                                 return;
//                             }
//                         };
//                         match self.tasks_manager.store_to_file(filename.as_str()) {
//                             Ok(msg) => println!("{}", msg),
//                             Err(msg) => println!("{}", msg),
//                         }
//                     },
//
//                     "7" => {
//                         let filename = match Self::input("Enter file name to read: ") {
//                             Ok(filename) => filename,
//                             Err(err) => {
//                                 println!("Error getting file name input: {}", err);
//                                 return;
//                             }
//                         };
//                         match self.tasks_manager.read_from_file(filename.as_str()) {
//                             Ok(msg) => println!("{}", msg),
//                             Err(msg) => println!("{}", msg),
//                         }
//                     },
//
//                     &_ => println!("Unexpect command :D")
//                 }
//             }
//             Err(err) => println!("Error getting user input: {err}")
//         }
//     }
// }
//.....................................................................................

 */
fn main() {
    // let task = task::new(
    //     "To learn Rust".to_owned(),
    //     "I need it..".to_owned(),
    //     Priority::High,
    // );
    // task.print_task();
    let mut manager = ConsoleManager::new();
    manager.print_menu();
    loop {
        manager.process_command();
        println!("\n");
    }
}