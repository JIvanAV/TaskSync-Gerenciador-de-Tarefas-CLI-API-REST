use clap::{App, Arg, SubCommand};
use crate::db;
use crate::models::{NewTask, UpdateTask};
use crate::auth::validate_jwt;
use sqlx::SqlitePool;
use reqwest::Client;

pub async fn start(pool: SqlitePool, tx: tokio::sync::mpsc::Sender<String>) {
    let matches = App::new("TaskSync CLI")
        .version("0.1.0")
        .about("Gerenciador de tarefas")
        .subcommand(
            SubCommand::with_name("create")
                .about("Cria uma nova tarefa")
                .arg(Arg::with_name("title").required(true))
                .arg(Arg::with_name("description").required(true)),
        )
        .subcommand(SubCommand::with_name("list").about("Lista todas as tarefas"))
        .subcommand(
            SubCommand::with_name("update")
                .about("Atualiza uma tarefa existente")
                .arg(Arg::with_name("id").required(true))
                .arg(Arg::with_name("title"))
                .arg(Arg::with_name("description"))
                .arg(Arg::with_name("completed").takes_value(true)),
        )
        .get_matches();

    match matches.subcommand() {
        ("create", Some(sub_m)) => {
            let title = sub_m.value_of("title").unwrap();
            let description = sub_m.value_of("description").unwrap();
            create_task(title, description, pool).await;
        }
        ("list", _) => {
            list_tasks(pool).await;
        }
        ("update", Some(sub_m)) => {
            let id = sub_m.value_of("id").unwrap().to_string();
            let title = sub_m.value_of("title").map(|s| s.to_string());
            let description = sub_m.value_of("description").map(|s| s.to_string());
            let completed = sub_m.value_of("completed").map(|s| s.parse().unwrap_or(false));
            update_task(id, title, description, completed, pool).await;
        }
        _ => println!("Comando inválido!"),
    }
}

async fn create_task(title: &str, description: &str, pool: SqlitePool) {
    let new_task = NewTask {
        title: title.to_string(),
        description: description.to_string(),
    };

    let result = reqwest::Client::new()
        .post("http://localhost:8080/tasks")
        .json(&new_task)
        .send()
        .await;

    match result {
        Ok(response) => {
            if response.status().is_success() {
                println!("Tarefa criada com sucesso!");
            } else {
                println!("Erro ao criar tarefa!");
            }
        }
        Err(_) => println!("Erro de conexão com o servidor."),
    }
}

async fn list_tasks(pool: SqlitePool) {
    let response = reqwest::Client::new()
        .get("http://localhost:8080/tasks")
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status().is_success() {
                let tasks: Vec<crate::models::Task> = response.json().await.unwrap();
                for task in tasks {
                    println!("{:?} - {}: {}", task.id, task.title, task.completed);
                }
            } else {
                println!("Erro ao listar tarefas!");
            }
        }
        Err(_) => println!("Erro de conexão com o servidor."),
    }
}

async fn update_task(id: String, title: Option<String>, description: Option<String>, completed: Option<bool>, pool: SqlitePool) {
    let update_task = UpdateTask {
        title,
        description,
        completed,
    };

    let response = reqwest::Client::new()
        .put(format!("http://localhost:8080/tasks/{}", id))
        .json(&update_task)
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status().is_success() {
                println!("Tarefa atualizada com sucesso!");
            } else {
                println!("Erro ao atualizar tarefa!");
            }
        }
        Err(_) => println!("Erro de conexão com o servidor."),
    }
}
