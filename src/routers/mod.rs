use salvo::{
    prelude::{CatchPanic, Logger, OpenApi, SwaggerUi},
    Router,
};
use crate::middleware::cors::cors_middleware;
use self::demo::hello;
pub mod demo;
mod static_routers;

pub fn router() -> Router {
    let _cors_handler = cors_middleware();
    let mut static_routers = static_routers::create_static_routers();
    let router = Router::new()
        //.hoop(_cors_handler)
        .hoop(Logger::new())
        .hoop(CatchPanic::new())
        .get(hello)
        .append(&mut static_routers);
    let doc = OpenApi::new("salvo web api", "0.0.1").merge_router(&router);
    router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"))
}
