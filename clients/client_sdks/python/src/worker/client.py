from dataclasses import dataclass
from typing import List
from uuid import UUID
from broker import create_broker_instance
from broker.core import BrokerClient, BrokerConfig
from manager.client import ManagerClient
from manager.config import ManagerConfig


@dataclass
class WorkerClient:
    """A client for executing tasks from the brokers. The manager is used for service discovery.

    ### Attributes
    - `manager_config`: The configuration for the manager.
    - `_manager_client`: The manager client.

    ### Methods
    - `register_worker`: Register a worker with the manager.
    """

    manager_config: ManagerConfig
    _manager_client: ManagerClient
    _broker_client: BrokerClient

    def __init__(self, manager_config: ManagerConfig):
        self._manager_config = manager_config
        self._manager_client = ManagerClient(manager_config)

    async def register_worker(self, worker_name: str, task_kinds: List[str]) -> UUID:
        """Register a worker with the manager. The broker will also be created based on the task kinds.

        ### Arguments
        - `worker_name`: The name of the worker.
        - `task_kinds`: The kinds of tasks the worker can execute.

        ### Returns
        - `UUID`: The UUID of the worker.
        """

        broker_url = await self._manager_client.register_worker(worker_name, task_kinds)
        self._broker_client = create_broker_instance(
            BrokerConfig(broker_url, exchanges=task_kinds)
        )
        await self._broker_client.connect()

    async def unregister_worker(self, worker_id: UUID):
        """Unregister a worker with the manager.

        ### Arguments
        - `worker_id`: The UUID of the worker.
        """

        await self._manager_client.unregister_worker(worker_id)
        await self._broker_client.disconnect()

    async def run(self):
        """Listen for tasks from the broker and updates its status on the worker."""
        pass
