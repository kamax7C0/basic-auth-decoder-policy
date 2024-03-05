mod generated;

use anyhow::{anyhow, Result};

use pdk::hl::*;
//use pdk::logger;

use http_auth_basic::Credentials;

use crate::generated::config::Config;


async fn request_filter(request_state: RequestState, _config: &Config) {
    let headers_state = request_state.into_headers_state().await;
    let auth_header = headers_state.handler().header("Authorization").unwrap_or_default();
    if auth_header.starts_with("Basic ") {
        //logger::info!("Header value: {auth_header}");
        let credentials = Credentials::from_header(auth_header).unwrap();
        headers_state.handler().set_header("client_id", &credentials.user_id);
        headers_state.handler().set_header("client_secret", &credentials.password);
    }    
}

#[entrypoint]
async fn configure(launcher: Launcher, Configuration(bytes): Configuration) -> Result<()> {
    let config: Config = serde_json::from_slice(&bytes).map_err(|err| {
        anyhow!(
            "Failed to parse configuration '{}'. Cause: {}",
            String::from_utf8_lossy(&bytes),
            err
        )
    })?;
    let filter = on_request(|rs| request_filter(rs, &config));
    launcher.launch(filter).await?;
    Ok(())
}
