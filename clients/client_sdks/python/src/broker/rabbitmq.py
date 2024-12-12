import asyncio

from broker.core import BrokerClient
from aio_pika import connect_robust, Message, ExchangeType
from typing import Callable, List


class RabbitMQBroker(BrokerClient):
    """A implementation of a broker using RabbitMQ.

    ### Attributes
    - `url`: The URL of the RabbitMQ server.
    - `exchange_name`: The name of the exchange.
    - `connection`: The connection to the RabbitMQ server.
    - `channel`: The channel to the RabbitMQ server.

    ### Methods
    - `connect`: Connect to the RabbitMQ server.
    - `subscribe`: Subscribe to multiple queues and handle incoming messages.
    - `publish`: Publish a message to a channel.
    """

    def __init__(self, url: str, exchange_name: str):
        self.url = url
        self.exchange_name = exchange_name
        self.connection = None
        self.channel = None

    async def connect(self) -> None:
        self.connection = await connect_robust(self.url)
        self.channel = await self.connection.channel()
        await self.channel.declare_exchange(self.exchange_name, ExchangeType.DIRECT)

    async def disconnect(self) -> None:
        await self.connection.close()

    async def subscribe(
        self, queues: List[str], handler: Callable[[str, str], None]
    ) -> None:
        async def listen_to_queue(queue_name: str):
            queue = await self.channel.declare_queue(queue_name, durable=True)
            await queue.bind(self.exchange_name, routing_key=queue_name)

            async def wrapped_handler(message):
                async with message.process():
                    await handler(queue_name, message.body.decode())

            await queue.consume(wrapped_handler)

        # Start listening to all queues concurrently
        await asyncio.gather(*(listen_to_queue(queue) for queue in queues))
