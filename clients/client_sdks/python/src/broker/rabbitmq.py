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

    def __init__(self, config: BrokerConfig, exchange_name: str):
        self.config = config
        self.exchange_name = exchange_name

    async def connect(self) -> None:
        self.connection = await connect_robust(
            self.config.url, login=self.config.username, password=self.config.password
        )
        self.channel = await self.connection.channel()
        self.exchange = await self.channel.declare_exchange(
            self.exchange_name, type=ExchangeType.DIRECT
        )

    async def disconnect(self) -> None:
        # Remove the exchanges
        await self.channel.exchange_delete(self.exchange_name)
        await self.connection.close()

    async def listen(self, queue: str):
        queue_instance = await self.channel.declare_queue(queue, durable=False)
        await queue_instance.bind(self.exchange_name, routing_key=queue)

        async for message in queue_instance.iterator():
            async with message.process():
                yield message.body.decode()

        await queue_instance.delete()
