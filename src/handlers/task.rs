//! API tasks handlers module

use crate::errors::AppError;
use crate::models::task::{Task, TaskCreation};
use crate::repositories::task::TaskRepository;
use actix_web::web::{Bytes, BytesMut};
use actix_web::{web, HttpResponse, Responder};
use actix_web_validator::Json;
use futures::TryStreamExt;
use sqlx::MySqlPool;

// Watch https://github.com/rich-murphey/sqlx-actix-streaming

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

// Route: GET "/v1/tasks/stream"
pub async fn get_all_stream(pool: web::Data<MySqlPool>) -> Result<impl Responder, AppError> {
    let stream_tasks = async_stream::stream! {
        let mut tasks = TaskRepository::get_all(pool.get_ref());
        let mut bytes = BytesMut::with_capacity(64);

        while let Some(row) = tasks.try_next().await? {
            let _task = row.unwrap();
            bytes.extend_from_slice(serde_json::to_string(&_task).unwrap().as_bytes());
            let byte = bytes.split().freeze();
            yield Ok::<Bytes, AppError>(byte)
        }
    };

    Ok(HttpResponse::Ok().streaming(Box::pin(stream_tasks)))
}

// Route: GET "/v1/tasks/big"
pub async fn get_all_big() -> Result<impl Responder, AppError> {
    let mut tasks: Vec<Task> = Vec::new();

    for _i in 0..100_000 {
        tasks.push(Task {
            id: String::from(""),
            name: String::from("A Task"),
            description: Some(String::from("A Long Long Description")),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
        });
    }

    Ok(HttpResponse::Ok().json(tasks))
}
