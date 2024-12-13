import asyncio
from dataclasses import dataclass
from typing import Callable, Awaitable, Optional, Dict

from broker import create_broker_instance, BrokerClient
from manager import ManagerClient
from models.task import TaskInput, TaskOutput, TaskStatus

from worker.config import WorkerApplicationConfig


@dataclass
class WorkerApplication:
    """A worker application that processes tasks from a task queue.

    Attributes:
        _config (WorkerApplicationConfig): Configuration for the worker application
        _tasks (Dict[str, Callable]): Mapping of task kinds to their handler functions
        _broker_client (Optional[BrokerClient]): Client for communicating with the message broker
        _manager_client (ManagerClient): Client for communicating with the task manager
        _id (Optional[str]): Unique identifier assigned by the manager
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
        """Register a task handler function for a specific task kind.

        Args:
            kind: Unique identifier for the task type
            task: Async function that processes tasks of this kind
        """
        self._tasks[kind] = task

    def task(self, kind: str):
        """Decorator for registering task handler functions.

        Args:
            kind: Unique identifier for the task type

        Returns:
            Callable: Decorator function that registers the task handler
        """

        def decorator(task: Callable[[TaskInput], Awaitable[TaskOutput]]):
            self.register_task(kind, task)
            return task

        return decorator

    async def _register_worker(self):
        """Register this worker with the manager and initialize broker connection.

        Raises:
            ConnectionError: If connection to manager or broker fails
        """
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
        """Unregister from the manager and clean up broker connection.

        Raises:
            ValueError: If worker is not registered
        """
        if self._id is None:
            raise ValueError("Worker is not registered.")

        await self._manager_client.unregister_worker(self._id)
        if self._broker_client:
            await self._broker_client.disconnect()

    async def _execute_task(self, kind: str, input_data: TaskInput):
        """Execute a task and update its status in the manager.

        Args:
            kind: Type of task to execute
            input_data: Input data for the task

        Raises:
            ValueError: If task kind is not registered
        """
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
        """Listen for tasks of a specific kind from the broker.

        Args:
            kind: Type of task to listen for

        Raises:
            RuntimeError: If broker client is not initialized
        """
        if not self._broker_client:
            raise RuntimeError("Broker client is not initialized.")

        async for input_data in self._broker_client.listen(kind):
            await self._execute_task(kind, input_data)

    async def entrypoint(self):
        """Start the worker application.

        This method registers the worker, starts listening for tasks,
        and handles graceful shutdown.
        """
        await self._register_worker()
        try:
            await asyncio.gather(*[self._listen(kind) for kind in self._tasks.keys()])
        except asyncio.CancelledError:
            pass
        finally:
            await self._unregister_worker()
