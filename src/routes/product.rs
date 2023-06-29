use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

use crate::data::product::DbProduct;
use crate::models::product::Product;

// Cria um novo produto com os dados fornecidos.
//
// Retorna o código 201 (Created) se o produto foi criado com sucesso.
// Retorna o código 400 (Bad Request) com uma mensagem de erro se os dados fornecidos são inválidos.
// Retorna o código 500 (Internal Server Error) com uma mensagem de erro se houve problema ao criar o produto ou o nome já existir.
#[post("/api/v1/product/create")]
pub async fn create(product: web::Json<Product>) -> impl Responder {
    match Product::from_json(product) {
        Ok(new_product) => match DbProduct::create(new_product).await {
            Ok(_) => HttpResponse::Created().finish(),
            Err(e) => {
                HttpResponse::InternalServerError().body(format!("Failed to create product: {}", e))
            }
        },
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

// Obtém as informações do produto correspondente ao id fornecido.
//
// Retorna o código 200 (Ok) e um objeto JSON com as informações do produto se a operação for bem sucedida.
// Retorna o código 404 (Not Found) se não conseguiu encontrar o produto.
#[get("/api/v1/product/read/{id}")]
async fn read(path: web::Path<i32>) -> impl Responder {
    match DbProduct::read(*path).await {
        Ok(product) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .json(product),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

// Retorna uma lista de produtos dentro de um intervalo especificado.
//
// Retorna o código 200 (Ok) e um objeto JSON com as informações dos produtos se a operação for bem sucedida.
// Retorna o código 400 (Bad Request) com uma mensagem de erro se os dados são inválidos.
// Retorna o código 500 (Internal Server Error) com uma mensagem de erro se houver problema ao ler os produtos.
#[get("/api/v1/product/read/{start}/{end}/{only_avaliable}")]
async fn read_in_range(path: web::Path<(usize, usize, bool)>) -> impl Responder {
    let (start, limit, only_avaliable) = (path.0, path.1, path.2);

    match limit < start || limit - start > 100 {
        true => HttpResponse::BadRequest().body("Invalid range"),
        false => {
            match DbProduct::read_in_range(start.try_into().unwrap(), limit.try_into().unwrap(), only_avaliable).await {
                Ok(products) => HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .json(products),
                Err(e) => HttpResponse::InternalServerError()
                    .body(format!("Failed to read products: {}", e)),
            }
        }
    }
}

// Atualiza as informações de um produto existente se o id fornecido for valido.
//
// Retorna o código 200 (OK) se a operação for bem sucedida.
// Retorna o código 400 (Bad Request) com uma mensagem de erro se os dados são inválidos.
// Retorna o código 500 (Internal Server Error) com uma mensagem de erro se houver problema ao atualizar o produto.
#[put("/api/v1/product/update/{id}")]
async fn update(path: web::Path<i32>, product: web::Json<Product>) -> impl Responder {
    match Product::from_json(product) {
        Ok(new_product) => match DbProduct::update(*path, new_product).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => {
                HttpResponse::InternalServerError().body(format!("Failed to update product: {}", e))
            }
        },
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

// Remove um produto com o id fornecido
//
// Retorna o código 200 (Ok) se a operação for bem sucedida.
// Retorna o código 404 (Not Found) se o produto não for encontrado.
#[delete("/api/v1/product/delete/{id}")]
async fn delete(path: web::Path<i32>) -> impl Responder {
    match DbProduct::delete(*path).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
