#[cfg(feature = "actix")]
mod inner {
    use actix_web::middleware::Logger;
    use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Pet {
        pub id: u32,
        pub name: String,
        pub tag: String,
    }

    pub fn pets() -> Vec<Pet> {
        vec![
            Pet {
                id: 1,
                name: "Rust".to_string(),
                tag: "Rust".to_string(),
            },
            Pet {
                id: 2,
                name: "Python".to_string(),
                tag: "Python".to_string(),
            },
            Pet {
                id: 3,
                name: "Typescript".to_string(),
                tag: "Typescript".to_string(),
            },
        ]
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListPetsParams {
        pub limit: Option<u32>,
    }

    #[get("/pets")]
    async fn list_pets(query: web::Query<ListPetsParams>) -> impl Responder {
        println!("{:?}", query);
        HttpResponse::Ok().json(pets())
    }

    #[post("/pets")]
    async fn create_pet(_req_body: String) -> impl Responder {
        HttpResponse::Created()
    }

    #[get("/pets/{id}")]
    async fn get_pet_by_id(path: web::Path<(u32,)>) -> impl Responder {
        let pet = pets().into_iter().find(|p| p.id == path.0);
        HttpResponse::Ok().json(pet)
    }

    #[actix_web::main]
    async fn main() -> std::io::Result<()> {
        let port = 5000;
        env_logger::init();
        println!("Listening on http://localhost:{port}");
        HttpServer::new(|| {
            App::new()
                .wrap(Logger::default())
                .service(list_pets)
                .service(create_pet)
                .service(get_pet_by_id)
        })
            .bind(("127.0.0.1", port))?
            .run()
            .await
    }
}

fn main() {
    #[cfg(feature = "actix")]
    inner::main().unwrap();
}
