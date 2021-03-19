use lambda_runtime::{handler_fn, Context};
use rotate_icon::run;
use serde_json::Value;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;
#[tokio::main]
async fn main() -> Result<(), Error> {
    openssl_probe::init_ssl_cert_env_vars();
    lambda_runtime::run(handler_fn(handler)).await?;
    Ok(())
}

async fn handler(_: Value, _: Context) -> Result<Value, Error> {
    Ok(run().await?)
}
