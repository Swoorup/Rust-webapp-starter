use actix_web::actix::Addr;
use crate::model::db::ConnDsl;

pub struct AppState {
    pub db: Addr<ConnDsl>,
}
