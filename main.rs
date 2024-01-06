use std::io;

struct Task {
    id: usize,
    description: String,
    completed: bool,
}

struct TaskManager {
    tasks: Vec<Task>,
    counter: usize,
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager {
            tasks: Vec::new(),
            counter: 1,
        }
    }

    fn add_task(&mut self, description: String) {
        let new_task = Task {
            id: self.counter,
            description,
            completed: false,
        };
        self.counter += 1;
        self.tasks.push(new_task);
        println!("Task added successfully!");
    }

    fn list_tasks(&self) {
        println!("Tasks:");
        for task in &self.tasks {
            let status = if task.completed { "Completed" } else { "Not Completed" };
            println!("ID: {}, Description: {}, Status: {}", task.id, task.description, status);
        }
    }

    fn mark_completed(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            println!("Task marked as completed!");
        } else {
            println!("Task not found!");
        }
    }

    fn remove_task(&mut self, id: usize) {
        if let Some(index) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(index);
            println!("Task removed!");
        } else {
            println!("Task not found!");
        }
    }
}

fn main() {
    let mut task_manager = TaskManager::new();

    loop {
        println!("Task Manager");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Mark Task as Completed");
        println!("4. Remove Task");
        println!("5. Exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter task description:");
                let mut task_description = String::new();
                io::stdin()
                    .read_line(&mut task_description)
                    .expect("Failed to read line");
                task_manager.add_task(task_description.trim().to_string());
            }
            2 => {
                task_manager.list_tasks();
            }
            3 => {
                println!("Enter task ID to mark as completed:");
                let mut task_id = String::new();
                io::stdin()
                    .read_line(&mut task_id)
                    .expect("Failed to read line");
                let task_id: usize = match task_id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a valid ID");
                        continue;
                    }
                };
                task_manager.mark_completed(task_id);
            }
            4 => {
                println!("Enter task ID to remove:");
                let mut task_id = String::new();
                io::stdin()
                    .read_line(&mut task_id)
                    .expect("Failed to read line");
                let task_id: usize = match task_id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a valid ID");
                        continue;
                    }
                };
                task_manager.remove_task(task_id);
            }
            5 => {
                println!("Exiting Task Manager");
                break;
            }
            _ => {
                println!("Invalid choice, please choose a number between 1-5");
            }
        }
    }
}
