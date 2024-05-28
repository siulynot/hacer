use std::io::{self, Write};
use std::fs::{self, OpenOptions};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            completed: false,
        }
    }
}

fn add_task(tasks: &mut Vec<Task>, description: String) {
    let task = Task::new(description);
    tasks.push(task);
    println!("Task added!");
}

fn view_tasks(tasks: &Vec<Task>) {
    for (index, task) in tasks.iter().enumerate() {
        let status = if task.completed { "✓" } else { " " };
        println!("{}: [{}] {}", index + 1, status, task.description);
    }
}

fn mark_task_complete(tasks: &mut Vec<Task>, index: usize) {
    if index < tasks.len() {
        tasks[index].completed = true;
        println!("La tarea fue completada!");
    } else {
        println!("Número de tarea no existe.");
    }
}

fn delete_task(tasks: &mut Vec<Task>, index: usize) {
    if index < tasks.len() {
        tasks.remove(index);
        println!("Tarea borrada!");
    } else {
        println!("Número de tarea no existe.");
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let serialized = serde_json::to_string(tasks).expect("Fallo en numerar tarea.");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("tasks.json")
        .expect("Fallo en abrir archivo.");
    file.write_all(serialized.as_bytes()).expect("Fallo en escribir al archivo.");
}

fn load_tasks() -> Vec<Task> {
    let file_content = fs::read_to_string("tasks.json");
    match file_content {
        Ok(content) => {
            serde_json::from_str(&content).expect("Fallo en numerar tarea.")
        }
        Err(_) => Vec::new(),
    }
}

fn main() {
    let mut tasks = load_tasks();

    loop {
        println!("Cosas para Hacer:");
        println!("1. Añadir Tarea");
        println!("2. Ver Tareas");
        println!("3. Marcar Tarea completada");
        println!("4. Borrar Tarea");
        println!("5. Salir");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Fallo en leer línea.");
        let choice = choice.trim();

        match choice {
            "1" => {
                let mut description = String::new();
                println!("Escribe descripción de tarea:");
                io::stdin().read_line(&mut description).expect("Fallo en leer línea.");
                add_task(&mut tasks, description.trim().to_string());
            }
            "2" => view_tasks(&tasks),
            "3" => {
                let mut index = String::new();
                println!("Escribe número de tarea completada:");
                io::stdin().read_line(&mut index).expect("Fallo en leer línea.");
                if let Ok(index) = index.trim().parse::<usize>() {
                    mark_task_complete(&mut tasks, index - 1);
                }
            }
            "4" => {
                let mut index = String::new();
                println!("Escribe número de tarea para borrar:");
                io::stdin().read_line(&mut index).expect("Fallo en leer línea.");
                if let Ok(index) = index.trim().parse::<usize>() {
                    delete_task(&mut tasks, index - 1);
                }
            }
            "5" => {
                save_tasks(&tasks);
                break;
            }
            _ => println!("Selección inválida."),
        }
    }
}

