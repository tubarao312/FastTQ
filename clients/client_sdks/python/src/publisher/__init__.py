from uuid import UUID

from src.manager import ManagerConfig
from src.models.task import TaskInput, TaskStatusUpdate


class PublisherClient:
    """A client for publishing tasks to the manager."""

    _manager_config: ManagerConfig

    def __init__(self, manager_config: ManagerConfig):
        self._manager_config = manager_config

    # Task Retrieval

    async def publish_task(self, task_kind: str, input_data: TaskInput) -> UUID:
        """Publish a task to the manager.

        ### Arguments
        - `task_kind`: The kind of the task.
        - `input_data`: The data to publish.

        ### Returns
        - `UUID`: The UUID of the task returned by the manager.
        """
        ...

    async def get_task_status_update(self, task_id: UUID) -> TaskStatusUpdate:
        """Get the status of a task by its UUID.

        ### Arguments
        - `task_id`: The UUID of the task.

        ### Returns
        - `TaskStatus`: The status of the task.
        """
        ...
