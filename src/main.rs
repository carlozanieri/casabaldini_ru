use axum::{extract::State, response::{Html, IntoResponse}, routing::get, Router};
use sqlx::{SqlitePool, FromRow};
use std::sync::Arc;
use tera::Tera;
use serde::Serialize;
use tower_http::services::ServeDir;

//newuse

// Definiamo la struttura dati per la tabella
#[derive(Serialize, FromRow)]
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
#[derive(Serialize, FromRow)]
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

#[derive(Serialize, FromRow)]
struct Menus {
	id:       i64,
	codice:   String,
	radice:   String,
	livello:  i64,
	titolo:   String,
	link:     String,
	
}
#[derive(Serialize, FromRow)]
struct Submenus{
	id:       i64,
	codice:   String,
	radice:   String,
	livello:  i64,
	titolo:   String,
	link:     String,
	
}

struct AppState {
    db: SqlitePool,
    //tera: tera::Tera,
    templates: Tera,
    
}

#[derive(Serialize)]
struct BaseContext {
    menu: Vec<Menus>,
    submenu: Vec<Submenus>,
    links: Vec<Links>,
    slide: Vec<Slider>,
 
}


#[tokio::main]
async fn main()  {
    // 1. Inizializza SQLite e crea una tabella di prova
    //let db_path = "casabaldini.sqlite";
    
   // let tera = Tera::new("templates/**/*").expect("Errore template");
    let templates  = Tera::new("templates/**/*").expect("Errore template");
    let db = SqlitePool::connect("sqlite:casabaldini.sqlite").await.unwrap();
    let shared_state = Arc::new(AppState {
        db,
        //tera,
        templates,
    });

    let app = Router::new()
        .route("/", get(home_handler))
        .route("/about", get(about_handler))
        .route("/prenotazioni", get(prenotazioni_handler))
        .route("/menu", get(menu_handler))
        .route("/lacasailpaese", get(lacasailpaese_handler))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(shared_state);

    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server attivo su http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
   
async fn home_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
//use axum::{extract::State, response::Html};
use tera::Context;
 
let base =  get_base_context(&state.db).await;
let codice = "index";
//let slider: BaseContext = get_slide_context(&state.db, &codice).await?;

    // 2. Crea il contesto per Tera e inserisci i dati comuni
    let mut ctx = Context::new();
    ctx.insert("menu", &base.menu);
    ctx.insert("submenu", &base.submenu);
    ctx.insert("sliders", &base.slide);
    ctx.insert("links", &base.links);
    ctx.insert("pagina_titolo", "Home Page");
    
    let rendered = state.templates.render("home.html", &ctx).unwrap();
    Html(rendered)
}

async fn lacasailpaese_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    
//use axum::{extract::State, response::Html};
use tera::Context;
//use std::sync::Arc;
    let base =  get_base_context(&state.db).await;
let codice = "index";
//let slider: BaseContext = get_slide_context(&state.db, &codice).await?;

    // 2. Crea il contesto per Tera e inserisci i dati comuni
    let mut ctx = Context::new();
    ctx.insert("menu", &base.menu);
    ctx.insert("submenu", &base.submenu);
    ctx.insert("sliders", &base.slide);
    ctx.insert("links", &base.links);
    ctx.insert("pagina_titolo", "Home Page");
    
    let rendered = state.templates.render("slider.html", &ctx).unwrap();
    Html(rendered)
}

async fn menu_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
//use axum::{extract::State, response::Html};
use tera::Context;
//use std::sync::Arc;
let base =  get_base_context(&state.db).await;
let codice = "index";
    // 2. Crea il contesto per Tera e inserisci i dati comuni
    let mut ctx = Context::new();
    ctx.insert("menu", &base.menu);
    ctx.insert("submenu", &base.submenu);
    ctx.insert("sliders", &base.slide);
    ctx.insert("links", &base.links);
    // 3. Aggiungi dati specifici per questa pagina
    ctx.insert("pagina_titolo", "Home Page");
    
    let rendered = state.templates.render("AceMenu.html", &ctx).unwrap();
    Html(rendered)
}

async fn about_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
//use axum::{extract::State, response::Html};
use tera::Context;
 
let base =  get_base_context(&state.db).await;
let codice = "index";
//let slider: BaseContext = get_slide_context(&state.db, &codice).await?;

    // 2. Crea il contesto per Tera e inserisci i dati comuni
    let mut ctx = Context::new();
    ctx.insert("menu", &base.menu);
    ctx.insert("submenu", &base.submenu);
    ctx.insert("sliders", &base.slide);
    ctx.insert("links", &base.links);
    ctx.insert("pagina_titolo", "Home Page");
    
    let rendered = state.templates.render("linkutili.html", &ctx).unwrap();
    Html(rendered)
}

async fn prenotazioni_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
//use axum::{extract::State, response::Html};
use tera::Context;
 
let base =  get_base_context(&state.db).await;
let codice = "index";
//let slider: BaseContext = get_slide_context(&state.db, &codice).await?;

    // 2. Crea il contesto per Tera e inserisci i dati comuni
    let mut ctx = Context::new();
    ctx.insert("menu", &base.menu);
    ctx.insert("submenu", &base.submenu);
    ctx.insert("sliders", &base.slide);
    ctx.insert("links", &base.links);
    ctx.insert("pagina_titolo", "Home Page");
    
    let rendered = state.templates.render("prenotazioni.html", &ctx).unwrap();
    Html(rendered)
}

async fn get_base_context(pool: &SqlitePool,) -> BaseContext {
    let menu = sqlx::query_as::<_, Menus>("SELECT id, codice,  radice, livello, titolo,link FROM menu where livello=2 and attivo= 1 order by ordine")
        .fetch_all(pool)
        .await
        .unwrap_or_default();

    let submenu = sqlx::query_as::<_, Submenus>("SELECT id, codice,  radice, livello, titolo,link FROM submenu where attivo = 1 order by ordine")
        .fetch_all(pool)
        .await
        .unwrap_or_default();

    let links = sqlx::query_as::<_, Links>("SELECT id,codice,img,titolo,descrizione,link,attivo,height,width FROM beb_links")
        .fetch_all(pool)
        .await
        .unwrap_or_default();
    let slide = sqlx::query_as::<_, Slider>("SELECT id, codice, codice2, img, titolo, caption, link, testo
        FROM beb_slider
        WHERE codice2 = 'lasala'")
        .fetch_all(pool)
        .await
        .unwrap_or_default();


    BaseContext { menu, submenu, links, slide }
}

