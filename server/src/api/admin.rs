use std::{sync::Arc, time::Instant};

use axum::{
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
use hyper::StatusCode;
use once_cell::sync::Lazy;
use rand_chacha::ChaCha8Rng;
use rand_core::RngCore;
use shuttle_secrets::SecretStore;
use tokio::sync::{Mutex, RwLock};

static ADMIN_TIMEOUT: f64 = 900f64;

type Random = Arc<Mutex<ChaCha8Rng>>;

#[derive(TryFromMultipart)]
struct AdminLoginRequest {
    username: String,
    password: String,
}

struct SessionHandler {
    instant: Option<Instant>,
    session_id: String,
}

impl SessionHandler {
    pub fn new() -> Self {
        Self {
            instant: None,
            session_id: String::new(),
        }
    }

    pub fn invalidate(&mut self) {
        self.instant = None;
    }

    pub async fn get_session(&mut self, random: Random) -> String {
        if let Some(instant) = self.instant {
            if Instant::now().duration_since(instant).as_secs_f64() < ADMIN_TIMEOUT {
                self.reset_timer();
                return self.session_id.clone();
            }
        }

        let mut pool = [0u8; 16];
        random.lock().await.fill_bytes(&mut pool);
        let session_token = u128::from_le_bytes(pool).to_string();
        self.session_id = session_token.clone();
        self.reset_timer();
        session_token
    }

    pub fn check(&mut self, session_id: &str) -> bool {
        let Some(instant) = self.instant else {
            return false;
        };

        if Instant::now().duration_since(instant).as_secs_f64() > ADMIN_TIMEOUT {
            return false;
        }

        self.reset_timer();
        self.session_id == session_id
    }

    pub fn reset_timer(&mut self) {
        self.instant = Some(Instant::now());
    }
}

static ADMIN_SESSION: Lazy<RwLock<SessionHandler>> =
    Lazy::new(|| RwLock::new(SessionHandler::new()));

async fn is_logged_in(jar: CookieJar) -> impl IntoResponse {
    let Some(received_token) = jar.get("session_token") else {
        return Json(false);
    };

    Json(ADMIN_SESSION.write().await.check(received_token.value()))
}

async fn login(
    Extension(secrets): Extension<SecretStore>,
    Extension(random): Extension<Random>,
    jar: CookieJar,
    TypedMultipart(form): TypedMultipart<AdminLoginRequest>,
) -> impl IntoResponse {
    println!("LOGGING IN");
    if Some(form.username) != secrets.get("ADMIN_USER")
        || Some(form.password) != secrets.get("ADMIN_PASS")
    {
        return Err(StatusCode::FORBIDDEN);
    }

    let session_token = ADMIN_SESSION.write().await.get_session(random).await;
    return Ok(jar.add(
        Cookie::build("session_token", session_token)
            .path("/")
            .finish(),
    ));
}

async fn logout_all() {
    ADMIN_SESSION.write().await.invalidate();
    println!("LOGGED OUT ALL USERS");
}

pub fn router() -> Router {
    Router::new()
        .route("/login", get(is_logged_in).post(login))
        .route("/logout_all", post(logout_all))
}
