use super::{handlers::*, state::DappState};
use actix_web::{App, HttpServer};

pub async fn start(
    config_path: String,
    network: Option<String>,
    private_key_path: String,
    listen_url: String,
) -> std::io::Result<()> {
    let dapp_state =
        DappState::new(config_path, network, private_key_path).expect("init dapp server error");
    let local = tokio::task::LocalSet::new();
    let sys = actix_web::rt::System::run_in_tokio("server", &local);
    let _server_res = HttpServer::new(move || {
        let cors = actix_cors::Cors::permissive();
        App::new()
            .wrap(cors)
            .data(dapp_state.clone())
            .service(index)
            .service(settings)
            .service(get_or_create_bridge_cell)
            .service(burn)
            .service(lock)
            .service(get_best_block_height)
            .service(get_sudt_balance)
    })
    .bind(&listen_url)?
    .run()
    .await?;
    sys.await?;
    Ok(())
}
