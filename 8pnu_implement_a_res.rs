use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use tokio::{prelude::*, task};

struct Monitor {
    deployments: Vec<Deployment>,
}

#[derive(Serialize, Deserialize)]
struct Deployment {
    id: String,
    service: String,
    environment: String,
    status: String,
}

async fn get_deployments() -> Vec<Deployment> {
    // implement your deployment fetch logic here
    vec![
        Deployment {
            id: "1".to_string(),
            service: "my-service".to_string(),
            environment: "prod".to_string(),
            status: " healthy".to_string(),
        },
        Deployment {
            id: "2".to_string(),
            service: "my-service".to_string(),
            environment: "stg".to_string(),
            status: " degraded".to_string(),
        },
    ]
}

async fn get_deployment_status(id: &str) -> String {
    // implement your deployment status fetch logic here
    format!("Deployment {} is healthy", id)
}

async fn monitor(req: web::ReqData<Monitor>) -> impl Responder {
    let monitor = req.into_inner();
    let deployments = get_deployments().await;
    let statuses: Vec<String> = deployments
        .into_iter()
        .map(|deployment| get_deployment_status(&deployment.id).await)
        .collect();

    format!("Statuses: {:?}\n", statuses)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let monitor = Monitor { deployments: vec![] };
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(monitor.clone()))
            .service(web::resource("/monitor").route(web::get().to(monitor)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}