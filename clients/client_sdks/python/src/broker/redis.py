import asyncio
import aioredis
from typing import Callable, List

from broker.core import BrokerClient


class RedisBroker(BrokerClient):
    """A implementation of a broker using Redis.

    ### Attributes
    - `url`: The URL of the Redis server.

    ### Methods
    - `connect`: Connect to the Redis server.
    - `subscribe`: Subscribe to multiple channels and handle incoming messages.
    """

    def __init__(self, url: str):
        self.url = url
        self.client = None

    async def connect(self) -> None:
        self.client = await aioredis.from_url(self.url)

    async def disconnect(self) -> None:
        await self.client.close()

    async def subscribe(
        self, channels: List[str], handler: Callable[[str, str], None]
    ) -> None:
        pubsub = self.client.pubsub()
        await pubsub.subscribe(*channels)

        async def listener():
            async for message in pubsub.listen():
                if message["type"] == "message":
                    channel = message["channel"].decode()
                    data = message["data"].decode()
                    await handler(channel, data)

        asyncio.create_task(listener())
