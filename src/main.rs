use crate::middleware::handle_404::handle_404;
use crate::routers::router;
use config::CFG;
use salvo::catcher::Catcher;
use salvo::prelude::*;
use salvo::server::ServerHandle;
use tokio::signal;
use tracing::info;
mod app_error;
mod utils;
#[allow(dead_code)]
mod app_response;
mod config;
mod middleware;
mod routers;

fn get_address() -> String{
    let args: Vec<String> = std::env::args().collect();
    let port = args
        .iter()
        .position(|a| a == "--port")
        .map_or(&CFG.server.port, |i| &args[i + 1]);

    format!("{}:{port}", &CFG.server.address)
}

#[tokio::main]
async fn main() {
    let address = get_address();

    //At the same time, logs are only output to the terminal or file
    let _guard = clia_tracing_config::build()
        .filter_level(&CFG.log.filter_level)
        .with_ansi(CFG.log.with_ansi)
        .to_stdout(CFG.log.to_stdout)
        .directory(&CFG.log.directory)
        .file_name(&CFG.log.file_name)
        .rolling(&CFG.log.rolling)
        .init();
    tracing::info!("log level: {}", &CFG.log.filter_level);

    let router = router();
    let service: Service = router.into();
    let service = service.catcher(Catcher::default().hoop(handle_404));
    println!("🌪️ {} is starting ", &CFG.server.name);
    println!("🔄 listen on {}", &address);

    println!(
        "📖 Open API Page: http://{}/swagger-ui",
        &address.replace("0.0.0.0", "127.0.0.1")
    );
    let acceptor = TcpListener::new(&address).bind().await;
    let server = Server::new(acceptor);
    let handle = server.handle();
    tokio::spawn(shutdown_signal(handle));
    server.serve(service).await;
}

async fn shutdown_signal(handle: ServerHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("ctrl_c signal received"),
        _ = terminate => info!("terminate signal received"),
    }
    handle.stop_graceful(std::time::Duration::from_secs(60));
}

#[cfg(test)]
mod tests {
    use salvo::prelude::*;
    use salvo::test::{ResponseExt, TestClient};

    use crate::get_address;

    #[tokio::test]
    async fn test_hello_world() {
        let service = Service::new(super::router());

        let content = TestClient::get(format!(
            "http://{}",
            &get_address().replace("0.0.0.0", "127.0.0.1")
        ))
        .send(&service)
        .await
        .take_string()
        .await
        .unwrap();
        assert_eq!(content, "Hello World from salvo");
    }
}
