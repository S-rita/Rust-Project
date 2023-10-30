use std::collections::HashMap;
use chrono::NaiveDate;
use std::io::{self, Write};
use std::fs::File;
use std::io::BufRead;

struct Task {
    description: String,
    priority: u32,
    deadline: String,
    completed: bool,
}

impl Task {
    fn new(description: String, priority: u32, deadline: String) -> Task {
        Task {
            description,
            priority,
            deadline,
            completed: false,
        }
    }

    fn complete(&mut self) {
        self.completed = true;
    }

    fn display(&self) {
        let status = if self.completed { "Completed" } else { "Pending" };
        println!(
            "Description: {}\nPriority: {}\nDeadline: {}\nStatus: {}",
            self.description, self.priority, self.deadline, status
        );
    }
}

fn main() {
    let mut task_list: HashMap<usize, Task> = HashMap::new();
    let mut task_id = 1;

    loop {
        println!("---------------------------");
        println!("Main Menu:");
        println!("(1) Add Task");
        println!("(2) Mark Task as Completed");
        println!("(3) View All Tasks");
        println!("(4) Save Task List");
        println!("(5) Clear All Tasks");
        println!("(6) Import File");
        println!("(7) Exit");
        println!("---------------------------");

        print!("Select an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read line");
                let description = description.trim().to_string();
            
                if description.is_empty() {
                    println!("Description cannot be empty. Please try again.");
                    continue;
                }
            
                if task_list.values().any(|task| task.description == description) {
                    println!("Task with the same description already exists. Please try again.");
                    continue;
                }

                print!("Enter task priority (1-5): ");
                io::stdout().flush().unwrap();
                let mut priority_input = String::new();
                let mut priority: u32 = 1;
                loop {
                    io::stdin().read_line(&mut priority_input).expect("Failed to read line");
                    match priority_input.trim().parse() {
                        Ok(num) if num >= 1 && num <= 5 => {
                            priority = num;
                            break;
                        }
                        _ => {
                            println!("Invalid input. Priority must be between 1 and 5. Please try again: ");
                            priority_input.clear();
                        }
                    }
                }

                let mut deadline: String = String::new();
                loop {
                    print!("Enter task deadline (DD-MM-YYYY): ");
                    io::stdout().flush().unwrap();
                    let mut deadline_input = String::new();
                    io::stdin().read_line(&mut deadline_input).expect("Failed to read line");
                    let deadline_str = deadline_input.trim().to_string();
                    if let Ok(_) = NaiveDate::parse_from_str(&deadline_str, "%d-%m-%Y") {
                        deadline = deadline_str;
                        break;
                    } else {
                        println!("Invalid input. Please enter a valid date in the format DD-MM-YYYY");
                    }
                }

                println!("Task ID: {}", task_id);

                let task = Task::new(description, priority, deadline);
                task_list.insert(task_id, task);
                task_id += 1;

                println!("Task added successfully!");
            }

            2 => {
                print!("Enter the ID of the task to mark as completed: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: usize = match id.trim().parse() {
                    Ok(num) if num > 0 && num <= task_id => num,
                    _ => {
                        println!("Invalid input. Please enter a valid task ID.");
                        continue;
                    }
                };

                if let Some(task) = task_list.get_mut(&id) {
                    task.complete();
                    println!("Task marked as completed!");
                } else {
                    println!("Task not found!");
                }
            }

            3 => {
                if task_list.is_empty() {
                    println!(">>>> No Tasks! <<<<")
                } else {
                    let pending_tasks: Vec<_> = task_list
                        .iter()
                        .filter(|&(_, task)| !task.completed)
                        .collect();
                    let completed_tasks: Vec<_> = task_list
                        .iter()
                        .filter(|&(_, task)| task.completed)
                        .collect();
                    let completed_count = completed_tasks.len();
                    let pending_count = pending_tasks.len();

                    println!("============ All Tasks ============");

                    println!();
                    println!("Pending Task Count: {}", pending_count);
                    for (id, task) in &pending_tasks {
                        println!("--------");
                        println!("Task ID: {}", id);
                        task.display();
                        println!("--------");
                    }

                    println!();
                    println!("Completed Task Count: {}", completed_count);
                    for (id, task) in &completed_tasks {
                        println!("--------");
                        println!("Task ID: {}", id);
                        task.display();
                        println!("--------");
                    }

                    println!("===================================");
                }
            }

            4 => {
                if task_list.is_empty() {
                    println!("Nothing to save. Task list is empty.");
                } else {
                    println!("Save Menu:");
                    println!("(1) Save as text file.");
                    println!("(2) Save as html file.");
                    println!("(3) Save as both text and html file.");
                    print!("Select an option: ");
                    io::stdout().flush().unwrap();
                    let mut save = String::new();
                    let mut save_as: u32 = 3;

                    loop {
                        io::stdin().read_line(&mut save).expect("Failed to read line");
                        match save.trim().parse() {
                            Ok(num) if num >= 1 && num <= 3 => {
                                save_as = num;
                                break;
                            }
                            _ => {
                                println!("Invalid input. Must be between 1 and 3. Please try again: ");
                                save.clear();
                            }
                        }
                    }

                    if save_as == 1 {
                        if save_task_list(&task_list, "to-do list.txt") {
                            println!("Task list saved successfully!");
                        } else {
                            println!("Failed to save the task list.");
                        }
                    }
                    if save_as == 2 {
                        if save_task_list_html(&task_list, "to-do list.html") {
                            println!("Task list saved successfully!");
                        } else {
                            println!("Failed to save the task list.");
                        }
                    }
                    if save_as == 3 {
                        if save_task_list(&task_list, "to-do list.txt") && save_task_list_html(&task_list, "to-do list.html") {
                            println!("Task list saved successfully!");
                        } else {
                            println!("Failed to save the task list.");
                        }
                    }
                }
            }

            5 => {
                task_list.clear();
                println!("All tasks have been cleared.");
            }

            6 => {

                if let Some(imported_task_list) = import("to-do list.txt") {
                    task_list = imported_task_list;
                    println!("Todo List imported successfully!");
                } else {
                    println!("Failed to import the Todo List.");
                }
            }

            7 => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid option. Please select a valid option.");
            }
        }
    }
}

//------------------- TEXT FILE -------------------//

fn save_task_list(task_list: &HashMap<usize, Task>, filename: &str) -> bool {
    let mut file = match File::create(filename) {
        Ok(file) => file,
        Err(_) => return false,
    };

    let pending_count = task_list.values().filter(|task| !task.completed).count();
    let completed_count = task_list.values().filter(|task| task.completed).count();

    if let Err(_) = writeln!(file, "============= All TASKS ============") {
        return false;
    }
    if let Err(_) = writeln!(file, "Pending Task Count: {}", pending_count) {
        return false;
    }

    for (id, task) in task_list {
        if !task.completed {
            if let Err(_) = writeln!(file, "----------") {
                return false;
            }
            if let Err(_) = writeln!(file, "Task ID: {}", id) {
                return false;
            }
            if let Err(_) = writeln!(file, "Description: {}", task.description) {
                return false;
            }
            if let Err(_) = writeln!(file, "Priority: {}", task.priority) {
                return false;
            }
            if let Err(_) = writeln!(file, "Deadline: {:?}", task.deadline) {
                return false;
            }
            if let Err(_) = writeln!(file, "Status: Pending") {
                return false;
            }
            if let Err(_) = writeln!(file, "----------") {
                return false;
            }
        }
    }
    if let Err(_) = writeln!(file, "") {
        return false;
    }

    if let Err(_) = writeln!(file, "Completed Task Count: {}", completed_count) {
        return false;
    }

    for (id, task) in task_list {
        if task.completed {
            if let Err(_) = writeln!(file, "----------") {
                return false;
            }
            if let Err(_) = writeln!(file, "Task ID: {}", id) {
                return false;
            }
            if let Err(_) = writeln!(file, "Description: {}", task.description) {
                return false;
            }
            if let Err(_) = writeln!(file, "Priority: {}", task.priority) {
                return false;
            }
            if let Err(_) = writeln!(file, "Deadline: {:?}", task.deadline) {
                return false;
            }
            if let Err(_) = writeln!(file, "Status: Completed") {
                return false;
            }
            if let Err(_) = writeln!(file, "----------") {
                return false;
            }
        }
    }

    if let Err(_) = writeln!(file, "===================================") {
        return false;
    }


    true
}

//------------------- HTML FILE -------------------//

fn save_task_list_html(task_list: &HashMap<usize, Task>, filename: &str) -> bool {
    let mut file = match File::create(filename) {
        Ok(file) => file,
        Err(_) => return false,
    };

    let html = generate_task_list_html(task_list);

    if let Err(_) = write!(file, "{}", html) {
        return false;
    }

    true
}

fn generate_task_list_html(task_list: &HashMap<usize, Task>) -> String {
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str("<style>\n");
    html.push_str("table, td {\n");
    html.push_str("  border: 1px solid #000000;\n");
    html.push_str("  border-collapse: collapse;\n");
    html.push_str("  text-align: center;\n");
    html.push_str("}\n");
    html.push_str(".completed { color: green; }");
    html.push_str(".pending { color: red; }");
    html.push_str("table {");
    html.push_str("  margin: 0 auto;");
    html.push_str("}\n");
    html.push_str("</style>\n");
    html.push_str("</head>\n<body>\n");

    html.push_str("<table width=50%>\n");
    html.push_str("<tr>\n");
    html.push_str("  <th>Task ID</th>\n");
    html.push_str("  <th>Description</th>\n");
    html.push_str("  <th>Priority</th>\n");
    html.push_str("  <th>Deadline</th>\n");
    html.push_str("  <th>Status</th>\n");
    html.push_str("</tr>\n");

    for (id, task) in task_list {
        html.push_str("<tr>\n");
        html.push_str(format!("<td>{}</td>\n", id).as_str());
        html.push_str(format!("<td>{}</td>\n", task.description).as_str());
        html.push_str(format!("<td>{}</td>\n", task.priority).as_str());
        html.push_str(format!("<td>{:?}</td>\n", task.deadline).as_str());
        if task.completed {
            html.push_str("<td class=\"completed\">&#10004;</td>\n");
        } else {
            html.push_str("<td class=\"pending\">&#10008;</td>\n");
        }
        html.push_str("</tr>\n");
    }

    html.push_str("</table>\n");
    html.push_str("</body>\n</html>");

    html
}


//------------------- IMPORT FILE -------------------//
fn import(filename: &str) -> Option<HashMap<usize, Task>> {
    let mut task_list: HashMap<usize, Task> = HashMap::new();

    match File::open(filename) {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            let mut in_completed_section = false;
            let mut task_data: Vec<String> = Vec::new();
            let mut task_id: usize = 1;

            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.starts_with("Completed Task Count:") {
                        in_completed_section = true;
                        continue;
                    }

                    if in_completed_section && line.starts_with("Task ID: ") {
                        in_completed_section = false;
                        continue;
                    }

                    if line.starts_with("Task ID: ") {
                        if !task_data.is_empty() {
                            let task = parse_task_data(&task_data);
                            task_list.insert(task_id, task);
                            task_id += 1;
                            task_data.clear();
                        }
                    }

                    task_data.push(line);
                }
            }

            if !task_data.is_empty() {
                let task = parse_task_data(&task_data);
                task_list.insert(task_id, task);
            }

            Some(task_list)
        }
        Err(_) => None,
    }
}

fn parse_task_data(data: &Vec<String>) -> Task {
    let mut description = String::new();
    let mut priority = 1;
    let mut deadline = String::new();

    for line in data.iter() {
        if line.starts_with("Description: ") {
            description = line.trim_start_matches("Description: ").to_string();
        } else if line.starts_with("Priority: ") {
            priority = line.trim_start_matches("Priority: ").parse().unwrap();
        } else if line.starts_with("Deadline: ") {
            deadline = line.trim_start_matches("Deadline: ").to_string();
        }
    }

    Task::new(description, priority, deadline)
}
