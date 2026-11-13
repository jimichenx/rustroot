// ANCHOR: full
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::sync::{mpsc, watch};

// ANCHOR: app_state_and_timeout
struct AppState {
    service_name: String,
    timeout_ms: u64,
    completed_jobs: AtomicUsize,
}

async fn process_job(state: Arc<AppState>, job_id: u64, work_ms: u64) {
    let work = async {
        tokio::time::sleep(Duration::from_millis(work_ms)).await;
        let done = state.completed_jobs.fetch_add(1, Ordering::Relaxed) + 1;
        println!(
            "[{}] job #{job_id} committed (total={done})",
            state.service_name
        );
    };

    if tokio::time::timeout(Duration::from_millis(state.timeout_ms), work)
        .await
        .is_err()
    {
        println!("[{}] job #{job_id} timed out and dropped", state.service_name);
    }
}
// ANCHOR_END: app_state_and_timeout

// ANCHOR: select_and_shutdown
async fn run_worker(
    state: Arc<AppState>,
    mut job_rx: mpsc::Receiver<(u64, u64)>,
    mut shutdown_rx: watch::Receiver<bool>,
) {
    let mut inflight = Vec::new();

    loop {
        tokio::select! {
            maybe_job = job_rx.recv() => {
                let Some((job_id, work_ms)) = maybe_job else { break };
                let st = Arc::clone(&state);
                inflight.push(tokio::spawn(async move {
                    process_job(st, job_id, work_ms).await;
                }));
            }
            _ = shutdown_rx.changed() => {
                println!("shutdown signal received: stop accepting new jobs");
                break;
            }
        }
    }

    for handle in inflight {
        let _ = handle.await;
    }
    println!(
        "worker drained cleanly, completed={}",
        state.completed_jobs.load(Ordering::Relaxed)
    );
}
// ANCHOR_END: select_and_shutdown

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let state = Arc::new(AppState {
        service_name: String::from("billing-svc"),
        timeout_ms: 50,
        completed_jobs: AtomicUsize::new(0),
    });

    let (job_tx, job_rx) = mpsc::channel(8);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let worker = tokio::spawn(run_worker(Arc::clone(&state), job_rx, shutdown_rx));

    job_tx.send((1, 20)).await.unwrap();
    job_tx.send((2, 90)).await.unwrap();
    tokio::time::sleep(Duration::from_millis(5)).await;

    shutdown_tx.send(true).unwrap();
    worker.await.unwrap();
}
// ANCHOR_END: full
