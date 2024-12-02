use crate::ProtocolError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::mpsc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub timestamp: u64,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trace {
    pub trace_id: String,
    pub span_id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub start_time: u64,
    pub duration: u64,
    pub attributes: HashMap<String, String>,
}

pub struct Telemetry {
    metrics_tx: mpsc::Sender<Metric>,
    traces_tx: mpsc::Sender<Trace>,
    request_counter: AtomicU64,
    error_counter: AtomicU64,
}

impl Telemetry {
    pub fn new(
        metrics_capacity: usize,
        traces_capacity: usize,
    ) -> (Self, mpsc::Receiver<Metric>, mpsc::Receiver<Trace>) {
        let (metrics_tx, metrics_rx) = mpsc::channel(metrics_capacity);
        let (traces_tx, traces_rx) = mpsc::channel(traces_capacity);

        (
            Self {
                metrics_tx,
                traces_tx,
                request_counter: AtomicU64::new(0),
                error_counter: AtomicU64::new(0),
            },
            metrics_rx,
            traces_rx,
        )
    }

    pub async fn record_metric(&self, metric: Metric) -> Result<(), ProtocolError> {
        self.metrics_tx
            .send(metric)
            .await
            .map_err(|e| ProtocolError::InvalidFormat(e.to_string()))
    }

    pub async fn record_trace(&self, trace: Trace) -> Result<(), ProtocolError> {
        self.traces_tx
            .send(trace)
            .await
            .map_err(|e| ProtocolError::InvalidFormat(e.to_string()))
    }

    pub fn increment_requests(&self) {
        self.request_counter.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_errors(&self) {
        self.error_counter.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_request_count(&self) -> u64 {
        self.request_counter.load(Ordering::Relaxed)
    }

    pub fn get_error_count(&self) -> u64 {
        self.error_counter.load(Ordering::Relaxed)
    }
} 