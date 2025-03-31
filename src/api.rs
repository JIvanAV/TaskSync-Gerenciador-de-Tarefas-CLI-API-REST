use actix_web::{web, HttpResponse, Responder};
use crate::db;
use crate::models::{Task, NewTask, UpdateTask};
use crate::auth::validate_jwt;
use sqlx::SqlitePool;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/tasks", web::post().to(create_task))
        .route("/tasks", web::get().to(list_tasks))
        .route("/tasks/{id}", web::put().to(update_task));
}

async fn create_task(
    task: web::Json<NewTask>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let new_task = task.into_inner();
    let id = uuid::Uuid::new_v4();
    let now = chrono::Utc::now().naive_utc();

    let result = sqlx::query!(
        "INSERT INTO tasks (id, title, description, created_at, updated_at, completed) VALUES (?, ?, ?, ?, ?, ?)",
        id.to_string(),
        new_task.title,
        new_task.description,
        now,
        now,
        false
    )
    .execute(&**pool)
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json(Task {
            id,
            title: new_task.title,
            description: new_task.description,
            created_at: now,
            updated_at: now,
            completed: false,
        }),
        Err(_) => HttpResponse::InternalServerError().body("Falha ao criar tarefa"),
    }
}

async fn list_tasks(pool: web::Data<SqlitePool>) -> impl Responder {
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(&**pool)
        .await;

    match tasks {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao listar tarefas"),
    }
}

async fn update_task(
    path: web::Path<(String,)>,
    task: web::Json<UpdateTask>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let (id,) = path.into_inner();
    let now = chrono::Utc::now().naive_utc();
    let update = task.into_inner();

    let mut query = String::from("UPDATE tasks SET updated_at = ?");
    let mut params: Vec<&(dyn sqlx::Encode<'_> + sqlx::Type<Sqlite> + Sync)> = vec![&now];

    if let Some(title) = update.title {
        query.push_str(", title = ?");
        params.push(&title);
    }

    if let Some(description) = update.description {
        query.push_str(", description = ?");
        params.push(&description);
    }

    if let Some(completed) = update.completed {
        query.push_str(", completed = ?");
        params.push(&completed);
    }

    query.push_str(" WHERE id = ?");

    let result = sqlx::query(&query)
        .bind_all(params)
        .bind(id)
        .execute(&**pool)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Tarefa atualizada"),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao atualizar tarefa"),
    }
}
