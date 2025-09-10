use actix_web::{post, web, get, put, delete, HttpResponse, Responder};
use sqlx::FromRow;
use serde::{Deserialize, Serialize};

use crate::Db; // Correct import for your Db struct

#[derive(Serialize, Deserialize, FromRow, Debug, PartialEq, Eq)]
pub struct Poll {
    pub id: i32,
    pub title: String,
    pub options: Vec<String>,
    pub votes: Vec<i32>,
    pub created_at: String,
}

// Separate struct for incoming vote request
#[derive(Debug, Deserialize)]
pub struct VoteRequest {
    pub option_index: i32,
}

// Struct for poll statistics response
#[derive(Serialize)]
pub struct PollOptionStats {
    pub option: String,
    pub votes: i32,
    pub percentage: f64,
}

/// Create a new poll
#[post("/poll")]
pub async fn create_poll(db: web::Data<Db>, poll: web::Json<Poll>) -> impl Responder {
    // Validate poll data
    if poll.options.len() < 2 {
        return HttpResponse::BadRequest().body("At least 2 options required");
    }
    
    // Initialize votes array with zeros for each option
    let initial_votes = vec![0; poll.options.len()];
    
    let result = sqlx::query_as::<_, Poll>(
        "INSERT INTO poll (title, options, votes) VALUES ($1, $2, $3) RETURNING id, title, options, votes, created_at"
    )
        .bind(&poll.title)
        .bind(&poll.options)
        .bind(&initial_votes)
        .fetch_one(&db.pool) // Corrected: use db.db
        .await;
    
    match result {
        Ok(poll) => HttpResponse::Ok().json(poll),
        Err(e) => {
            eprintln!("Failed to create poll: {:?}", e); // Log the error for debugging
            HttpResponse::InternalServerError().body("Failed to create poll")
        },
    }
}

/// Get a poll by ID
#[get("/poll/{id}")] // Corrected: closing bracket for id
pub async fn get_poll(db: web::Data<Db>, path: web::Path<i32>) -> impl Responder { // Corrected: path parameter type
    let id = *path;
    let result = sqlx::query_as::<_, Poll>("SELECT id, title, options, votes, created_at FROM poll WHERE id = $1")
        .bind(id)
        .fetch_optional(&db.pool) // Corrected: use db.db
        .await;
    
    match result {
        Ok(Some(poll)) => HttpResponse::Ok().json(poll),
        Ok(None) => HttpResponse::NotFound().body("Poll not found"),
        Err(e) => {
            eprintln!("Database error when getting poll: {:?}", e);
            HttpResponse::InternalServerError().body("Database error")
        },
    }
}

/// Update a poll
#[put("/poll/{id}")] // Corrected: closing bracket for id
pub async fn update_poll(db: web::Data<Db>, path: web::Path<i32>, poll_data: web::Json<Poll>) -> impl Responder {
    let id = *path;

    // Fetch the existing poll to compare options and reset votes if needed
    let existing_poll_result = sqlx::query_as::<_, Poll>("SELECT id, options, votes FROM poll WHERE id = $1")
        .bind(id)
        .fetch_optional(&db.pool) // Corrected: use db.db
        .await;

    let (current_options, current_votes) = match existing_poll_result {
        Ok(Some(p)) => (p.options, p.votes),
        Ok(None) => return HttpResponse::NotFound().body("Poll not found"),
        Err(e) => {
            eprintln!("Failed to fetch existing poll for update: {:?}", e);
            return HttpResponse::InternalServerError().body("Database error");
        }
    };

    let new_title = &poll_data.title;
    let new_options = &poll_data.options;

    // If options have changed, reset votes to zero for the new options length
    let updated_votes = if current_options != *new_options {
        if new_options.len() < 2 {
            return HttpResponse::BadRequest().body("At least 2 options required for update");
        }
        vec![0; new_options.len()] // Reset votes
    } else {
        current_votes // Keep existing votes if options didn't change
    };

    let result = sqlx::query(
        "UPDATE poll SET title = $1, options = $2, votes = $3 WHERE id = $4"
    )
        .bind(new_title)
        .bind(new_options)
        .bind(&updated_votes)
        .bind(id)
        .execute(&db.pool) // Corrected: use db.db
        .await;
    
    match result {
        Ok(_) => HttpResponse::Ok().body("Poll updated"),
        Err(e) => {
            eprintln!("Failed to update poll: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update poll")
        },
    }
}

/// Delete a poll
#[delete("/poll/{id}")]
pub async fn delete_poll(db: web::Data<Db>, path: web::Path<i32>) -> impl Responder {
    let id = *path;
    let result = sqlx::query("DELETE FROM poll WHERE id = $1")
        .bind(id)
        .execute(&db.pool) // Corrected: use db.db
        .await;
    
    match result {
        Ok(_) => HttpResponse::Ok().body("Poll deleted"),
        Err(e) => {
            eprintln!("Failed to delete poll: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete poll")
        },
    }
}

/// Vote on a poll
#[post("/poll/{id}/vote")]
pub async fn vote(db: web::Data<Db>, path: web::Path<i32>, vote_req: web::Json<VoteRequest>) -> impl Responder {
    let poll_id = path.into_inner();
    let vote_index = vote_req.option_index;

    // 1. Fetch the poll to validate the option index.
    let poll_result = sqlx::query_as::<_, Poll>("SELECT id, options, votes, created_at FROM poll WHERE id = $1")
        .bind(poll_id)
        .fetch_optional(&db.pool) // Corrected: use db.db
        .await;

    let poll = match poll_result {
        Ok(Some(p)) => p,
        Ok(None) => return HttpResponse::NotFound().body("Poll not found"),
        Err(e) => {
            eprintln!("Failed to fetch poll for voting: {:?}", e);
            return HttpResponse::InternalServerError().body("Failed to fetch poll");
        }
    };

    // 2. Validate vote index against the actual options.
    // PostgreSQL array indices are 1-based, Rust Vec indices are 0-based.
    // So, if vote_index is 0, it refers to poll.options[0].
    if vote_index < 0 || (vote_index as usize) >= poll.options.len() {
        return HttpResponse::BadRequest().body("Invalid option selected");
    }

    // 3. Atomically update the vote count at the specified index.
    // We update votes[vote_index] (0-based) which corresponds to votes[$1] (1-based) in SQL
    let update_result = sqlx::query("UPDATE poll SET votes[$1] = votes[$1] + 1 WHERE id = $2")
        .bind(vote_index + 1) // Convert 0-based Rust index to 1-based PostgreSQL index
        .bind(poll_id)
        .execute(&db.pool) // Corrected: use db.db
        .await;

    match update_result {
        Ok(_) => HttpResponse::Ok().body("Vote recorded"),
        Err(e) => {
            eprintln!("Failed to record vote: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to record vote")
        },
    }
}

/// Get poll statistics
#[get("/poll/{id}/stats")]
pub async fn get_stats(db: web::Data<Db>, path: web::Path<i32>) -> impl Responder {
    let id = *path;
    let result = sqlx::query_as::<_, Poll>("SELECT id, title, options, votes, created_at FROM poll WHERE id = $1")
        .bind(id)
        .fetch_optional(&db.pool) // Corrected: use db.db
        .await;
    
    match result {
        Ok(Some(poll)) => {
            let total_votes: i32 = poll.votes.iter().sum();

            let stats: Vec<PollOptionStats> = poll.options.iter()
                .zip(poll.votes.iter())
                .map(|(option_text, &vote_count)| {
                    let percentage = if total_votes > 0 {
                        (vote_count as f64 / total_votes as f64) * 100.0
                    } else {
                        0.0
                    };
                    PollOptionStats {
                        option: option_text.clone(),
                        votes: vote_count,
                        percentage: percentage,
                    }
                })
                .collect();
            
            HttpResponse::Ok().json(stats)
        },
        Ok(None) => HttpResponse::NotFound().body("Poll not found"),
        Err(e) => {
            eprintln!("Failed to get stats: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to get stats")
        },
    }
}

/// Get expiring polls
#[get("/polls/expiring")]
async fn get_expiring_polls(db: web::Data<Db>) -> impl Responder {
    // Note: This query checks polls created more than 24 hours ago.
    // "Expiring" usually implies a future expiration. If you mean "past due for voting" or similar,
    // you might need an `expires_at` column in your table for more accurate "expiring" logic.
    let result = sqlx::query_as::<_, Poll>("SELECT id, title, options, votes, created_at FROM poll WHERE created_at < NOW() - INTERVAL '24 hours'")
        .fetch_all(&db.pool) // Corrected: use db.db
        .await;
    
    match result {
        Ok(polls) => HttpResponse::Ok().json(polls),
        Err(e) => {
            eprintln!("Failed to get expiring polls: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to get expiring polls")
        },
    }
}

/// Get all polls
#[get("/polls")]
pub async fn get_all_polls(db: web::Data<Db>) -> impl Responder {
    let result = sqlx::query_as::<_, Poll>("SELECT id, title, options, votes, created_at FROM poll ORDER BY created_at DESC")
        .fetch_all(&db.pool)
        .await;
    
    match result {
        Ok(polls) => HttpResponse::Ok().json(polls),
        Err(e) => {
            eprintln!("Failed to get polls: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to get polls")
        },
    }
}

// Re-export functions for use in main.rs or other modules to register services
pub fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_poll)
       .service(get_poll)
       .service(update_poll)
       .service(delete_poll)
       .service(vote)
       .service(get_stats)
       .service(get_expiring_polls)
       .service(get_all_polls);
}