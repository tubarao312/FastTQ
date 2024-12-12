from abc import ABC, abstractmethod
from dataclasses import dataclass


@dataclass
class BrokerConfig:
    """Configuration for a broker."""

    url: str
    exchange_name: str


class BrokerClient(ABC):
    @abstractmethod
    async def connect(self) -> None:
        """Connect to the broker."""
        pass

    @abstractmethod
    async def disconnect(self) -> None:
        """Disconnect from the broker."""
        pass

    @abstractmethod
    async def listen(self, queue: str) -> None:
        """Listen to a specific queue."""
        pass
