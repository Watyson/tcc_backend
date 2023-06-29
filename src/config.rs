use crate::routes::auth::{check as user_check, login as user_login, logout as user_logout};
use crate::routes::history::{
    create as hist_create, read as hist_read, read_all, update as hist_update,
};
use crate::routes::product::{
    create as prod_create, delete as prod_delete, read as prod_read,
    read_in_range as prod_read_in_range, update as prod_update,
};
use crate::routes::user::{
    create as user_create, delete as user_delete, get_admin, read as user_read, send_recovery_code,
    update as user_update, change_password,
};

use actix_files::Files;
use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Historic
        .service(hist_create)
        .service(hist_read)
        .service(hist_update)
        .service(read_all)
        // Product
        .service(prod_create)
        .service(prod_read)
        .service(prod_read_in_range)
        .service(prod_update)
        .service(prod_delete)
        // User
        .service(user_create)
        .service(user_read)
        .service(user_update)
        .service(user_delete)
        // Authenticate
        .service(user_login)
        .service(user_check)
        .service(user_logout)
        .service(get_admin)
        // Utils
        .service(Files::new("/images", "./src/images/"))
        .service(send_recovery_code)
        .service(change_password);
    //\images\almondegas.png
}
