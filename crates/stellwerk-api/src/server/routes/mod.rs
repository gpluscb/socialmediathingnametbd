use crate::server::ServerRouter;

mod docs;
mod oauth2;
mod posts;
mod users;

pub fn routes() -> ServerRouter {
    ServerRouter::new()
        .merge(posts::routes())
        .merge(users::routes())
        .merge(oauth2::routes())
        .merge(docs::routes())
}
