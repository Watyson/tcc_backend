use actix_web::{delete, get, post, web, HttpResponse, Responder};

use crate::data::user::DbUser;
use crate::models::{
    login_response::LoginResponse,
    credentials::Credentials,
    tokens::{find_user_id_by_token, generate_token, remove_token},
};

// Autentica um usuário com as credenciais fornecidas via requisição HTTP POST na rota '/api/v1/authenticate'.
//
// Retorna o codigo 200 (OK) com um token de autenticação se as credenciais estiverem corretas.
// Retorna o código 400 (Bad Request) com uma mensagem de erro se os dados fornecidos são inválidos.
// Retorna o código 401 (Unauthorized) se as credenciais estiverem incorretas ou não existirem.
#[post("/api/v1/authenticate/login")]
async fn login(credentials: web::Json<Credentials>) -> impl Responder {
    match Credentials::from_json(credentials) {
        Ok(credentials) => {
            match DbUser::autenticate(credentials.username(), credentials.password()).await {
                Ok(id) => match generate_token(id) {
                    Ok(token) => HttpResponse::Ok().json(LoginResponse::new(id, token)),
                    Err(e) => HttpResponse::BadRequest().body(e),
                },
                Err(_) => HttpResponse::Unauthorized().finish(),
            }
        }
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

// Finalização a autenticação do usuario.
//
// Retorna o código 200 (Ok) se a operação for bem sucedida.
// Retorna o código 401 (Unauthorized) se não houver token de autenticação válido.
#[delete("/api/v1/authenticate/logout/{id}/{token}")]
pub async fn logout(path: web::Path<(String, String)>) -> impl Responder {
    let (id, token) = (&path.0, &path.1);

    match find_user_id_by_token(token) {
        Some(user_id) if user_id.to_string() == *id => {
            remove_token(token);
            HttpResponse::Ok().finish()
        }
        Some(_) => HttpResponse::Unauthorized().finish(),
        None => HttpResponse::Unauthorized().finish(),
    }
}

// Verifica se um usuário está autenticado a partir de um token de autenticação.
//
// Retorna o código 200 (Ok) se a operação for bem sucedida.
// Retorna o código 401 (Unauthorized) se não houver token de autenticação válido.
#[get("/api/v1/authenticate/check/{id}/{token}")]
async fn check(path: web::Path<(String, String)>) -> impl Responder {
    let (id, token) = (&path.0, &path.1);

    match find_user_id_by_token(token) {
        Some(user_id) if user_id.to_string() == *id => {
            HttpResponse::Ok().finish()
        }
        Some(_) => HttpResponse::Unauthorized().finish(),
        None => HttpResponse::Unauthorized().finish(),
    }
}
