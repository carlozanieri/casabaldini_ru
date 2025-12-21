use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::sync::Arc;
use tera::{Context, Tera};

// Creiamo una struttura per lo stato condiviso dell'applicazione
struct AppState {
    templates: Tera,
}

#[tokio::main]
async fn main() {
    // 1. Inizializziamo il motore di template caricando tutto dalla cartella /templates
    let mut tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Errore nel parsing dei template: {}", e);
            std::process::exit(1);
        }
    };

    // Disabilita l'auto-escape se necessario, ma di base Tera protegge da XSS
    tera.full_reload().unwrap();

    // 2. Creiamo lo stato condiviso avvolto in un Arc (Atomic Reference Counter)
    let shared_state = Arc::new(AppState { templates: tera });

    // 3. Definiamo le rotte e passiamo lo stato
    let app = Router::new()
        .route("/", get(home_handler))
        .route("/about", get(about_handler))
        .with_state(shared_state);

    // 4. Avvio del server
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 Server in ascolto su http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

// Handler per la Home Page
async fn home_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut context = Context::new();
    context.insert("nome_utente", "Mario");
    context.insert("data", "21 Dicembre 2025");
    match state.templates.render("index.html", &context) {
        Ok(rendered) => Html(rendered).into_response(),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Errore nel rendering").into_response(),
    }
}

// Handler per la pagina About
async fn about_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let context = Context::new(); // Pagina statica, contesto vuoto

    match state.templates.render("about.html", &context) {
        Ok(rendered) => Html(rendered).into_response(),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Errore nel rendering").into_response(),
    }
}