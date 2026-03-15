use axum::{
    extract::State,
    http::{header, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
    Json, Router,
};
use chrono::Datelike;
use pulldown_cmark::{html::push_html, Options, Parser};
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use std::{env, net::SocketAddr, path::PathBuf, sync::Arc};
use tera::{Context, Tera};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

#[derive(Clone)]
struct AppState {
    client: Client,
    projects_path: PathBuf,
    profiles_path: PathBuf,
    contact_path: PathBuf,
    tagline_path: PathBuf,
    footer_path: PathBuf,
    templates: Arc<Tera>,
}

#[derive(Serialize)]
struct LastfmResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    track_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    artist_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    album_art: Option<String>,
    now_playing: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Serialize)]
struct TagsResponse {
    ok: bool,
    tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Serialize)]
struct TemplateTag {
    label: String,
    url: String,
}

#[derive(Serialize)]
struct InitialLastfm {
    ok: bool,
    track_name: String,
    artist_name: String,
    album_art: String,
}

impl InitialLastfm {
    fn connecting() -> Self {
        Self {
            ok: false,
            track_name: String::new(),
            artist_name: String::new(),
            album_art: String::new(),
        }
    }
}

enum LastfmFetchError {
    MissingKey(String),
    Upstream(String),
}

#[tokio::main]
async fn main() {
    let client = Client::builder()
        .user_agent("kierand.dev-rust-server")
        .build()
        .expect("failed to build reqwest client");

    let templates = Tera::new("templates/**/*").expect("failed to initialize tera templates");

    let state = Arc::new(AppState {
        client,
        projects_path: PathBuf::from("content/projects.md"),
        profiles_path: PathBuf::from("content/profiles.md"),
        contact_path: PathBuf::from("content/contact.md"),
        tagline_path: PathBuf::from("content/tagline.md"),
        footer_path: PathBuf::from("content/footer.md"),
        templates: Arc::new(templates),
    });

    let app = Router::new()
        .route("/", get(home))
        .route("/cv", get(cv_redirect))
        .route("/cv.pdf", get(cv_pdf))
        .route("/favicon.ico", get(favicon))
        .route("/favicon.png", get(favicon))
        .route("/api/lastfm", get(api_lastfm))
        .route("/api/lastfm/tags", get(api_lastfm_tags))
        .route_service("/app.css", ServeFile::new("static/app.css"))
        .route_service("/app.js", ServeFile::new("static/app.js"))
        .route_service("/robots.txt", ServeFile::new("static/robots.txt"))
        .nest_service("/fonts", ServeDir::new("static/fonts"))
        .nest_service("/images", ServeDir::new("static/images"))
        .nest_service("/.well-known", ServeDir::new("static/.well-known"))
        .fallback(get(catchall_redirect))
        .with_state(state);

    let port = env::var("PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(4173);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP listener");

    println!("Rust server listening on http://{}", addr);

    axum::serve(listener, app)
        .await
        .expect("server exited unexpectedly");
}

async fn home(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let projects_fut = tokio::fs::read_to_string(&state.projects_path);
    let profiles_fut = tokio::fs::read_to_string(&state.profiles_path);
    let contact_fut = tokio::fs::read_to_string(&state.contact_path);
    let tagline_fut = tokio::fs::read_to_string(&state.tagline_path);
    let footer_fut = tokio::fs::read_to_string(&state.footer_path);
    let tags_fut = fetch_top_tags(&state.client);
    let lastfm_fut = fetch_currently_scrobbling(&state.client);

    let (
        projects_result,
        profiles_result,
        contact_result,
        tagline_result,
        footer_result,
        tags_result,
        lastfm_result,
    ) = tokio::join!(
        projects_fut,
        profiles_fut,
        contact_fut,
        tagline_fut,
        footer_fut,
        tags_fut,
        lastfm_fut
    );

    let projects_markdown = projects_result.unwrap_or_else(|_| {
        "- **Projects unavailable** — Could not read `content/projects.md`.".to_string()
    });
    let profiles_markdown = profiles_result.unwrap_or_else(|_| {
        "- **Profiles unavailable** — Could not read `content/profiles.md`.".to_string()
    });
    let contact_markdown = contact_result.unwrap_or_else(|_| {
        "- **Contact unavailable** — Could not read `content/contact.md`.".to_string()
    });
    let tagline_markdown =
        tagline_result.unwrap_or_else(|_| "Software Engineer based in the UK.".to_string());
    let footer_markdown = footer_result.unwrap_or_else(|_| "© Kieran Drewett".to_string());

    let initial_tags = tags_result.unwrap_or_default();
    let projects_html = render_markdown(&preprocess_markdown(&projects_markdown));
    let profiles_html = render_markdown(&preprocess_markdown(&profiles_markdown));
    let contact_html = render_markdown(&preprocess_markdown(&contact_markdown));
    let tagline_html = render_markdown(&preprocess_markdown(&tagline_markdown));
    let footer_html = render_markdown(&preprocess_markdown(&footer_markdown));
    let initial_lastfm = match lastfm_result {
        Ok(payload) => InitialLastfm {
            ok: payload.ok,
            track_name: payload.track_name.unwrap_or_default(),
            artist_name: payload.artist_name.unwrap_or_default(),
            album_art: payload
                .album_art
                .unwrap_or_else(|| "/images/album_art.svg".to_string()),
        },
        Err(_) => InitialLastfm::connecting(),
    };

    let mut context = Context::new();
    context.insert("projects_html", &projects_html);
    context.insert("profiles_html", &profiles_html);
    context.insert("contact_html", &contact_html);
    context.insert("tagline_html", &tagline_html);
    context.insert("footer_html", &footer_html);
    context.insert("initial_tags", &build_template_tags(&initial_tags));
    context.insert("initial_lastfm", &initial_lastfm);

    match state.templates.render("index.html", &context) {
        Ok(rendered) => Html(rendered).into_response(),
        Err(error) => plain_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("template render failed: {error}"),
        ),
    }
}

async fn cv_redirect() -> impl IntoResponse {
    Redirect::temporary("/cv.pdf")
}

async fn catchall_redirect() -> impl IntoResponse {
    Redirect::temporary("/")
}

async fn favicon(State(state): State<Arc<AppState>>) -> Response {
    match state
        .client
        .get("https://github.com/kierandrewett.png")
        .send()
        .await
    {
        Ok(response) => match response.bytes().await {
            Ok(bytes) => bytes_response(
                StatusCode::OK,
                "image/png",
                bytes.to_vec(),
                Some((
                    header::CACHE_CONTROL,
                    HeaderValue::from_static("max-age=604800"),
                )),
            ),
            Err(error) => plain_error(
                StatusCode::BAD_GATEWAY,
                &format!("favicon fetch failed: {error}"),
            ),
        },
        Err(error) => plain_error(
            StatusCode::BAD_GATEWAY,
            &format!("favicon request failed: {error}"),
        ),
    }
}

async fn cv_pdf(State(state): State<Arc<AppState>>) -> Response {
    let release = match state
        .client
        .get("https://api.github.com/repos/EnderDev/cv/releases/latest")
        .send()
        .await
    {
        Ok(response) => response,
        Err(error) => {
            return plain_error(
                StatusCode::BAD_GATEWAY,
                &format!("cv release request failed: {error}"),
            )
        }
    };

    let payload: Value = match release.json().await {
        Ok(payload) => payload,
        Err(error) => {
            return plain_error(
                StatusCode::BAD_GATEWAY,
                &format!("cv release parse failed: {error}"),
            )
        }
    };

    let url = match payload
        .get("assets")
        .and_then(|value| value.as_array())
        .and_then(|assets| assets.first())
        .and_then(|asset| asset.get("browser_download_url"))
        .and_then(|value| value.as_str())
    {
        Some(url) => url,
        None => return plain_error(StatusCode::BAD_GATEWAY, "cv release asset missing"),
    };

    match state.client.get(url).send().await {
        Ok(response) => match response.bytes().await {
            Ok(bytes) => {
                let mut res =
                    bytes_response(StatusCode::OK, "application/pdf", bytes.to_vec(), None);
                res.headers_mut().insert(
                    header::CONTENT_DISPOSITION,
                    HeaderValue::from_static("inline; filename=cv.pdf"),
                );
                res
            }
            Err(error) => plain_error(
                StatusCode::BAD_GATEWAY,
                &format!("cv pdf fetch failed: {error}"),
            ),
        },
        Err(error) => plain_error(
            StatusCode::BAD_GATEWAY,
            &format!("cv pdf request failed: {error}"),
        ),
    }
}

async fn api_lastfm(State(state): State<Arc<AppState>>) -> Response {
    match fetch_currently_scrobbling(&state.client).await {
        Ok(payload) => (StatusCode::OK, Json(payload)).into_response(),
        Err(LastfmFetchError::MissingKey(error)) => (
            StatusCode::BAD_REQUEST,
            Json(LastfmResponse {
                ok: false,
                track_name: None,
                artist_name: None,
                album_art: None,
                now_playing: false,
                error: Some(error),
            }),
        )
            .into_response(),
        Err(LastfmFetchError::Upstream(error)) => (
            StatusCode::BAD_GATEWAY,
            Json(LastfmResponse {
                ok: false,
                track_name: None,
                artist_name: None,
                album_art: None,
                now_playing: false,
                error: Some(error),
            }),
        )
            .into_response(),
    }
}

async fn fetch_currently_scrobbling(client: &Client) -> Result<LastfmResponse, LastfmFetchError> {
    let api_key = match env::var("LASTFM_API_KEY") {
        Ok(value) if !value.is_empty() => value,
        _ => {
            return Err(LastfmFetchError::MissingKey(
                "No API key provided.".to_string(),
            ))
        }
    };

    let response = client
        .get("https://ws.audioscrobbler.com/2.0")
        .query(&[
            ("method", "user.getrecenttracks"),
            ("user", "EnderDev"),
            ("api_key", api_key.as_str()),
            ("format", "json"),
            ("limit", "1"),
        ])
        .send()
        .await
        .map_err(|error| LastfmFetchError::Upstream(error.to_string()))?;

    let payload: Value = response
        .json()
        .await
        .map_err(|error| LastfmFetchError::Upstream(error.to_string()))?;

    let track = payload
        .get("recenttracks")
        .and_then(|value| value.get("track"))
        .and_then(|value| value.as_array())
        .and_then(|tracks| tracks.first());

    let track_name = track
        .and_then(|value| value.get("name"))
        .and_then(|value| value.as_str())
        .unwrap_or("Track");
    let artist_name = track
        .and_then(|value| value.get("artist"))
        .and_then(|value| value.get("#text"))
        .and_then(|value| value.as_str())
        .unwrap_or("Artist");
    let album_art = track
        .and_then(|value| value.get("image"))
        .and_then(|value| value.as_array())
        .and_then(|images| images.last())
        .and_then(|value| value.get("#text"))
        .and_then(|value| value.as_str())
        .filter(|value| !value.is_empty())
        .unwrap_or("/images/album_art.svg");
    let is_playing = track
        .and_then(|value| value.get("@attr"))
        .and_then(|value| value.get("nowplaying"))
        .and_then(|value| value.as_str())
        .map(|value| value == "true")
        .unwrap_or(false);

    Ok(LastfmResponse {
        ok: true,
        track_name: is_playing.then(|| track_name.to_string()),
        artist_name: is_playing.then(|| artist_name.to_string()),
        album_art: Some(album_art.to_string()),
        now_playing: is_playing,
        error: None,
    })
}

async fn api_lastfm_tags(State(state): State<Arc<AppState>>) -> Response {
    match fetch_top_tags(&state.client).await {
        Ok(tags) => (
            StatusCode::OK,
            Json(TagsResponse {
                ok: true,
                tags,
                error: None,
            }),
        )
            .into_response(),
        Err(error) => (
            StatusCode::BAD_GATEWAY,
            Json(TagsResponse {
                ok: false,
                tags: vec![],
                error: Some(error),
            }),
        )
            .into_response(),
    }
}

async fn fetch_top_tags(client: &Client) -> Result<Vec<String>, String> {
    let response = client
        .get("https://raw.githubusercontent.com/kierandrewett/lastfm-data/main/week_10_tags.json")
        .send()
        .await
        .map_err(|error| error.to_string())?;

    let tags = response
        .json::<Vec<String>>()
        .await
        .map_err(|error| error.to_string())?;

    Ok(tags)
}

fn build_template_tags(tags: &[String]) -> Vec<TemplateTag> {
    tags.iter()
        .take(7)
        .map(|tag| TemplateTag {
            label: title_case_tag(tag),
            url: format!("https://www.last.fm/tag/{}", tag.to_lowercase()),
        })
        .collect()
}

fn build_template_context() -> tera::Context {
    let now = chrono::Local::now();
    let mut ctx = tera::Context::new();
    ctx.insert("current_year", &now.year());
    ctx.insert("current_month", &now.format("%B").to_string());
    ctx.insert("current_date", &now.format("%Y-%m-%d").to_string());
    ctx
}

fn preprocess_markdown(content: &str) -> String {
    let ctx = build_template_context();
    Tera::one_off(content, &ctx, false).unwrap_or_else(|_| content.to_string())
}

fn render_markdown(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(markdown, options);
    let mut output = String::new();
    push_html(&mut output, parser);
    output.replace(
        "<a href=\"",
        "<a target=\"_blank\" rel=\"noreferrer\" href=\"",
    )
}

fn title_case_tag(tag: &str) -> String {
    tag.split(' ')
        .map(|word| {
            word.split('-')
                .map(|part| {
                    let mut chars = part.chars();
                    match chars.next() {
                        Some(first) => {
                            let mut output = first.to_uppercase().collect::<String>();
                            output.push_str(chars.as_str());
                            output
                        }
                        None => String::new(),
                    }
                })
                .collect::<Vec<_>>()
                .join("-")
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn bytes_response(
    status: StatusCode,
    content_type: &'static str,
    bytes: Vec<u8>,
    extra_header: Option<(header::HeaderName, HeaderValue)>,
) -> Response {
    let mut response = Response::new(axum::body::Body::from(bytes));
    *response.status_mut() = status;
    response
        .headers_mut()
        .insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));

    if let Some((name, value)) = extra_header {
        response.headers_mut().insert(name, value);
    }

    response
}

fn plain_error(status: StatusCode, message: &str) -> Response {
    (status, message.to_string()).into_response()
}
