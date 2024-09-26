#![allow(dead_code)]
use std::future::Future;

use tokio::select;
fn main() {
    let runtime = tokio::runtime::Runtime::new();
    runtime.unwrap().block_on(async {
        println!("Hello, world!");
        let _x = foo();
        let network = read_from_network();
        let terminal = read_from_terminal();
        loop {
            select! {
                stream <- network.await => {
                    //do st
                }
                line <- terminal.await => {

                }
            }
        }
    });
}

async fn read_from_network() {}
async fn read_from_terminal() {}
async fn foo1() -> usize {
    print!("Print foo 1");
    0
}

fn foo() -> impl Future<Output = usize> {
    async {
        print!("Print foo");
        foo1().await;
        print!("Print foo2");
        0
    }
}
