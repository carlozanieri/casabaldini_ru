use axum::{extract::State, response::{Html, IntoResponse}, routing::get, Router};
use rusqlite::{Connection, Result};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tera::{Context, Tera};

// Definiamo la struttura dati per la tabella
#[derive(Serialize)]
struct Utente {
    id: i32,
    nome: String,
    email: String,
}

struct AppState {
    templates: Tera,
    // Usiamo Mutex perché la connessione SQLite non è "Thread Safe" di natura
    db_conn: Mutex<Connection>,
}

#[tokio::main]
async fn main() {
    // 1. Inizializza SQLite e crea una tabella di prova
    let conn = Connection::open_in_memory().expect("Errore apertura DB");
    conn.execute(
        "CREATE TABLE utenti (id INTEGER PRIMARY KEY, nome TEXT, email TEXT)",
        (),
    ).unwrap();
    conn.execute("INSERT INTO utenti (nome, email) VALUES ('Mario Rossi', 'mario@example.com')", ()).unwrap();
    conn.execute("INSERT INTO utenti (nome, email) VALUES ('Paola Bianchi', 'paola@example.com')", ()).unwrap();

    // 2. Inizializza Tera
    let tera = Tera::new("templates/**/*").expect("Errore template");

    let shared_state = Arc::new(AppState {
        templates: tera,
        db_conn: Mutex::new(conn),
    });

    let app = Router::new()
        .route("/", get(home_handler))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server attivo su http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn home_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // 3. Estraiamo i dati dal database
    let conn = state.db_conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, nome, email FROM utenti").unwrap();
    
    let utenti_iter = stmt.query_map([], |row| {
        Ok(Utente {
            id: row.get(0)?,
            nome: row.get(1)?,
            email: row.get(2)?,
        })
    }).unwrap();

    let mut lista_utenti = Vec::new();
    for utente in utenti_iter {
        lista_utenti.push(utente.unwrap());
    }

    // 4. Passiamo la lista al template
    let mut context = Context::new();
    context.insert("utenti", &lista_utenti);

    let rendered = state.templates.render("index.html", &context).unwrap();
    Html(rendered)
}