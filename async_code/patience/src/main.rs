#![allow(dead_code)]
use std::future::Future;

use tokio::select;
fn main() {
    let runtime = tokio::runtime::Runtime::new();
    runtime.unwrap().block_on(async {
        println!("Hello, world!");
        let _x = foo1();
        let network = read_from_network();
        let terminal = read_from_terminal();
        loop {
            select! {
                stream = network.await => {
                    //do st
                }
                line = terminal.await => {
                    //do something
                }
                foo = foo2.await => {

                }
            }
        }
    });
}

async fn read_from_network() {}
async fn read_from_terminal() {}
async fn read_to_string(_: &str) {}
fn expensive_function(_: ()) {}
async fn foo1() -> usize {
    print!("Print foo 1");
    0
}

fn foo2(cancel: tokio::sync::mpsc::Receiver<()>) -> impl Future<Output = usize> {
    // This is how async before fn expand;
    async {
        print!("Print foo1");
        foo1().await;
        print!("Print foo1");
        race! {
            done = read_to_string("file1").await => {
                println!("read to string file1\r\n");
            }
            cancel = cancel.await => {
                return 0;
            }
        }
        read_to_string("filed1").await;
        println!("fooo1");
        read_to_string("file2").await;
        println!("foo1");
        let x3 = read_to_string("file3").await;
        expensive_function(x3);
        0
    }
}
