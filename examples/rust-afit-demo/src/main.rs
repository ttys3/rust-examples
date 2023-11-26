//! Run with
//!
//! ```not_rust
//! cargo run -p example-rust-afit-demo
//! ```
//!
//! This example shows how to run axum using hyper's low level API.
//!
//! The [hyper-util] crate exists to provide high level utilities but its still in early stages of
//! development.
//!
//! [hyper-util]: https://crates.io/crates/hyper-util
use std::fmt::{Debug, Display};
use std::future::Future;

#[tokio::main]
async fn main() {
    // async_fn_in_trait https://rust-lang.github.io/rfcs/3185-static-async-fn-in-trait.html
    let a = ServiceAFIT;
    call(a, 1).await;

    // uses return_position_impl_trait_in_trait https://rust-lang.github.io/rfcs/3425-return-position-impl-trait-in-traits.html
    let b = ServiceRPITIT;
    call(b, 2).await;
}

async fn call<S: Service>(s : S, key: i32) where <S as Service>::Response: Debug + Display {
    let result = s.request(key).await;
    println!("{}", result);
}

trait Service {
    type Response;
    async fn request(&self, key: i32) -> Self::Response;
}

trait ServiceClassic {
    type Response;
    fn request(&self, key: i32) -> impl Future<Output = Self::Response>;
}

struct ServiceAFIT;
impl Service for ServiceAFIT {
    type Response = String;
    async fn request(&self, key: i32) -> Self::Response {
        format!("ServiceA: {}", key)
    }
}

struct ServiceRPITIT;
impl Service for ServiceRPITIT {
    type Response = i32;

    fn request(&self, key: i32) -> impl Future<Output = Self::Response> {
        async move {
            key * key
        }
    }
}