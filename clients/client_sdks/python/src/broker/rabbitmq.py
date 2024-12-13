import json
from broker.config import BrokerConfig
from broker.core import BrokerClient
from aio_pika import connect_robust, ExchangeType


class RabbitMQBroker(BrokerClient):
    """RabbitMQ implementation of the broker interface.

    Attributes:
        config (BrokerConfig): Configuration for connecting to RabbitMQ
        exchange_name (str): Name of the primary exchange
        worker_id (str): Unique identifier for this worker instance
        connection: Active connection to RabbitMQ server
        channel: Active channel for communication
        exchange: Declared exchange for message routing
    """

    def __init__(self, config: BrokerConfig, exchange_name: str, worker_id: str):
        self.config = config
        self.exchange_name = exchange_name
        self.worker_id = worker_id  # Add worker_id to identify this worker

    async def connect(self) -> None:
        """Establish connection to RabbitMQ server and setup channel.

        Raises:
            ConnectionError: If connection to RabbitMQ fails
        """
        self.connection = await connect_robust(
            self.config.url, login=self.config.username, password=self.config.password
        )
        self.channel = await self.connection.channel()

        # Mirror the backend's exchange setup
        self.exchange = await self.channel.declare_exchange(
            self.exchange_name, type=ExchangeType.DIRECT, durable=True
        )

    async def disconnect(self) -> None:
        """Clean up RabbitMQ resources and close connection."""
        # Remove the exchanges
        await self.channel.exchange_delete(self.exchange_name)
        await self.connection.close()

    async def listen(self, task_type: str):
        """Listen for tasks of a specific type.

        Args:
            task_type: Type of tasks to listen for

        Yields:
            str: Decoded message body containing task data

        Raises:
            ConnectionError: If broker connection is lost
        """
        # Use worker ID as queue name and routing key
        queue_instance = await self.channel.declare_queue(
            self.worker_id,  # Use worker ID as queue name
            durable=True,
            auto_delete=False,
        )

        # Create exchange for this task type
        exchange = await self.channel.declare_exchange(
            task_type,  # Use task type as exchange name
            type=ExchangeType.DIRECT,
            durable=True,
        )

        # Bind using worker ID as routing key
        await queue_instance.bind(
            exchange.name,
            routing_key=self.worker_id,  # Use worker ID as routing key
        )

        async for message in queue_instance.iterator():
            async with message.process():
                yield json.loads(message.body.decode()), message.message_id

        await queue_instance.delete()
