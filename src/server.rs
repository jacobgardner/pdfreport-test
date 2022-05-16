use actix_web::{
    http::header::ContentType,
    web::{self, Bytes},
    App, HttpResponse, HttpServer,
};

use pdf_render::{build_pdf_from_dom, dom::PdfDom, BadPdfLayout};

async fn render_pdf(body: web::Json<PdfDom>) -> HttpResponse {
    let mut rendered_pdf_bytes = Vec::new();

    let response = build_pdf_from_dom(&body.0, &mut rendered_pdf_bytes);

    match response {
        Ok(_) => HttpResponse::Ok()
            .content_type(ContentType::octet_stream())
            .body(Bytes::copy_from_slice(&rendered_pdf_bytes)),
        Err(BadPdfLayout::UnknownError) => {
            // TODO: Fix & Remove
            HttpResponse::InternalServerError().body("We dont know what happened")
        },
        Err(_) => {
            // TODO: Return with actual error message
            HttpResponse::BadRequest().body("You did something wrong")
        }
    }
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().route("/", web::post().to(render_pdf)))
        .bind(("127.0.0.1", 1234))?
        .run()
        .await
}
