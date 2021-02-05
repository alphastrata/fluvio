mod public_server;
mod spg;
mod spu;
mod topic;
mod partition;
mod api_version;
mod create;
mod delete;
mod list;
mod watch;

pub use server::start_public_server;

mod server {

    use std::fmt::Debug;

    use tracing::info;

    use fluvio_future::net::TcpStream;
    use fluvio_service::KfApiServer;
    use fluvio_auth::Authorization;

    use crate::services::auth::AuthGlobalContext;
    use super::public_server::PublicService;

    /// create public server
    pub fn start_public_server<A>(ctx: AuthGlobalContext<A>)
    where
        A: Authorization<Stream = TcpStream> + Sync + Send + Debug + 'static,
        AuthGlobalContext<A>: Clone + Debug,
        <A as Authorization>::Context: Send + Sync,
    {
        let addr = ctx.global_ctx.config().public_endpoint.clone();
        info!("start public api service");
        let server = KfApiServer::new(addr, ctx, PublicService::new());
        server.run();
    }
}
