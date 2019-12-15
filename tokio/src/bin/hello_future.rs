use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct HelloWorld;

impl Future for HelloWorld {
    type Output = String;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready("Hello world".to_string())
    }
}

struct Display<T>(T);

impl<T> Future for Display<T>
where
    T: Future + Unpin,
    T::Output: std::fmt::Display,
{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let inner = Pin::new(&mut self.0);
        let value = match inner.poll(cx) {
            Poll::Ready(value) => value,
            Poll::Pending => return Poll::Pending,
        };

        println!("Display: {}", value);
        Poll::Ready(())
    }
}

#[tokio::main]
async fn main() {
    let future = Display(HelloWorld);
    future.await
}
