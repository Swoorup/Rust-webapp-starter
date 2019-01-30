use actix_web::{App,fs, http::{header, Method},middleware::{self,cors::Cors}};

use crate::model::db::init;
use crate::share::state::AppState;

use crate::api::{home::{index,path},auth::{signup, signin}};
use crate::api::article::{article,article_list, article_new};
use crate::api::user::{user_info, user_delete, user_update};

pub fn app_state() -> App<AppState> {
     let addr = init();
     App::with_state(AppState{ db: addr.clone()})
         .middleware(middleware::Logger::default())
         .prefix("/api")
         .configure(|app| Cors::for_app(app)
         .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
         .allowed_headers(vec![header::ORIGIN, header::AUTHORIZATION,header::ACCEPT, header::CONTENT_TYPE])
         .supports_credentials()
         .max_age(3600)
             .resource("/signup", |r| { r.method(Method::POST).with(signup); })
             .resource("/signin", |r| { r.method(Method::POST).with(signin); })
             .resource("/user_info", |r| { r.method(Method::GET).with(user_info); })
             .resource("/user_delete", |r| { r.method(Method::GET).with(user_delete); })
             .resource("/user_update", |r| { r.method(Method::POST).with(user_update); })
             .resource("/article_list", |r| { r.method(Method::GET).with(article_list); })
             .resource("/article_new", |r| { r.method(Method::POST).with(article_new); })
             .resource("/article/{article_id}", |r| { r.method(Method::GET).with(article);})
         .register())
}

pub fn app() -> App {
    App::new()
        .middleware(middleware::Logger::default())
        .resource("/", |r| r.f(index))
        .resource("/a/{tail:.*}", |r| r.f(path))
        .handler("/", fs::StaticFiles::new("public").unwrap())
}