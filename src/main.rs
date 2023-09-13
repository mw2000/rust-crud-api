use actix_web::{get, post, delete, put, web, App, HttpResponse, HttpServer, Error as ActixError};
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;
use tokio::spawn;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct User {
    id: i32,
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
pub struct InsertableUser {
    name: String,
    email: String,
}

#[post("/users")]
async fn create_user(user: web::Json<InsertableUser>) -> Result<HttpResponse, ActixError> {
    // Define the connection string
    let connection_string = "host=postgres port=5432 user=myuser password=mysecretpassword dbname=mydb";
    
    // Database connection logic...
    let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await.expect("Connection error");

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Insert the user into the database
    let insert_stmt = "INSERT INTO users (name, email) VALUES ($1, $2);";
    client.execute(insert_stmt, &[&user.name, &user.email]).await.expect("Query Error");
    // Return HttpResponse
    Ok(HttpResponse::Created().json(user.into_inner()))
}

#[get("/users")]
async fn get_users() -> Result<HttpResponse, ActixError> {
    // Define the connection string
    let connection_string = "host=postgres port=5432 user=myuser password=mysecretpassword dbname=mydb";
    
    // Database connection logic...
    let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await.expect("Connection Error");

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let result = client.query("SELECT id, name, email FROM users;", &[]).await.expect("Query Error");
    
    // Convert rows into Vec<User>
    let mut users = Vec::new();
    for row in result {
        let user = User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
        };
        users.push(user);
    }

    // Return HttpResponse
    Ok(HttpResponse::Ok().json(users))
}

#[put("/users/{id}")]
async fn update_user(id: web::Path<i32>, user: web::Json<InsertableUser>) -> Result<HttpResponse, ActixError> {
    // Define the connection string
    let connection_string = "host=postgres port=5432 user=myuser password=mysecretpassword dbname=mydb";
    
    let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await.expect("Connection Error");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let update_stmt = "UPDATE users SET name=$1, email=$2 WHERE id=$3;";
    client.execute(update_stmt, &[&user.name, &user.email,  &*id]).await.expect("Query Error");

    Ok(HttpResponse::Ok().json(user.into_inner()))
}

#[delete("/users/{id}")]
async fn delete_user(id: web::Path<i32>) -> Result<HttpResponse, ActixError> {
    // Define the connection string
    let connection_string = "host=postgres port=5432 user=myuser password=mysecretpassword dbname=mydb";

    let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await.expect("Connection Error");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let delete_stmt = "DELETE FROM users WHERE id=$1;";
    client.execute(delete_stmt, &[&*id]).await.expect("Query Error");

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create_user)
            .service(get_users)
            .service(update_user)
            .service(delete_user)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

