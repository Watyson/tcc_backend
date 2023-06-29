use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::Deserialize;

use std::collections::HashMap;
use std::sync::Mutex;

// Estrutura para armazenar os códigos de recuperação
pub struct RecoveryCodes {
    pub codes: Mutex<HashMap<String, String>>, // E-mail -> Código de recuperação
}

impl RecoveryCodes {
    pub fn new() -> Self {
        RecoveryCodes {
            codes: Mutex::new(HashMap::new()),
        }
    }

    // Função para adicionar um código de recuperação
    fn add_code(&self, email: String, code: String) {
        let mut codes = self.codes.lock().unwrap();
        codes.insert(email, code);
    }

    pub fn remove_code(&self, email: &str) {
        let mut codes = self.codes.lock().unwrap();
        codes.remove(email);
    }

    // Função para verificar se um código de recuperação está correto
    fn is_valid_code(&self, email: &str, code: &str) -> bool {
        let codes = self.codes.lock().unwrap();
        if let Some(saved_code) = codes.get(email) {
            saved_code == code
        } else {
            false
        }
    }
}

#[derive(Deserialize)]
struct Email {
    email: String,
}

use crate::data::user::DbUser;
use crate::models::{
    tokens::{find_user_id_by_token, remove_token},
    user::User,
};

// Cria um novo usuário com os dados fornecidos.
//
// Retorna o código 201 (Created) se o usuário foi criado com sucesso.
// Retorna o código 400 (Bad Request) com uma mensagem de erro se os dados fornecidos são inválidos.
// Retorna o código 422 (Unprocessable Entity) com uma mensagem de erro se o username já existe.
// Retorna o código 500 (Internal Server Error) com uma mensagem de erro se houve problema ao criar o usuário.
#[post("/api/v1/user/create")]
pub async fn create(user: web::Json<User>) -> impl Responder {
    match User::from_json(user) {
        Ok(new_user) => match DbUser::create(new_user).await {
            Ok(_) => HttpResponse::Created().finish(),
            Err(e) => {
                if e.contains("Username already exists.") {
                    HttpResponse::UnprocessableEntity().body(e)
                } else {
                    HttpResponse::InternalServerError()
                        .body(format!("Failed to create user: {}", e))
                }
            }
        },
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

// Obtém as informações do usuário correspondente ao id fornecido, desde que o token fornecido seja válido.
//
// Retorna o código 200 (Ok) e um objeto JSON com as informações do usuário se a operação for bem sucedida.
// Retorna o código 401 (Unauthorized) se o token fornecido é inválido ou não corresponde ao ID do usuário fornecido.
// Retorna o código 404 (Not Found) se não conseguiu encontrar o usuario.
#[get("/api/v1/user/read/{id}/{token}")]
pub async fn read(path: web::Path<(i32, String)>) -> impl Responder {
    let (id, token) = (path.0, &path.1);

    match find_user_id_by_token(token) {
        Some(user_id) if user_id == id => match DbUser::read(id).await {
            Ok(user) => HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .json(user),
            Err(e) => HttpResponse::NotFound().body(e),
        },
        Some(_) => HttpResponse::Unauthorized().finish(),
        None => HttpResponse::Unauthorized().finish(),
    }
}

// Atualiza as informações de um usuário existente se o id e o token fornecidos forem validos.
//
// Retorna o código 200 (OK) se a operação for bem sucedida.
// Retorna o código 400 (Bad Request) com uma mensagem de erro se os dados são inválidos.
// Retorna o código 401 (Unauthorized) se o token fornecido é inválido ou não corresponde ao ID do usuário fornecido.
// Retorna o código 500 (Internal Server Error) com uma mensagem de erro se houver problema ao atualizar o usuário.
#[put("/api/v1/user/update/{id}/{token}")]
pub async fn update(path: web::Path<(i32, String)>, user: web::Json<User>) -> impl Responder {
    let (id, token) = (path.0, &path.1);

    match find_user_id_by_token(token) {
        Some(user_id) if user_id == id => match User::from_json(user) {
            Ok(new_user) => match DbUser::read_acess(id).await {
                Ok(acess) if Some(acess) >= new_user.acess() => {
                    match DbUser::update(new_user).await {
                        Ok(_) => HttpResponse::Ok().finish(),
                        Err(e) => HttpResponse::InternalServerError()
                            .body(format!("Failed to update user: {}", e)),
                    }
                }
                Ok(_) => HttpResponse::Unauthorized().finish(),
                Err(e) => HttpResponse::BadRequest().body(e),
            },
            Err(e) => HttpResponse::BadRequest().body(e),
        },
        Some(_) => HttpResponse::Unauthorized().finish(),
        None => HttpResponse::Unauthorized().finish(),
    }
}

// Remove um usuário com o id fornecido, desde que o token fornecido seja válido.
//
// Retorna o código 200 (OK) se a operação for bem sucedida.
// Retorna o código 401 (Unauthorized) se o token fornecido é inválido ou não corresponde ao ID do usuário fornecido.
// Retorna o código 404 (Not Found) se o usuário não for encontrado.
#[delete("/api/v1/user/delete/{id}/{token}")]
pub async fn delete(path: web::Path<(i32, String)>) -> impl Responder {
    let (id, token) = (path.0, &path.1);

    match find_user_id_by_token(token) {
        Some(user_id) if user_id == id => match DbUser::delete(id).await {
            Ok(_) => {
                remove_token(token);
                HttpResponse::Ok().finish()
            }
            Err(_) => HttpResponse::NotFound().finish(),
        },
        Some(_) => HttpResponse::Unauthorized().finish(),
        None => HttpResponse::Unauthorized().finish(),
    }
}

// Pega todos os funcionarios e administradores do sistema
//
// Retorna o código 200 (OK) se a operação for bem sucedida.
// Retorna o código 401 (Unauthorized) se o token fornecido é inválido ou não corresponde ao ID do usuário fornecido.
// Retorna o código 404 (Not Found) se o usuário não for encontrado.
#[get("/api/v1/user/get_adm")]
pub async fn get_admin() -> impl Responder {
    match DbUser::read_admins().await {
        Ok(users) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .json(users),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

#[post("/send_recovery_code")]
async fn send_recovery_code(
    email: web::Json<Email>,
    recovery_codes: web::Data<RecoveryCodes>,
) -> HttpResponse {
    // Gerar um código de recuperação aleatório
    let recovery_code: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    // Configurações do envio de e-mail
    let email_subject = "Recuperação de Senha";
    let email_body = format!("Seu código de recuperação é: {}", recovery_code);
    let email_from = "watysonsoares@hotmail.com"; // Substitua pelo seu e-mail
    let email_password = "rfxojwrgdydkapmm"; // Substitua pela sua senha
    let smtp_server = "smtp.office365.com";
    let smtp_port = 587;

    // Criação da mensagem de e-mail
    let email_message = Message::builder()
        .from(email_from.parse().unwrap())
        .to(email.email.clone().parse().unwrap())
        .subject(email_subject)
        .body(email_body)
        .unwrap();

    // Configuração do transporte SMTP
    let smtp_credentials = Credentials::new(email_from.to_string(), email_password.to_string());
    let smtp_transport = SmtpTransport::starttls_relay(smtp_server)
        .unwrap()
        .credentials(smtp_credentials)
        .port(smtp_port)
        .build();

    // Envio do e-mail
    match smtp_transport.send(&email_message) {
        Ok(_) => {
            // Adicionar o código de recuperação ao mecanismo de armazenamento
            recovery_codes.add_code(email.email.clone(), recovery_code.clone());
            HttpResponse::Ok().json("Código de recuperação enviado com sucesso.")
        }
        Err(e) => {
            eprintln!("Erro ao enviar e-mail: {:?}", e);
            HttpResponse::InternalServerError().json("Erro ao enviar o e-mail.")
        }
    }
}

#[get("/change_password/{email}/{code}/{password}")]
async fn change_password(
    path: web::Path<(String, String, String)>,
    recovery_codes: web::Data<RecoveryCodes>,
) -> HttpResponse {
    let email = path.0.clone();
    let code = &path.1;
    let password = &path.2;

    // Verificar se o código de recuperação é válido
    let is_valid = recovery_codes.is_valid_code(&email.clone(), code);

    if is_valid {
        recovery_codes.remove_code(&email);
        match DbUser::update_password(&email, password).await {
            Ok(_) => return HttpResponse::Ok().json("Senha alterada com sucesso."),
            Err(e) => return HttpResponse::BadRequest().body(e),
        };
    } else {
        return HttpResponse::BadRequest().json("Código de recuperação inválido.");
    }
}
