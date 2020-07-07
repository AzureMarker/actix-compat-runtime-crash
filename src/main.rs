use actix_web::{web, App, HttpServer};

fn main() {
    // Start actix with the tokio-compat runtime, which supports tokio 0.1 and 0.2
    let mut runtime = tokio_compat::runtime::Builder::new()
        .core_threads(1)
        .build()
        .unwrap();
    let local_tasks = tokio02::task::LocalSet::new();
    let sys = actix_rt::System::run_in_tokio("main", &local_tasks);
    local_tasks.spawn_local(sys);
    runtime.block_on_std(local_tasks.run_until(run()))
}

async fn run() {
    // Spawning with tokio 0.1 works on the main thread
    tokio01::spawn(futures01::lazy(|| {
        println!("Hello from tokio 0.1 + futures 0.1");
        futures01::future::ok(())
    }));

    // Spawning with tokio 0.2 works of course
    tokio02::spawn(async { println!("Hello from tokio 0.2") })
        .await
        .unwrap();

    HttpServer::new(|| App::new().route("/", web::get().to(handler)))
        .bind("127.0.0.1:8000")
        .unwrap()
        .run()
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    tokio01::spawn(futures01::lazy(|| {
        println!("This future will fail to spawn");
        futures01::future::ok(())
    }));

    "Spawned tokio 0.1 future"
}
