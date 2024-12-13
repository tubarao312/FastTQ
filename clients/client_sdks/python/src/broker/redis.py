import json
import aioredis

from broker.config import BrokerConfig
from broker.core import BrokerClient


class RedisBroker(BrokerClient):
    """A implementation of a broker using Redis.

    ### Attributes
    - `url`: The URL of the Redis server.

    ### Methods
    - `connect`: Connect to the Redis server.
    - `subscribe`: Subscribe to multiple channels and handle incoming messages.
    """

    def __init__(self, config: BrokerConfig, exchange: str):
        self.client = None
        self.config = config
        self.exchange = exchange

    async def connect(self) -> None:
        self.client = await aioredis.from_url(self.config.url)

    async def disconnect(self) -> None:
        await self.client.close()

    async def listen(self, queue: str):
        pubsub = self.client.pubsub()
        await pubsub.subscribe(self.exchange + ":" + queue)

        async for message in pubsub.listen():
            if message["type"] == "message":
                yield json.loads(message["data"])
