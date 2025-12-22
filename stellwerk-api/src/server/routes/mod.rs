use crate::server::ServerRouter;

mod docs;
mod posts;
mod users;

pub fn routes() -> ServerRouter {
    ServerRouter::new()
        .merge(posts::routes())
        .merge(users::routes())
        .merge(docs::routes())
}
