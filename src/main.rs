#[macro_use]
extern crate actix_web;

use std::sync::{Mutex, Arc};
use std::{env, io};

use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use chessforyou::application::game_app_service::{GameAppService, self};
use chessforyou::application::player_app_service::PlayerAppService;
use chessforyou::controller;
use chessforyou::infrastructure::game_repository::InMemoryGameRepository;
use chessforyou::infrastructure::player_repository::InMemoryPlayerRepository;
use chessforyou::model::PlayerRepository;
use chessforyou::model::allowed_movement::AllPiecesAllowedMoveCalculator;
use chessforyou::model::check_verifier::create_check_verifier;
use chessforyou::model::movement::MovementExecutor;

struct PlayerRepositoryHolder {
    pub player_repository: Option<Box<dyn PlayerRepository>>,
}

static mut PLAYER_REPOSITORY: PlayerRepositoryHolder = PlayerRepositoryHolder {
    player_repository: None,
};

unsafe fn initialize_player_repository() {
    PLAYER_REPOSITORY.player_repository = Some(Box::new(InMemoryPlayerRepository::new()) as Box<dyn PlayerRepository>);
}

#[actix_rt::main]
async fn main() -> io::Result<()> {

    let app_services = unsafe {

        initialize_player_repository();
    
        let game_app_service = GameAppService::new(
            Box::new(InMemoryGameRepository::new()),
            PLAYER_REPOSITORY.player_repository.as_ref().unwrap(),
            AllPiecesAllowedMoveCalculator::new(),
            MovementExecutor::new(),
            create_check_verifier(),
        );

        let game_app_service = Arc::new(Mutex::new(game_app_service));

        let player_app_service = PlayerAppService::new(PLAYER_REPOSITORY.player_repository.as_mut().unwrap());

        let player_app_service = Arc::new(Mutex::new(player_app_service));

        (game_app_service, player_app_service)
    };

    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .data(app_services.0.clone())
            .data(app_services.1.clone())
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(controller::game_controller::create)
            .service(controller::game_controller::get)
            .service(controller::game_controller::move_piece)
            .service(controller::player_controller::create)
            .service(controller::player_controller::get)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
