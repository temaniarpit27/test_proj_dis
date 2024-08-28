use anyhow::Result;
use ops::register;
use paladin::runtime::WorkerRuntime;

#[tokio::main]
async fn main() -> Result<()> {
    let mut paladin_args = paladin::config::Config::default();
    paladin_args.task_bus_routing_key = Some("worker2".to_string());
    paladin_args.amqp_uri = Some("amqp://localhost:5672".to_string());
    let runtime = WorkerRuntime::from_config(&paladin_args, register()).await?;
    runtime.main_loop().await?;

    Ok(())
}
