import asyncio
from dataclasses import dataclass
from typing import List
from uuid import UUID
from broker import create_broker_instance
from broker.core import BrokerClient, BrokerConfig
from manager.client import ManagerClient
from manager.config import ManagerConfig
from models.task import TaskStatus


@dataclass
class WorkerClient:
    """A client for executing tasks from the brokers. The manager is used for service discovery.

    ### Attributes
    - `manager_config`: The configuration for the manager.
    - `_manager_client`: The manager client.

    ### Methods
    - `register_worker`: Register a worker with the manager.
    - `unregister_worker`: Unregister a worker with the manager.
    - `task`: A decorator to register a task with the worker.
    - `listen`: Listen for tasks from the broker and updates its status on the worker.
    - `execute_task`: Execute a task. For every state of the task the status will be updated on the manager asynchronously.
    - `run`: Listen for tasks from the broker and updates its status on the worker. This is the main method to run the worker.
    """

    id: UUID
    manager_config: ManagerConfig
    _manager_client: ManagerClient
    _broker_client: BrokerClient

    task_registry = {}

    def __init__(self, manager_config: ManagerConfig):
        self.id = UUID()
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
            BrokerConfig(broker_url, exchange=self.id)
        )
        await self._broker_client.connect()

    async def unregister_worker(self, worker_id: UUID):
        """Unregister a worker with the manager.

        ### Arguments
        - `worker_id`: The UUID of the worker.
        """

        await self._manager_client.unregister_worker(worker_id)
        await self._broker_client.disconnect()

    def task(self, task_name: str):
        """A decorator to register a task with the worker.

        ### Arguments
        - `task_name`: The name of the task.
        """

        def decorator(func):
            self.task_registry[task_name] = func
            return func

        return decorator

    async def _listen(self, task_name: str):
        """Listen for tasks from the broker and updates its status on the worker."""
        async for input_data in self._broker_client.subscribe(task_name):
            await self.execute_task(task_name, input_data)

    async def _execute_task(self, task_name: str, input_data: dict) -> dict:
        """Execute a task. For every state of the task the status will be updated on the manager asynchronously.

        ### Arguments
        - `task_name`: The name of the task.
        - `input_data`: The input data for the task.

        ### Returns
        - `dict`: The output data of the task.
        """

        task_func = self.task_registry.get(task_name)
        if task_func is None:
            raise ValueError(f"Task {task_name} not registered with the worker.")

        try:
            result = await task_func(input_data)
            await self._manager_client.update_task_status(task_name, result)
        # This should be improved in the future with more specific exceptions
        except Exception:
            await self._manager_client.update_task_status(task_name, TaskStatus.FAILED)
            # Possibly log the exception

    async def run(self):
        """Listen for tasks from the broker and updates its status on the worker."""
        await self.register_worker()

        try:
            asyncio.gather(
                *[self.listen(task_name) for task_name in self.task_registry.keys()]
            )
        except asyncio.CancelledError:
            # Log here if some error occurred
            pass
        finally:
            await self.unregister_worker()
