use anyhow::Result;
use ops::register;
use ops::{FibAt1, FibAt2, Mul, Sum};
use paladin::directive::{indexed_stream::IndexedStream, Directive};
use paladin::runtime::Runtime;

#[tokio::main]
async fn main() -> Result<()> {
    let mut paladin_args1 = paladin::config::Config::default();
    paladin_args1.task_bus_routing_key = Some("worker1".to_string());
    paladin_args1.amqp_uri = Some("amqp://localhost:5672".to_string());
    println!("paladin_args1: {:?}", paladin_args1);
    let mut paladin_args2 = paladin::config::Config::default();
    paladin_args2.task_bus_routing_key = Some("worker2".to_string());
    paladin_args2.amqp_uri = Some("amqp://localhost:5672".to_string());
    println!("paladin_args1: {:?}", paladin_args1);
    let runtime1 = Runtime::from_config(&paladin_args1, register()).await?;
    let runtime2 = Runtime::from_config(&paladin_args2, register()).await?;
    println!("runtime1:");
    println!("runtime2:");

    let stream = IndexedStream::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    // Compute the fibonacci number at each element in the stream with our
    // previously declared `FibAt1` operation.
    println!("stream");
    let fibs = stream.map(&FibAt1);
    // Sum the fibonacci numbers.
    println!("fibs");
    let sum = fibs.fold(&Sum);
    println!("sum");

    // Run the computation.
    let result = sum.run(&runtime1).await;

    println!("Result1: {:?}", result);

    // Close the runtime1
    runtime1.close().await?;

    let stream = IndexedStream::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    // Compute the fibonacci number at each element in the stream with our
    // previously declared `FibAt2` operation.
    let fibs = stream.map(&FibAt2);
    // Sum the fibonacci numbers.
    let mul = fibs.fold(&Mul);

    // Run the computation.
    let result = mul.run(&runtime2).await;

    println!("Result2: {:?}", result);

    // Close the runtime2
    runtime2.close().await?;

    Ok(())
}
