use lambda_runtime::{error::HandlerError, lambda, Context};

async fn handler(
    _event: std::collections::HashMap<String, String>,
    _context: Context,
) -> Result<(), HandlerError> {
    Ok(())
}

fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    let rt = tokio::runtime::Runtime::new().unwrap();
    lambda!(move |event, context| rt.block_on(handler(event, context)));
}
