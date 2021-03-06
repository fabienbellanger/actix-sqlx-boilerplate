//! API tasks handlers module

use crate::errors::AppError;
use crate::models::task::{Task, TaskCreation};
use crate::repositories::task::TaskRepository;
use actix_web::web::{Bytes, BytesMut};
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

// Route: GET "/v1/tasks/stream"
// TODO: Try with futures::StreamExt instead of async_stream.
pub async fn get_all_stream(pool: web::Data<MySqlPool>) -> Result<impl Responder, AppError> {
    let stream_tasks = async_stream::stream! {
        let mut tasks = TaskRepository::get_all(pool.get_ref());
        let mut bytes = BytesMut::new();

        bytes.extend_from_slice("[".as_bytes());
        let byte = bytes.split().freeze();
        yield Ok::<Bytes, AppError>(byte);

        let mut i = 0;
        while let Some(row) = tasks.try_next().await? {
            if i > 0 {
                bytes.extend_from_slice(",".as_bytes());
            }
            i += 1;

            match row {
                Ok(row) => match serde_json::to_string(&row) {
                        Ok(task) => {
                            bytes.extend_from_slice(task.as_bytes());
                            let byte = bytes.split().freeze();
                            yield Ok::<Bytes, AppError>(byte)
                        },
                        Err(err) => error!("Tasks list stream error: {}", err)
                    },
                Err(err) => error!("Tasks list stream error: {}", err)
            }
        }

        bytes.extend_from_slice("]".as_bytes());
        let byte = bytes.split().freeze();
        yield Ok::<Bytes, AppError>(byte);
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .streaming(Box::pin(stream_tasks)))
}
