from typing import Callable, Awaitable, Optional
from uuid import UUID
import asyncio

from dataclasses import dataclass

from src.broker import BrokerConfig
from src.manager import ManagerConfig
from src.models.task import TaskInput, TaskOutput


@dataclass
class WorkerApplicationConfig:
    """Configuration for a worker application. This is passed in on
    initialization of the `WorkerApplication` class, and can come from a config
    file or other sources.

    ### Attributes
    - `name`: The name of the worker.
    - `url`: The URL of the manager.
    - `broker`: The broker to use for communication with the manager.
    """

    name: str
    broker_config: BrokerConfig
    manager_config: ManagerConfig


@dataclass
class WorkerApplication:
    """A worker application that can execute tasks.

    ### Attributes
    - `_config`: The worker application configuration.
    - `_tasks`: A dictionary of tasks that can be executed.
    - `_id`: The ID of the worker, assigned by the manager on registration.

    ### Methods
    - `register_task`: Registers a task function for a specific kind.
    - `task`: Decorator to register a task function for a specific kind.
    - `_publish_result`: Publishes a result to the client.
    - `_execute_task`: Executes a task for a specific kind with known data.
    """

    _config: WorkerApplicationConfig
    _tasks: dict[str, Callable[[TaskInput], Awaitable[TaskOutput]]]
    _id: Optional[UUID]  # NOTE - How should workers identify themselves to the manager?

    def __init__(self, config: WorkerApplicationConfig):
        self._config = config
        self._tasks = {}
        self._id = None

    # Task Registration

    def register_task(
        self, kind: str, task: Callable[[TaskInput], Awaitable[TaskOutput]]
    ):
        """Registers a task function for a specific kind.

        ### Arguments:
        - `kind`: The kind of the task.
        - `task`: The task function to be registered.
        """

        # If the worker has already registered, we can't register the task again
        if self._id is not None:
            raise ValueError(
                "Worker has already registered, so it can't register any new tasks."
            )

        # Register the task in the worker
        self._tasks[kind] = task

    def task(
        self, kind: str
    ) -> Callable[
        [Callable[[TaskInput], Awaitable[TaskOutput]]],
        Callable[[TaskInput], Awaitable[TaskOutput]],
    ]:
        """Decorator to register a task function for a specific kind.

        ### Arguments:
        - `kind`: The kind of the task.

        ### Returns:
        - A decorator that can be used to register a task function for a specific kind.
        """

        def decorator(
            task: Callable[[TaskInput], Awaitable[TaskOutput]],
        ) -> Callable[[TaskInput], Awaitable[TaskOutput]]:
            self.register_task(kind, task)
            return task

        return decorator

    # Worker Registration

    # TODO - Implement this
    async def _register_worker(self):
        """Register the worker with the manager."""
        ...

    # TODO - Implement this
    async def _unregister_worker(self):
        """Unregister the worker with the manager."""
        ...

    # Lifecycle

    async def entrypoint(self):
        """Entrypoint for the worker. This gets called by the CLI tool."""
        await self._register_worker()
        await self._run()
        await self._exit()

    async def _run(self):
        """Run the worker, processing tasks from the broker until shutdown."""
        try:
            while True:
                await asyncio.sleep(0.1)
        except asyncio.CancelledError:
            return

    async def _exit(self):
        """Exit the worker gracefully, unregistering it from the manager and
        cleaning up all resources."""
        await self._unregister_worker()
