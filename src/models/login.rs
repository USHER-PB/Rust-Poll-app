use actix_web::{post, web, HttpResponse, Responder};
use argon2::password_hash;
use argon2::{Argon2, PasswordHash};
use password_hash::PasswordVerifier;
use crate::{User, Db}; // Ensure Db and User are correctly imported from crate root
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use jsonwebtoken::errors::ErrorKind; // Import ErrorKind to create specific JWT errors

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
            // If duration calculation fails, return a jsonwebtoken error from ErrorKind
            jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken)
        })?;

    // Create JWT claims
    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration.timestamp() as usize, // Convert i64 timestamp to usize
    };

    // Generate JWT token using default header and provided secret
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
}


/// Login handler: verifies password and returns JWT if successful
#[post("/login")]
pub async fn login(db: web::Data<Db>, data: web::Json<User>) -> impl Responder {
    // Query the database to find the user by username
    // Assuming your user table is named 'users' (plural)
    let result = sqlx::query_as::<_, User>("SELECT name, password FROM \"user\" WHERE name = $1")
        .bind(&data.name)
        .fetch_optional(&db.pool) // Access the PgPool via db.pool
        .await;

    match result {
        Ok(Some(user)) => {
            // Initialize Argon2 for password verification
            let argon2 = Argon2::default();
            
            // Parse the stored hashed password
            let parsed_hash = match PasswordHash::new(&user.password) {
                Ok(hash) => hash,
                Err(e) => {
                    eprintln!("Invalid stored password hash for user {}: {:?}", user.name, e);
                    return HttpResponse::InternalServerError().body("Invalid stored hash format");
                },
            };
            
            // Verify the provided password against the stored hash
            match argon2.verify_password(data.password.as_bytes(), &parsed_hash) {
                Ok(_) => {
                    // Password is correct, proceed to create JWT
                    // Get JWT secret from environment variable, or use a placeholder
                    let secret = std::env::var("JWT_SECRET")
                        .unwrap_or_else(|_| "your-super-secret-jwt-key-change-me".into()); // Strongly advise changing this default!
                    
                    match create_jwt(&user.name, secret.as_bytes()) {
                        Ok(token) => HttpResponse::Ok().body(token),
                        Err(e) => {
                            eprintln!("Token creation failed for user {}: {:?}", user.name, e);
                            HttpResponse::InternalServerError().body("Token creation failed")
                        },
                    }
                },
                Err(_) => {
                    // Password verification failed
                    HttpResponse::Unauthorized().body("Invalid credentials")
                },
            }
        }
        Ok(None) => {
            // User not found in the database
            HttpResponse::Unauthorized().body("User not found")
        },
        Err(e) => {
            // Database query error
            eprintln!("Database error during login for user {}: {:?}", data.name, e);
            HttpResponse::InternalServerError().body("Database error during login")
        },
    }
}