//! Task model module

use actix_web::{web::Bytes, Error};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use std::pin::Pin;
use std::task::{Context, Poll};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Task {
    fn _init(
        id: String,
        name: String,
        description: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        deleted_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    pub fn new(task: TaskCreation) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: task.name,
            description: task.description,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct TaskCreation {
    pub name: String,
    pub description: Option<String>,
}

// TODO: Watch https://github.com/rich-murphey/sqlx-actix-streaming
pub struct TaskStream {
    pub number: u32,
    pub next: u32,
    pub buf: Vec<u8>,
}

impl futures::Stream for TaskStream {
    type Item = Result<Bytes, Error>;

    // TODO: Ne fonctionne pas très bien si this.number = 0
    fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        if this.next == this.number {
            // Stop stream
            Poll::Ready(None)
        } else {
            // Array start
            if this.next == 0 {
                for v in b"[" {
                    this.buf.push(*v);
                }
            }

            let res = serde_json::to_writer(
                &mut this.buf,
                &Task {
                    id: String::from(""),
                    name: String::from("Task"),
                    description: Some(String::from("Description")),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                },
            );

            if let Err(e) = res {
                return Poll::Ready(Some(Err(e.into())));
            }

            this.next += 1;

            if this.next < this.number {
                // Comma between tasks
                for v in b"," {
                    this.buf.push(*v);
                }
            } else {
                // Array end
                for v in b"]" {
                    this.buf.push(*v);
                }
            }

            let poll = Poll::Ready(Some(Ok(Bytes::copy_from_slice(&this.buf))));

            this.buf.clear();

            poll
        }
    }
}
