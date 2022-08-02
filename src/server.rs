use actix_web::{dev::Service, get, middleware::Logger, post, web, App, HttpResponse, HttpServer};

use logger::LogData;
use logzio::{LogzIoSender, LogzIoSenderBuilder};
use pdf_render::{build_pdf_from_dom, doc_structure::DocStructure, error::DocumentGenerationError};

use rollbar::{self, report_panics};
use tracing::{error, info, info_span, Instrument};
use tracing_subscriber::{filter, prelude::*, EnvFilter};

mod logger;

struct AppState {}

fn pdf_response_from_dom(pdf_dom: DocStructure, _app_state: &AppState) -> HttpResponse {
    let _span = info_span!("Beginning to parse PDF struct").entered();

    let filename = pdf_dom.filename.clone();
    let response = build_pdf_from_dom(&pdf_dom, Vec::new());

    info!("An event!");

    match response {
        Ok(rendered_bytes) => HttpResponse::Ok()
            .content_type("application/pdf")
            .append_header((
                "Content-Disposition",
                format!("inline; filename=\"{filename}\""),
            ))
            // Don't cache
            .append_header(("Cache-Control", "private"))
            .body(rendered_bytes),
        Err(DocumentGenerationError::InternalServerError(err)) => {
            HttpResponse::InternalServerError().body(err.to_string())
        }
        Err(DocumentGenerationError::UserInputError(err)) => {
            HttpResponse::BadRequest().body(err.to_string())
        }
    }
}

#[post("/")]
async fn render_pdf(body: web::Json<DocStructure>, data: web::Data<AppState>) -> HttpResponse {
    let pdf_dom = body.into_inner();

    pdf_response_from_dom(pdf_dom, &data)
}

async fn heartbeat() -> HttpResponse {
    let _span = info_span!("Beginning return heartbeat").entered();
    info!("Heartbeat!");
    HttpResponse::Ok().body("Ba-bump")
}

#[cfg(feature = "develop")]
#[get("/test")]
async fn test_render_pdf(data: web::Data<AppState>) -> HttpResponse {
    let _span = info_span!("Beginning test route!").entered();
    let example_json = std::fs::read_to_string("./assets/example.json").unwrap();

    let pdf_dom: DocStructure = serde_json::from_str(&example_json).unwrap();

    pdf_response_from_dom(pdf_dom, &data)
}

const DEFAULT_PORT: u16 = 8181;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let environment = std::env::var("ENV").unwrap_or_else(|_| "local".to_owned());
    let access_token = std::env::var("LOGGER_SERVER_TOKEN").ok();
    let logz_shipping_token = std::env::var("LOGGER_TRANSPORTS_REMOTE_TOKEN").ok();

    let logz_shipping_layer = if let Some(shipping_token) = logz_shipping_token {
        let logz_sender: LogzIoSender<LogData> =
            LogzIoSenderBuilder::new("listener.logz.io".to_owned(), shipping_token).build();

        Some(
            logz_sender
                .with_filter(filter::LevelFilter::INFO)
                .with_filter(filter::filter_fn(|metadata| {
                    metadata.target().starts_with("server")
                        || metadata.target().starts_with("pdf_render")
                })),
        )
    } else {
        None
    };

    let subscriber = tracing_subscriber::registry()
        .with(logz_shipping_layer)
        .with(tracing_subscriber::fmt::layer().with_filter(EnvFilter::from_default_env()));

    tracing::subscriber::set_global_default(subscriber).unwrap();

    if let Some(access_token) = access_token {
        let _ = info_span!("Logging panics!").entered();
        let rollbar_client = rollbar::Client::new(&access_token, &environment);

        report_panics!(rollbar_client);
    }

    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic_info| {
        let panic_msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            format!("Panic Occurred: {s:?}")
        } else {
            "Panic Occurred!".to_string()
        };

        error!(message = panic_msg.as_str());

        original_hook(panic_info);
    }));

    let port =
        std::env::var("PORT").map_or(DEFAULT_PORT, |str| str.parse().unwrap_or(DEFAULT_PORT));

    let base_path = std::env::var("BASE_PATH").unwrap_or_else(|_| "/".to_owned());

    info!("Starting server w/ port: {port} and base-path: {base_path}");

    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(Logger::default())
            .wrap_fn(|req, srv| {
                let request_id = req
                    .headers()
                    .get("x-request-id")
                    .map(|value| value.to_str().unwrap_or("Error Parsing Request Id"))
                    .unwrap_or("No Request Id")
                    .to_owned();

                let fut = srv
                    .call(req)
                    .instrument(info_span!("HTTP Request", request_id = request_id.as_str()));

                async {
                    let res = fut.await?;

                    Ok(res)
                }
            })
            .app_data(web::Data::new(AppState {}))
            .service(
                web::scope(&base_path)
                    .service(render_pdf)
                    .route("/health-check", web::get().to(heartbeat)),
            );

        // We don't want the test endpoint in prod builds
        #[cfg(feature = "develop")]
        {
            app = app.service(test_render_pdf);
        }

        app = app.route("/heartbeat", web::get().to(heartbeat));

        app
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
