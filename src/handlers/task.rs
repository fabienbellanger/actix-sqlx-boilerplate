//! API tasks handlers module

use crate::errors::AppError;
use crate::models::task::{Task, TaskCreation};
use crate::repositories::task::TaskRepository;
use actix_web::{web, HttpResponse, Responder};
use actix_web_validator::Json;
use futures::TryStreamExt;
use sqlx::MySqlPool;

// Route: POST "/v1/tasks"
pub async fn create(pool: web::Data<MySqlPool>, form: Json<TaskCreation>) -> Result<impl Responder, AppError> {
    let mut task = Task::new(form.0);
    let result = TaskRepository::create(pool.get_ref(), &mut task).await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().json(task)),
        _ => Err(AppError::InternalError {
            message: String::from("Error during task creation"),
        }),
    }
}

// Route: GET "/v1/tasks"
pub async fn get_all(pool: web::Data<MySqlPool>) -> Result<impl Responder, AppError> {
    let mut stream = TaskRepository::get_all(pool.get_ref());
    let mut tasks: Vec<Task> = Vec::new();
    while let Some(row) = stream.try_next().await? {
        tasks.push(row?);
    }

    Ok(HttpResponse::Ok().json(tasks))
}
