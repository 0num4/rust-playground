// use std::{
//     fmt::Display,
//     iter::Product,
//     sync::{Arc, Mutex},
// };

pub fn main() {
    // println!("q8!!!!!!!!!!!!!!!!!!");
    // let ps = ProductService::new();
    // let p = Product {
    //     id: 1,
    //     name: "tet".to_string(),
    //     price: 100,
    // };
    // ps.add_product(p);
}

// struct Product {
//     id: i32,
//     name: String,
//     price: f64,
// }

// struct InventoryItem<'a> {
//     product: &'a Product,
//     quantity: i32,
// }

// impl<'a> Display for InventoryItem<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "hello");
//         return Ok(());
//     }
//     // fn fmt
// }
// // TODO: ジェネリック型パラメータ T は、Product と InventoryItem<'a> のどちらかを受け入れます。
// struct ProductService<'a, T: Display + Clone> {
//     products: Arc<Mutex<Vec<T>>>,
// }
// impl ProductService<'a> {
//     fn new() -> ProductService<'a, T>
//     where
//         T: Product,
//     {
//         return ProductService { products: () };
//     }
//     fn add_product(&self, product: T) {
//         let mut l_p: std::sync::MutexGuard<Vec<T>> = self.products.lock().unwrap();
//         l_p.push(product)
//     }
//     fn get_product(&self, id: i32) -> Option<T> {
//         let mut l_p: std::sync::MutexGuard<Vec<T>> = self.products.lock().unwrap();
//         let p = l_p.iter().find(|x| x.id == id);
//         return p;
//     }
// }

// impl<'a> Clone for ProductService<'a, T> {
//     fn clone(&self) -> Self {
//         Self {
//             products: self.products.clone(),
//         }
//     }
// }
