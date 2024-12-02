use crate::{Message, MessageFlags, MessageType, ProtocolError};
use bytes::Bytes;
use futures::{Stream, StreamExt};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc;

pub struct MessageStream {
    rx: mpsc::Receiver<Message>,
    stream_id: u32,
    closed: bool,
}

impl MessageStream {
    pub fn new(capacity: usize) -> (mpsc::Sender<Message>, Self) {
        let (tx, rx) = mpsc::channel(capacity);
        let stream_id = rand::random();
        (tx, Self {
            rx,
            stream_id,
            closed: false,
        })
    }

    pub fn stream_id(&self) -> u32 {
        self.stream_id
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }
}

impl Stream for MessageStream {
    type Item = Result<Message, ProtocolError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.closed {
            return Poll::Ready(None);
        }

        match self.rx.poll_recv(cx) {
            Poll::Ready(Some(msg)) => {
                if msg.msg_type == MessageType::StreamEnd {
                    self.closed = true;
                }
                Poll::Ready(Some(Ok(msg)))
            }
            Poll::Ready(None) => {
                self.closed = true;
                Poll::Ready(None)
            }
            Poll::Pending => Poll::Pending,
        }
    }
} 