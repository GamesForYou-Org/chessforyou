
use std::sync::{Mutex, Arc};
use actix_web::{post, get, put, web};
use actix_web::web::{Json, Path};
use actix_web::HttpResponse;
use uuid::Uuid;
use crate::controller::APPLICATION_JSON;
use crate::application::game_app_service::{GameAppService, MoveCmd, CreateGameCmd};

/// create a game `/games`
#[post("/games")]
pub async fn create(
    create_game_cmd: Json<CreateGameCmd>,
    game_app_service: web::Data<Arc<Mutex<GameAppService<'_>>>>
) -> HttpResponse {

    let game = game_app_service.lock().unwrap().create(create_game_cmd.copy());

    match game {
        Ok(game) => HttpResponse::Created()
        .content_type(APPLICATION_JSON)
            .json(game),
        Err(msg) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(msg),
    }

}

/// find a game by its id `/games/{id}`
#[get("/games/{id}")]
pub async fn get(
    path: Path<(String,)>, 
    game_app_service: web::Data<Arc<Mutex<GameAppService<'_>>>>
) -> HttpResponse {

    let game_id = &path.0;

    let game_id = match Uuid::parse_str(game_id) {
        Ok(game_id) => game_id,
        Err(error) => {
            return HttpResponse::BadRequest()
            .content_type(APPLICATION_JSON)
            .json(format!("Game id is not UUID {}: {}", game_id, error))
        }
    };

    let game = game_app_service.lock().unwrap().get(game_id);

    match game {
        Ok(game) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(game),
        Err(msg) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(msg),
    }
}

// move a piece in a game `/games/{id}`
#[put("/games/{id}")]
pub async fn move_piece(
    _path: Path<(String,)>,
    move_cmd: Json<MoveCmd>,
    game_app_service: web::Data<Arc<Mutex<GameAppService<'_>>>>
) -> HttpResponse {

    let game = game_app_service.lock().unwrap().move_piece(move_cmd.copy());

    match game {
        Ok(game) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(game),
        Err(msg) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(msg),
    }
}
