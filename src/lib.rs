pub mod configuration;
pub mod routes;
pub mod startup;

// impl Responder significa que retorna uma implementação de Responder
// implementa caso possa ser convertido em um HttpResponse (string, status codes, bytes, HttpResponse etc)
// nem todos handlers precisam retornar impl Responder, actix-web faz magia negra por trás da cortina
// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }
