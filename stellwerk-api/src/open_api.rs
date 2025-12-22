use aide::openapi::{Components, OpenApi, ReferenceOr, SecurityScheme};
use tracing::error;

pub const BEARER_SECURITY_SCHEME: &str = "bearer";

pub fn install_open_api() -> OpenApi {
    aide::generate::on_error(|error| {
        error!(%error, "Error generating aide documentation");
    });

    // Using IndexMap::default instead of Default::default
    // would require adding another crate to Cargo.toml
    #[allow(clippy::default_trait_access)]
    let security_schemes = [(
        BEARER_SECURITY_SCHEME.into(),
        ReferenceOr::Item(SecurityScheme::Http {
            scheme: "bearer".into(),
            bearer_format: None,
            description: None,
            extensions: Default::default(),
        }),
    )]
    .into();

    OpenApi {
        components: Some(Components {
            security_schemes,
            ..Default::default()
        }),
        ..Default::default()
    }
}
