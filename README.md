# 🛒 Fake Store API Client (Rust)

This is a command-line client written in **Rust** that interacts with the [Fake Store API](https://fakestoreapi.com/). It allows you to list, view, create, update, and delete products via HTTP requests (`GET`, `POST`, `PUT`, and `DELETE`).

---

## 🚀 Features

- 📦 Fetch all products  
- 🔍 Get product by ID  
- ➕ Create a new product  
- ✏️ Edit an existing product  
- ❌ Delete a product  
- 🎨 Colored terminal output using `colored`

---

## 🧰 Technologies and Dependencies

- [Rust](https://www.rust-lang.org/)
- [`reqwest`](https://docs.rs/reqwest) - Async HTTP requests
- [`tokio`](https://tokio.rs/) - Async runtime
- [`serde`](https://serde.rs/) - Serialization/Deserialization
- [`colored`](https://docs.rs/colored) - Colored terminal output

---

## 📦 Installation

1. Clone the repository:

```bash
git clone https://github.com/your_username/fake-store-client-rust.git
cd fake-store-client-rust
```

2. Make sure you have Rust and Cargo installed:

```bash
rustup --version
cargo --version
```

3. Run the application:

```bash
cargo run
```

---

## 🧪 Usage

Once the app is running, you’ll see a menu like this:

```
This is a simple REST API client for a fake store API.
Select an operation:
1. Get Products
2. Get Product by ID
3. Create Product
4. Delete Product
5. Edit Product by ID
0. Exit
```

Enter the number of the action you want to perform and follow the on-screen prompts.

---

## 📁 Code Overview

- `Product`: Struct representing a product.
- `get_products()`: Fetches all products.
- `get_product_id(id)`: Fetches a single product by ID.
- `create_product()`: Creates a new product using user input.
- `edit_product_id(id)`: Updates a product by ID.
- `delete_product(id)`: Deletes a product by ID.
- `main()`: Entry point with interactive menu and control logic.

---

## 📝 Notes

- All requests are made to `https://fakestoreapi.com/products`.
- When creating or editing a product, the image URL is set to a default:  
  `https://example.com/image.jpg`.

---

## 📦 Example Product JSON

```json
{
  "title": "Rustacean T-shirt",
  "price": 19.99,
  "description": "100% cotton, safe for memory and threads.",
  "category": "clothing",
  "image": "https://example.com/image.jpg"
}
```

---
