// Example from:
// https://rust-lang.github.io/async-book/02_execution/04_executor.html

use std::{
    future::Future,
    pin::Pin,
    sync::mpsc::{Receiver, SyncSender, sync_channel},
    sync::{Arc, Mutex},
    task::{Context, Poll},
};
use futures::{
    future::BoxFuture,
    FutureExt,
    task::{ArcWake, waker_ref},
};

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(async {
        println!("hello!");
        println!("{}", HelloFuture.await);
    });

    // signals to the executor that no omre tasks will be incoming
    drop(spawner);

    executor.run();
}

// A future that just says hello
struct HelloFuture;

impl Future for HelloFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        Poll::Ready("Hello from the future".to_string())
    }
}

/// Task executor that receives tasks off of a channel and runs them.
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);

                // Note: as_mut comes from Pin
                if let Poll::Pending = future.as_mut().poll(context) {
                    // We're not done, put the future back
                    *future_slot = Some(future);
                }
            }
        }
    }
}

/// Spawns new futures onto the task channel
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

/// A future that can reschedule itself to be polled by an Executor
struct Task {
    /// In-progress future that should be pushed to completion.
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// Handle to place the task itself back onto the task queue.
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // Sends this task back onto the task channel so it will be polled
        // again.
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("too many tasks queued");
    }
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}
