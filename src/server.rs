use actix_web::{get, post, web, App, HttpResponse, HttpServer};

use pdf_render::{build_pdf_from_dom, doc_structure::DocStructure, error::DocumentGenerationError};

fn pdf_response_from_dom(pdf_dom: DocStructure) -> HttpResponse {
    let filename = pdf_dom.filename.clone();
    let response = build_pdf_from_dom(&pdf_dom, Vec::new());

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
async fn render_pdf(body: web::Json<DocStructure>) -> HttpResponse {
    let pdf_dom = body.into_inner();

    pdf_response_from_dom(pdf_dom)
}

async fn heartbeat() -> HttpResponse {
    HttpResponse::Ok().body("Ba-bump")
}

#[cfg(feature = "develop")]
#[get("/test")]
async fn test_render_pdf() -> HttpResponse {
    let example_json = std::fs::read_to_string("./assets/example.json").unwrap();

    let pdf_dom: DocStructure = serde_json::from_str(&example_json).unwrap();

    pdf_response_from_dom(pdf_dom)
}

const DEFAULT_PORT: u16 = 8181;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let port = std::env::var("PORT").map_or(DEFAULT_PORT, |str| str.parse().unwrap_or(DEFAULT_PORT));

    let base_path = std::env::var("BASE_PATH").unwrap_or("/".to_owned());

    HttpServer::new(move || {
        let mut app = App::new().service(
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
