use crate::User;
use crate::Db;
use actix_web::post;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::{web , };
use argon2::password_hash;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use password_hash::rand_core::OsRng;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use jsonwebtoken::errors::ErrorKind;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

/// Create a JWT for the given username
fn create_jwt(username: &str, secret: &[u8]) -> Result<String, jsonwebtoken::errors::Error> {
    // Calculate expiration time (24 hours from now)
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .ok_or_else(|| {
            jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken)
        })?;

    // Create JWT claims
    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration.timestamp() as usize,
    };

    // Generate JWT token using default header and provided secret
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
}

#[post("/register")]
async fn register(user: web::Json<User>, db: web::Data<Db>) -> impl Responder {
    // Check if user already exists
    let existing_user = sqlx::query_as::<_, User>("SELECT name, password FROM \"user\" WHERE name = $1")
        .bind(&user.name)
        .fetch_optional(&db.pool)
        .await;

    match existing_user {
        Ok(Some(_)) => {
            return HttpResponse::Conflict().body("User already exists");
        }
        Ok(None) => {
            // User doesn't exist, proceed with registration
        }
        Err(e) => {
            eprintln!("Database error during registration: {:?}", e);
            return HttpResponse::InternalServerError().body("Database error during registration");
        }
    }

    let password_hash = match hash_password(&user) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().body("Password hashing failed"),
    };

    let insertion = sqlx::query_as::<_, User>("INSERT INTO \"user\" (name, password) VALUES ($1, $2) RETURNING name, password")
        .bind(&user.name)
        .bind(&password_hash)
        .fetch_one(&db.pool)
        .await;

    match insertion {
        Ok(_) => {
            // Get JWT secret from environment variable
            let secret = std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-super-secret-jwt-key-change-me".into());
            
            match create_jwt(&user.name, secret.as_bytes()) {
                Ok(token) => HttpResponse::Ok().body(token),
                Err(e) => {
                    eprintln!("Token creation failed for user {}: {:?}", user.name, e);
                    HttpResponse::InternalServerError().body("Token creation failed")
                },
            }
        }
        Err(e) => {
            eprintln!("Failed to register user: {:?}", e);
            HttpResponse::InternalServerError().body("Registration failed")
        }
    }
}

fn hash_password(user: &User) -> Result<String, Box<dyn std::error::Error>> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(user.password.as_bytes(), &salt).unwrap();
    Ok(hash.to_string())
}
 
