use hyper::body::Bytes;
use serde::Serialize;
use tokio::sync::mpsc;

/// Error returned when the streaming channel is closed.
#[derive(Debug)]
pub struct SendError;

impl std::fmt::Display for SendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "stream channel closed")
    }
}

impl std::error::Error for SendError {}

/// A typed sender for streaming RPC responses.
///
/// Wraps an internal channel and serializes each value as an SSE `data:` event
/// before sending it to the client.
///
/// # Example
///
/// ```rust,ignore
/// use metaxy::{rpc_stream, StreamSender};
///
/// #[rpc_stream]
/// async fn chat(input: ChatInput, tx: StreamSender) {
///     for token in generate_tokens(&input.prompt) {
///         tx.send(token).await.ok();
///     }
/// }
/// ```
pub struct StreamSender {
    tx: mpsc::Sender<Result<Bytes, std::io::Error>>,
}

impl StreamSender {
    /// Creates a new `StreamSender` wrapping the given channel.
    #[doc(hidden)]
    pub fn new(tx: mpsc::Sender<Result<Bytes, std::io::Error>>) -> Self {
        Self { tx }
    }

    /// Sends a serializable value as an SSE `data:` event.
    ///
    /// The value is serialized to JSON and formatted as:
    /// ```text
    /// data: {"token":"Hello"}\n\n
    /// ```
    pub async fn send<T: Serialize>(&self, data: T) -> Result<(), SendError> {
        let json = serde_json::to_string(&data).map_err(|_| SendError)?;
        let event = format!("data: {json}\n\n");
        self.tx
            .send(Ok(Bytes::from(event)))
            .await
            .map_err(|_| SendError)
    }

    /// Sends a raw string as an SSE `data:` event.
    pub async fn send_raw(&self, data: impl Into<String>) -> Result<(), SendError> {
        let event = format!("data: {}\n\n", data.into());
        self.tx
            .send(Ok(Bytes::from(event)))
            .await
            .map_err(|_| SendError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn send_formats_as_sse_json_event() {
        let (tx, mut rx) = mpsc::channel(16);
        let sender = StreamSender::new(tx);

        sender.send("hello").await.unwrap();

        let received = rx.recv().await.unwrap().unwrap();
        assert_eq!(received, Bytes::from("data: \"hello\"\n\n"));
    }

    #[tokio::test]
    async fn send_formats_struct_as_sse_json_event() {
        #[derive(Serialize)]
        struct Token {
            text: String,
        }

        let (tx, mut rx) = mpsc::channel(16);
        let sender = StreamSender::new(tx);

        sender
            .send(Token {
                text: "hello".to_string(),
            })
            .await
            .unwrap();

        let received = rx.recv().await.unwrap().unwrap();
        assert_eq!(received, Bytes::from("data: {\"text\":\"hello\"}\n\n"));
    }

    #[tokio::test]
    async fn send_raw_formats_as_sse_event() {
        let (tx, mut rx) = mpsc::channel(16);
        let sender = StreamSender::new(tx);

        sender.send_raw("raw text").await.unwrap();

        let received = rx.recv().await.unwrap().unwrap();
        assert_eq!(received, Bytes::from("data: raw text\n\n"));
    }

    #[tokio::test]
    async fn send_returns_error_on_closed_channel() {
        let (tx, rx) = mpsc::channel(16);
        let sender = StreamSender::new(tx);

        // Drop the receiver to close the channel
        drop(rx);

        let result = sender.send("test").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn send_raw_returns_error_on_closed_channel() {
        let (tx, rx) = mpsc::channel(16);
        let sender = StreamSender::new(tx);

        // Drop the receiver to close the channel
        drop(rx);

        let result = sender.send_raw("test").await;
        assert!(result.is_err());
    }

    #[test]
    fn send_error_display() {
        let error = SendError;
        assert_eq!(error.to_string(), "stream channel closed");
    }

    #[tokio::test]
    async fn multiple_sends_produce_separate_events() {
        let (tx, mut rx) = mpsc::channel(16);
        let sender = StreamSender::new(tx);

        sender.send("first").await.unwrap();
        sender.send("second").await.unwrap();
        sender.send("third").await.unwrap();

        let event1 = rx.recv().await.unwrap().unwrap();
        let event2 = rx.recv().await.unwrap().unwrap();
        let event3 = rx.recv().await.unwrap().unwrap();

        assert_eq!(event1, Bytes::from("data: \"first\"\n\n"));
        assert_eq!(event2, Bytes::from("data: \"second\"\n\n"));
        assert_eq!(event3, Bytes::from("data: \"third\"\n\n"));
    }
}
