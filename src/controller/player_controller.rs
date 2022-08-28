
use std::sync::{Mutex, Arc};
use actix_web::{post, get, put, web};
use actix_web::web::{Json, Path};
use actix_web::HttpResponse;
use uuid::Uuid;
use crate::application::player_app_service::{CreatePlayerCmd, PlayerAppService, self};
use crate::controller::APPLICATION_JSON;

/// create a player `/players`
#[post("/players")]
pub async fn create(
    create_player_cmd: Json<CreatePlayerCmd>,
    player_app_service: web::Data<Arc<Mutex<PlayerAppService<'_>>>>
) -> HttpResponse {

    let player = player_app_service.lock().unwrap().create(create_player_cmd.copy());

    match player {
        Ok(player) => HttpResponse::Created()
        .content_type(APPLICATION_JSON)
            .json(player),
        Err(msg) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(msg),
    }

}

/// find a player by its id `/players/{id}`
#[get("/players/{id}")]
pub async fn get(
    path: Path<(String,)>, 
    player_app_service: web::Data<Arc<Mutex<PlayerAppService<'_>>>>
) -> HttpResponse {

    let path_tuple = path.0;
    let player_id = path_tuple.0;

    let player_id = match Uuid::parse_str(&player_id) {
        Ok(player_id) => player_id,
        Err(error) => {
            return HttpResponse::BadRequest()
            .content_type(APPLICATION_JSON)
            .json(format!("Player id is not UUID {}: {}", player_id, error))
            .await
            .unwrap()
        }
    };

    let player = player_app_service.lock().unwrap().find_by_id(player_id);

    match player {
        Ok(player) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(player),
        Err(msg) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(msg),
    }
}
