from broker.config import BrokerConfig
from broker.core import BrokerClient
from aio_pika import connect_robust, ExchangeType


class RabbitMQBroker(BrokerClient):
    """A implementation of a broker using RabbitMQ.

    ### Attributes
    - `url`: The URL of the RabbitMQ server.
    - `exchange_name`: The name of the exchange.
    - `connection`: The connection to the RabbitMQ server.
    - `channel`: The channel to the RabbitMQ server.

    ### Methods
    - `connect`: Connect to the RabbitMQ server.
    - `disconnect`: Disconnect from the RabbitMQ server.
    - `listen`: Listen to a queue.
    """

    def __init__(self, config: BrokerConfig, exchange_name: str, worker_id: str):
        self.config = config
        self.exchange_name = exchange_name
        self.worker_id = worker_id  # Add worker_id to identify this worker

    async def connect(self) -> None:
        self.connection = await connect_robust(
            self.config.url, login=self.config.username, password=self.config.password
        )
        self.channel = await self.connection.channel()

        # Mirror the backend's exchange setup
        self.exchange = await self.channel.declare_exchange(
            self.exchange_name, type=ExchangeType.DIRECT, durable=True
        )

    async def disconnect(self) -> None:
        # Remove the exchanges
        await self.channel.exchange_delete(self.exchange_name)
        await self.connection.close()

    async def listen(self, task_type: str):
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
                yield message.body.decode()

        await queue_instance.delete()
