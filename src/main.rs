use axum::{extract::State, response::{Html, IntoResponse}, routing::get, Router};
//use rusqlite::{params, Connection, Result};
use rusqlite::{Connection,named_params};
//use rusqlite::{Connection, params};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tera::{Context, Tera};
use tower_http::services::ServeDir;
//newuse

// Definiamo la struttura dati per la tabella
#[derive(Serialize)]
struct Links{
	id:          i32,
	codice:      String,
	img:         String,
	titolo:      String,
	attivo:      i32,
	descrizione: String,
	link:        String,
	height:      String,
	width:       String,
}
#[derive(serde:: Serialize)]
struct Slider {
     id: i64,
     codice: String,
     codice2: String, // O String, a seconda del tuo DB
     img: String,
     titolo: String,
     caption: String,
     link: String,
     testo: String,
}
struct AppState {
    templates: Tera,
    // Usiamo Mutex perché la connessione SQLite non è "Thread Safe" di natura
    db_conn: Mutex<Connection>,
}

#[tokio::main]
async fn main() {
    // 1. Inizializza SQLite e crea una tabella di prova
    let db_path = "casabaldini.sqlite";
    let conn = Connection::open(db_path).expect("Impossibile trovare o aprire il file SQLite");

    //let conn = Connection::open_in_memory().expect("Errore apertura DB");
    //conn.execute(
    //    "CREATE TABLE utenti (id INTEGER PRIMARY KEY, nome TEXT, email TEXT)",
    //    (),
    //).unwrap();
    //conn.execute("INSERT INTO utenti (nome, email) VALUES ('Mario Rossi', 'mario@example.com')", ()).unwrap();
    //conn.execute("INSERT INTO utenti (nome, email) VALUES ('Paola Bianchi', 'paola@example.com')", ()).unwrap();

    // 2. Inizializza Tera
    let tera = Tera::new("templates/**/*").expect("Errore template");

    let shared_state = Arc::new(AppState {
        templates: tera,
        db_conn: Mutex::new(conn),
    });

    let app = Router::new()
        .route("/", get(home_handler))
        .route("/about", get(about_handler))
        //.route("/slider", get(slider_handler))
        .route("/lacasailpaese", get(lacasailpaese_handler))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(shared_state);

    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server attivo su http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn home_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // 3. Estraiamo i dati dal database
    let conn = state.db_conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id,codice,img,titolo,descrizione,link,attivo,height,width FROM beb_links").unwrap();
    
    let links_iter = stmt.query_map([], |row| {
        Ok(Links {
            id: row.get(0)?,
            codice: row.get(1)?,
            img: row.get(2)?,
            titolo: row.get(3)?,
            descrizione: row.get(4)?,
            link: row.get(5)?,
             attivo: row.get(6)?,
            height: row.get(7)?,
            width: row.get(8)?,
        })
    }).unwrap();

    let mut lista_links = Vec::new();
    for links in links_iter {
        lista_links.push(links.unwrap());
    }

    // 4. Passiamo la lista al template
    let mut context = Context::new();
    context.insert("links", &lista_links);

    let rendered = state.templates.render("index.html", &context).unwrap();
    Html(rendered)
}

async fn lacasailpaese_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // 3. Estraiamo i dati dal database
    let codice = "lasala";
    let conn = state.db_conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, codice, codice2, img, titolo, caption, link, testo FROM beb_slider WHERE codice2 = :codice").unwrap();
    
    let Slider_iter = stmt.query_map(named_params! { ":codice": codice }, |row| {
        Ok(Slider {
            id: row.get(0)?,
            codice: row.get(1)?,
            codice2: row.get(2)?,
            img: row.get(3)?,
            titolo: row.get(4)?,
            caption: row.get(5)?,
            link: row.get(6)?,
            testo: row.get(7)?,
        })
    }).unwrap();

    let mut lista_slider = Vec::new();
    for slider in Slider_iter {
        lista_slider.push(slider.unwrap());
    }

    // 4. Passiamo la lista al template
    let mut context = Context::new();
    context.insert("links", &lista_slider);

    let rendered = state.templates.render("slider.html", &context).unwrap();
    Html(rendered)
}
async fn about_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // 3. Estraiamo i dati dal database
    //let conn = state.db_conn.lock().unwrap();
    
let conn = state.db_conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id,codice,img,titolo,descrizione,link,attivo,height,width FROM beb_links").unwrap();
    
    let links_iter = stmt.query_map([], |row| {
        Ok(Links {
            id: row.get(0)?,
            codice: row.get(1)?,
            img: row.get(2)?,
            titolo: row.get(3)?,
            descrizione: row.get(4)?,
            link: row.get(5)?,
             attivo: row.get(6)?,
            height: row.get(7)?,
            width: row.get(8)?,
        })
    }).unwrap();
        
    let mut lista_links = Vec::new();
    for links in links_iter {
        lista_links.push(links.unwrap());
    }

    let mut context = Context::new();
    context.insert("links", &lista_links);

    let rendered = state.templates.render("about.html", &context).unwrap();
    Html(rendered)
}

