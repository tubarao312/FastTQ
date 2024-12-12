from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import Callable, List


@dataclass
class BrokerConfig:
    """Configuration for a broker."""

    url: str
    exchanges: List[str] = None


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
    async def subscribe(
        self, channels: List[str], handler: Callable[[str, str], None]
    ) -> None:
        """Subscribe to multiple channels or queues and handle incoming messages."""
        pass
