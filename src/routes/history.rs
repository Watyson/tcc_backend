use actix_web::{get, post, put, web, HttpResponse, Responder};

use crate::data::historic::DbHistoric;
use crate::models::{
    product_cart::ProductCart, product_cart_update::ProductCartUpdate,
    tokens::find_user_id_by_token,
};

// Cria um novo histórico de compra do usuário identificado pelo user_id e user_token com os dados fornecidos
//
// Retorna o código 200 (OK) se o historico de compras foi criado com sucesso.
// Retorna o código 401 (Unauthorized) se o token fornecido é inválido ou não corresponde ao ID do usuário fornecido.
// Retorna o código 500 (Internal Server Error) com uma mensagem de erro se houve problema ao criar o histórico de compra.
#[post("/api/v1/historic/create/{user_id}/{user_token}")]
async fn create(
    path: web::Path<(i32, String)>,
    purchase: web::Json<Vec<ProductCart>>,
) -> impl Responder {
    let (user_id, user_token) = (path.0, &path.1);

    match ProductCart::from_json(purchase) {
        Ok(items) => match find_user_id_by_token(user_token) {
            Some(user_id_token) if user_id_token == user_id => {
                match DbHistoric::create(&items, user_id).await {
                    Ok(_) => HttpResponse::Ok().finish(),
                    Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
                }
            }
            Some(_) => HttpResponse::Unauthorized().finish(),
            None => HttpResponse::Unauthorized().finish(),
        },
        Err(e) => {
            println!("Aqui o Erro: {}", e);
            return HttpResponse::InternalServerError().body(e.to_string());
        }
    }
}

// Busca o histórico de compras do usuário identificado pelo user_id e user_token
//
// Retorna o código 200 (Ok) se a leitura foi realizada com sucesso, junto com os dados do histórico de compras em formato JSON.
// Retorna o código 401 (Unauthorized) se o token fornecido é inválido ou não corresponde ao ID do usuário fornecido.
// Retorna o código 500 (Internal Server Error) com uma mensagem de erro se houve problema ao criar o histórico de compra.
#[get("/api/v1/historic/read/{user_id}/{user_token}")]
async fn read(path: web::Path<(i32, String)>) -> HttpResponse {
    let (user_id, user_token) = (path.0, &path.1);

    match find_user_id_by_token(user_token) {
        Some(user_id_token) if user_id_token == user_id => match DbHistoric::read(user_id).await {
            Ok(historic) => HttpResponse::Ok().json(historic),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        },
        Some(_) => HttpResponse::Unauthorized().finish(),
        None => HttpResponse::Unauthorized().finish(),
    }
}

// Atualiza o status e o tipo de pagamento de uma compra na tabela de histórico para um usuário autenticado.
//
// Retorna código de status 200 (OK) se a atualização for bem-sucedida
// Retorna código de status 401 (Unauthorized) se o usuário não estiver autenticado ou se o token fornecido for inválido
// Retorna código de status 500 (Internal Server Error) com uma mensagem de erro se houver algum problema ao atualizar o histórico
#[put("/api/v1/historic/update/")]
async fn update(items_update: web::Json<ProductCartUpdate>) -> HttpResponse {
    match ProductCartUpdate::from_json(items_update) {
        Ok(items) => {
            match DbHistoric::update(items.id_purchase(), items.status(), items.payment_type())
                .await
            {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        }
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    }

    /*
    match ProductCartUpdate::from_json(items_update) {
        Ok(items) => match find_user_id_by_token(user_token) {
            Some(user_id_token) if user_id_token == user_id => {
                match DbHistoric::update(items.id_purchase(), items.status(), items.payment_type())
                    .await
                {
                    Ok(_) => HttpResponse::Ok().finish(),
                    Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
                }
            }
            Some(_) => HttpResponse::Unauthorized().finish(),
            None => HttpResponse::Unauthorized().finish(),
        },
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    }
    */
}

// Busca o histórico de compras dos usuários
//
// Retorna o código 200 (Ok) se a leitura foi realizada com sucesso, junto com os dados do histórico de compras em formato JSON.
// Retorna o código 401 (Unauthorized) se o token fornecido é inválido ou não corresponde ao ID do usuário fornecido.
// Retorna o código 500 (Internal Server Error) com uma mensagem de erro se houve problema ao criar o histórico de compra.
#[get("/api/v1/historic/read_all/")]
async fn read_all() -> HttpResponse {
    match DbHistoric::read_all().await {
        Ok(historic) => HttpResponse::Ok().json(historic),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
