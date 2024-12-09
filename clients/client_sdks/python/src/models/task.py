from typing import Any, Optional
from uuid import UUID
from dataclasses import dataclass
from enum import Enum


# NOTE - These will change in the future if we end up using something other than
# JSON for the task data. E.g. Avro, Protocol Buffers, etc.
TaskInput = dict[Any, Any]
TaskOutput = dict[Any, Any]


@dataclass
class Task:
    """A task that can be published to the manager.

    ### Attributes
    - `id`: The ID of the task.
    - `kind`: The kind/class of the task.
    - `data`: The data of the task.
    """

    id: UUID
    kind: str
    data: Any  # TODO - Decide on a format for the data


class TaskStatus(str, Enum):
    """The status of a task."""

    PENDING = "pending"
    SUCCESS = "success"
    FAILURE = "failure"


@dataclass
class TaskStatusUpdate:
    """The status of a task and its original data.

    ### Attributes
    - `task_id`: The ID of the task.
    - `status`: The status of the task.
    - `result`: The result of the task.
    - `error`: The error of the task.
    """

    task_id: UUID
    status: TaskStatus  # NOTE - This could be done a lot prettier with Rust's typing system. Any suggestions?
    result: Optional[Any] = None
    error: Optional[Any] = None

    @property
    def has_completed(self) -> bool:
        return self.status in [TaskStatus.SUCCESS, TaskStatus.FAILURE]

    @property
    def has_failed(self) -> bool:
        return self.status == TaskStatus.FAILURE
