import asyncio
from dataclasses import dataclass
from typing import Callable, Awaitable, Optional, Dict

from broker import create_broker_instance, BrokerClient
from manager import ManagerClient
from models.task import TaskInput, TaskOutput, TaskStatus

from worker.config import WorkerApplicationConfig


@dataclass
class WorkerApplication:
    """A worker application that can execute tasks.

    ### Attributes
    - `_config`: The worker application configuration.
    - `_tasks`: A dictionary of tasks that can be executed.

    ### Methods
    - `register_task`: Registers a task function for a specific kind.
    - `task`: Decorator to register a task function for a specific kind.
    - `_publish_result`: Publishes a result to the client.
    - `_execute_task`: Executes a task for a specific kind with known data.
    - `entrypoint`: Main entrypoint to start the worker application.
    """

    _config: WorkerApplicationConfig
    _tasks: Dict[str, Callable[[TaskInput], Awaitable[TaskOutput]]]
    _broker_client: Optional[BrokerClient]
    _manager_client: ManagerClient

    def __init__(self, config: WorkerApplicationConfig):
        self._config = config
        self._id = None

        self._manager_client = ManagerClient(config.manager_config)
        self._tasks = {}

    def register_task(
        self, kind: str, task: Callable[[TaskInput], Awaitable[TaskOutput]]
    ):
        """Registers a task function for a specific kind.

        ### Arguments:
        - `kind`: The kind of the task.
        - `task`: The task function to be registered.
        """
        self._tasks[kind] = task

    def task(self, kind: str):
        """Decorator to register a task function for a specific kind.

        ### Arguments:
        - `kind`: The kind of the task.

        ### Returns:
        - A decorator to register a task function.
        """

        def decorator(task: Callable[[TaskInput], Awaitable[TaskOutput]]):
            self.register_task(kind, task)
            return task

        return decorator

    async def _register_worker(self):
        """Register the worker with the manager and set up the broker client."""
        worker = await self._manager_client.register_worker(
            self._config.name, list(self._tasks.keys())
        )
        self._id = worker

        # For this ideally we would get the broker information from the manager
        self._broker_client = create_broker_instance(
            self._config.broker_config, self._config.name, str(self._id)
        )
        await self._broker_client.connect()

    async def _unregister_worker(self):
        """Unregister the worker with the manager and disconnect the broker client."""
        if self._id is None:
            raise ValueError("Worker is not registered.")

        await self._manager_client.unregister_worker(self._id)
        if self._broker_client:
            await self._broker_client.disconnect()

    async def _execute_task(self, kind: str, input_data: TaskInput):
        """Execute a task and update its status asynchronously."""
        task_func = self._tasks.get(kind)
        if task_func is None:
            raise ValueError(f"Task {kind} not registered.")

        try:
            result = await task_func(input_data)
            await self._manager_client.update_task_status(
                kind, TaskStatus.COMPLETED, result
            )
        except Exception as e:
            await self._manager_client.update_task_status(kind, TaskStatus.FAILED)
            # Log the exception (could improve error handling)

    async def _listen(self, kind: str):
        """Listen for tasks from the broker and execute them."""
        if not self._broker_client:
            raise RuntimeError("Broker client is not initialized.")

        async for input_data in self._broker_client.listen(kind):
            await self._execute_task(kind, input_data)

    async def entrypoint(self):
        """Main entrypoint to start the worker application."""
        await self._register_worker()
        try:
            await asyncio.gather(*[self._listen(kind) for kind in self._tasks.keys()])
        except asyncio.CancelledError:
            pass
        finally:
            await self._unregister_worker()
