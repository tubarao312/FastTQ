from abc import ABC, abstractmethod


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
    async def listen(self) -> None:
        """Listen to the worker queue."""
        pass
