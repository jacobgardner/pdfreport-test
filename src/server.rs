use actix_web::{get, post, web, App, HttpResponse, HttpServer};

use pdf_render::{build_pdf_from_dom, dom::PdfDom, error::PdfGenerationError};

fn pdf_response_from_dom(pdf_dom: PdfDom) -> HttpResponse {
    let filename = pdf_dom.filename.clone();
    let response = build_pdf_from_dom(&pdf_dom, Vec::new());

    match response {
        Ok(rendered_bytes) => HttpResponse::Ok()
            .content_type("application/pdf")
            .append_header((
                "Content-Disposition",
                format!("attachment; filename=\"{filename}\""),
            ))
            // Don't cache
            .append_header(("Cache-Control", "private"))
            .body(rendered_bytes),
        Err(PdfGenerationError::InternalServerError(err)) => {
            HttpResponse::InternalServerError().body(err.to_string())
        }
        Err(PdfGenerationError::UserInputError(err)) => {
            HttpResponse::BadRequest().body(err.to_string())
        }
    }
}

#[post("/")]
async fn render_pdf(body: web::Json<PdfDom>) -> HttpResponse {
    let pdf_dom = body.into_inner();

    pdf_response_from_dom(pdf_dom)
}

#[cfg(feature = "develop")]
#[get("/test")]
async fn test_render_pdf() -> HttpResponse {
    let example_json = std::fs::read_to_string("./assets/example.json").unwrap();

    let pdf_dom: PdfDom = serde_json::from_str(&example_json).unwrap();

    pdf_response_from_dom(pdf_dom)
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        let mut app = App::new().service(render_pdf);

        // We don't want the test endpoint in prod builds
        #[cfg(feature = "develop")]
        {
            app = app.service(test_render_pdf);
        }

        app
    })
    .bind(("127.0.0.1", 1234))?
    .run()
    .await
}
