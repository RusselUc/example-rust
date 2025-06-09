use colored::*;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Product {
    id: Option<i32>,
    title: String,
    price: f64,
    description: String,
    category: String,
    image: String,
}

const BASE_URL: &str = "https://fakestoreapi.com/products";

async fn get_products() -> Result<Vec<Product>, Error> {
    let response = reqwest::get(BASE_URL).await?.json::<Vec<Product>>().await?;
    Ok(response)
}

async fn get_product_id(id: i32) -> Result<Product, Error> {
    let url_with_id = format!("{}/{}", BASE_URL, id);
    let response = reqwest::get(url_with_id).await?.json::<Product>().await?;
    Ok(response)
}

async fn create_product() -> Result<Product, Error> {
    let client = reqwest::Client::new();

    println!("Name of the product:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!("Price of the product:");
    let mut price = String::new();
    io::stdin()
        .read_line(&mut price)
        .expect("Failed to read line");

    println!("Description of the product:");
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");

    println!("Category of the product:");
    let mut category = String::new();
    io::stdin()
        .read_line(&mut category)
        .expect("Failed to read line");
    let image = String::from("https://example.com/image.jpg");

    let product = Product {
        id: None,
        title: name.trim().to_string(),
        price: price.trim().parse().unwrap_or(0.0),
        description: description.trim().to_string(),
        category: category.trim().to_string(),
        image,
    };

    let response = client
        .post(BASE_URL)
        .json(&product)
        .send()
        .await?
        .json::<Product>()
        .await?;
    Ok(response)
}

async fn edit_product_id(id: i32) -> Result<Product, Error> {
    let client = reqwest::Client::new();
    let url_with_id = format!("{}/{}", BASE_URL, id);

    println!("Name of the product:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!("Price of the product:");
    let mut price = String::new();
    io::stdin()
        .read_line(&mut price)
        .expect("Failed to read line");

    println!("Description of the product:");
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");

    println!("Category of the product:");
    let mut category = String::new();
    io::stdin()
        .read_line(&mut category)
        .expect("Failed to read line");

    let image = String::from("https://example.com/image.jpg");

    let product = Product {
        id: Some(id),
        title: name.trim().to_string(),
        price: price.trim().parse().unwrap_or(0.0),
        description: description.trim().to_string(),
        category: category.trim().to_string(),
        image,
    };

    let response = client
        .put(url_with_id)
        .json(&product)
        .send()
        .await?
        .json::<Product>()
        .await?;

    Ok(response)
}

async fn delete_product(id: i32) -> Result<(), Error> {
    let client = reqwest::Client::new();
    client
        .delete(&format!("{}/{}", BASE_URL, id))
        .send()
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let mut is_exit = false;

    while !is_exit {
        // std::process::Command::new("clear").status().unwrap();
        println!("This is a simple REST API client for a fake store API.");
        println!("Select an operation:");
        println!("1. Get Products");
        println!("2. Get Product by ID");
        println!("3. Create Product");
        println!("4. Delete Product");
        println!("5. Edit Product by ID");
        println!("0. Exit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let choice: u32 = input.trim().parse().expect("Please enter a number");
        match choice {
            0 => {
                println!("Exiting...");
                is_exit = true;
                return;
            }
            1 => match get_products().await {
                Ok(products) => {
                    for product in products {
                        println!("----------------------------------");
                        println!("ID: {}", product.id.unwrap_or(-1));
                        println!("{} {}", "Title:".blue(), product.title);
                        println!("Price: ${:.2}", product.price);
                        println!("Category: {}", product.category);
                        println!("Description: {}", product.description);
                        println!("Image URL: {}", product.image);
                    }
                    println!("----------------------------------");
                }
                Err(e) => eprintln!("Error fetching products: {}", e),
            },
            2 => {
                println!("Enter product ID:");
                let mut id_input = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Failed to read line");
                let id: i32 = id_input.trim().parse().expect("Please enter a valid ID");
                match get_product_id(id).await {
                    Ok(product) => {
                        println!("----------------------------------");
                        println!("ID: {}", product.id.unwrap_or(-1));
                        println!("{} {}", "Title:".blue(), product.title);
                        println!("Price: ${:.2}", product.price);
                        println!("Category: {}", product.category);
                        println!("Description: {}", product.description);
                        println!("Image URL: {}", product.image);
                        println!("----------------------------------");
                    }
                    Err(e) => eprintln!("Error fetching product: {}", e),
                }
            }
            3 => match create_product().await {
                Ok(product) => println!("Created Product: {:?}", product),
                Err(e) => eprintln!("Error creating product: {}", e),
            },
            4 => {
                println!("Enter product ID to delete:");
                let mut id_input = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Failed to read line");
                let id: i32 = id_input.trim().parse().expect("Please enter a valid ID");
                match delete_product(id).await {
                    Ok(_) => println!("Product deleted successfully"),
                    Err(e) => eprintln!("Error deleting product: {}", e),
                }
            }
            5 => {
                println!("Enter product ID to edit:");
                let mut id_input = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Failed to read line");
                let id: i32 = id_input.trim().parse().expect("Please enter a valid ID");
                match edit_product_id(id).await {
                    Ok(product) => println!("Edited Product: {:?}", product),
                    Err(e) => eprintln!("Error editing product: {}", e),
                }
            }
            _ => println!("Invalid choice, please select 1, 2, or 3."),
        }

        input.clear();
        println!();
        println!();
        println!();
    }
}
