use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    //HttpResponse:Ok().finish() pode omitir o .finish
    HttpResponse::Ok().finish()
}

// #[cfg(test)]
// mod tests {
//     use crate::health_check;
//
//     #[tokio::test]
//     async fn health_check_succeeds() {
//         let response = health_check().await;
//         assert!(response.status().is_success())
//     }
// }
