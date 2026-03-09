use actix_multipart::Multipart;
use actix_web::{HttpResponse, Responder};
use futures_util::StreamExt;
use serde_json::json;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

pub async fn upload_image(mut payload: Multipart) -> impl Responder {
    tokio::fs::create_dir_all("uploads").await.ok();

    while let Some(Ok(mut field)) = payload.next().await {
        let filename = format!("{}.png", Uuid::new_v4());
        let filepath = format!("uploads/{}", filename);

        match tokio::fs::File::create(&filepath).await {
            Ok(mut file) => {
                while let Some(Ok(chunk)) = field.next().await {
                    if file.write_all(&chunk).await.is_err() {
                        return HttpResponse::InternalServerError().body("Write error");
                    }
                }
                return HttpResponse::Ok().json(json!({
                    "url": format!("/uploads/{}", filename)
                }));
            }
            Err(_) => return HttpResponse::InternalServerError().body("File create error"),
        }
    }

    HttpResponse::BadRequest().body("No file provided")
}
