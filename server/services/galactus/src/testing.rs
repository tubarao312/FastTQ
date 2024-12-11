#[cfg(test)]
pub mod test {
    use std::sync::Arc;

    use async_trait::async_trait;
    use axum_test::TestServer;
    use sqlx::PgPool;

    use common::brokers::{core::BrokerCore, Broker};

    use crate::setup_app;

    /// Initializes a test logger with debug level output that writes to the test writer.
    /// This should be called at the start of test modules to enable logging during tests.
    pub fn init_test_logger() {
        let _ = tracing_subscriber::fmt()
            .with_test_writer()
            .with_max_level(tracing::Level::DEBUG)
            .try_init();
    }

    /// Creates and returns a test server instance with the application router.
    /// This provides a way to make test HTTP requests against the API endpoints.
    pub async fn get_test_server(db_pools: PgPool, broker: Broker) -> TestServer {
        let router = setup_app(db_pools, broker).await;
        TestServer::new(router).unwrap()
    }

    /// Mock implementations for BrokerCore that does nothing
    #[derive(Clone)]
    pub struct MockBrokerCore;

    impl MockBrokerCore {
        pub fn new() -> Self {
            MockBrokerCore
        }
    }

    #[async_trait]
    impl BrokerCore for MockBrokerCore {
        async fn register_queue(
            &self,
            _: &str,
            _: &str,
            _: &str,
        ) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }

        async fn publish_message(
            &self,
            _task_name: &str,
            _worker: &str,
            _message: &[u8],
        ) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }
    }

    /// Creates and returns a broker with a mock core
    pub fn get_mock_broker() -> Broker {
        Broker {
            uri: "mock".to_string(),
            broker: Arc::new(MockBrokerCore::new()),
            workers: Vec::new(),
            workers_index: 0,
        }
    }
}
