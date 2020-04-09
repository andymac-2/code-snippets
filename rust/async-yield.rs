use futures::executor::block_on;
use futures::future::join;
use futures::stream::StreamExt;
use futures::channel::mpsc::{channel, Sender, Receiver};

struct Yielder<S, R>(Sender<S>, Receiver<R>);
impl<S, R> Yielder<S, R> {
    async fn yield_me (&mut self, value: S) -> R {
        self.0.try_send(value).unwrap();
        self.1.next().await.unwrap()
    }
}

// async fn first (mut tx: Sender<()>, mut rx: Receiver<()>) {
//     println!("First Line One");
    
//     tx.try_send(()).unwrap();
//     rx.next().await.unwrap();
    
//     println!("First Line Two");
    
//     tx.try_send(()).unwrap();
//     rx.next().await.unwrap();
// }

// async fn second (mut tx: Sender<()>, mut rx: Receiver<()>) {
//     tx.try_send(()).unwrap();
//     rx.next().await.unwrap();
    
//     println!("Second Line One");
    
//     tx.try_send(()).unwrap();
//     rx.next().await.unwrap();
    
//     println!("Second Line Two");
// }

async fn first (mut yielder: Yielder<&str, ()>) {
    println!("Sending 'one'");
    yielder.yield_me("one").await;
    
    println!("Sending 'two'");
    yielder.yield_me("two").await;
    
    println!("Sending 'three'");
    yielder.yield_me("three").await;
    
    println!("Sending 'four'");
    yielder.yield_me("four").await;
    
    println!("Sending 'five'");
    yielder.yield_me("five").await;
    
    println!("Sending 'six'");
    yielder.yield_me("six").await;
    
    println!("Sending 'seven'");
    yielder.yield_me("seven").await;
    
    println!("Sending 'eight'");
    yielder.yield_me("eight").await;
}

async fn second (mut yielder: Yielder<(), &str>) {
    let result = yielder.yield_me(()).await;
    println!("Received: {}", result);
    
    let result = yielder.yield_me(()).await;
    println!("Received: {}", result);
    
    let result = yielder.yield_me(()).await;
    println!("Received: {}", result);
    
    let result = yielder.yield_me(()).await;
    println!("Received: {}", result);
    
    let result = yielder.yield_me(()).await;
    println!("Received: {}", result);
    
    let result = yielder.yield_me(()).await;
    println!("Received: {}", result);
    
    let result = yielder.yield_me(()).await;
    println!("Received: {}", result);
    
    let result = yielder.yield_me(()).await;
    println!("Received: {}", result);
}

fn main() {
    let (tx1, rx1) = channel(10);
    let (tx2, rx2) = channel(10);
    
    let y1 = Yielder(tx1, rx2);
    let y2 = Yielder(tx2, rx1);
    
    let f = first(y1);
    let s = second(y2);
    
    let pair = join(f, s);
    block_on(pair);
}