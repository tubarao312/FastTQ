import asyncio
from typing import Any
from clients.client_sdks.python.src.models.task import TaskStatus, TaskStatusUpdate
from clients.client_sdks.python.src.publisher import PublisherClient
from src.worker import WorkerApplication, WorkerApplicationConfig
from src.broker import BrokerConfig
from src.manager import ManagerConfig

# GENERAL CONFIGURATION _______________________________________________________
# These configs should be shared across both the publisher and the worker.

# Both the publisher and the worker need to know about the manager.
manager_config = ManagerConfig(url="http://localhost:8000")

# Both the publisher and the worker need to know about the task kinds and
# should have unified names for them.
TASK_1_NAME = "task_1"
TASK_2_NAME = "task_2"

# APPLICATION CONFIGURATION ___________________________________________________

# 1. Configure the manager & broker
broker_config = BrokerConfig(url="http://localhost:5672")

# 2. Configure the worker
worker_config = WorkerApplicationConfig(
    name="test_worker",
    manager_config=manager_config,
    broker_config=broker_config,
)

# 3. Create a worker application
worker_application = WorkerApplication(worker_config)

# 4. Create tasks and register them with the worker application


@worker_application.task(TASK_1_NAME)
async def task_1(input_data: dict[Any, Any]) -> dict[Any, Any]:
    await asyncio.sleep(1)
    return input_data


@worker_application.task(TASK_2_NAME)
async def task_2(input_data: dict[Any, Any]) -> dict[Any, Any]:
    raise Exception("This is a test exception")


# PUBLISHER CONFIGURATION ______________________________________________________


# Create a publisher client
publisher_client = PublisherClient(manager_config)


async def get_task_result() -> TaskStatusUpdate:
    # Publish a task
    task_id = await publisher_client.publish_task(TASK_1_NAME, {"foo": "bar"})

    # Get the status of a task when it's ready
    while True:
        status_update = await publisher_client.get_task_status_update(task_id)
        if status_update.status in (TaskStatus.SUCCESS, TaskStatus.FAILURE):
            return status_update
        await asyncio.sleep(1)


# IDEA - We could also have a publisher_client.await_task_completition() method that simply
# executes the function as if it was a normal function call.

if __name__ == "__main__":
    result = asyncio.run(get_task_result())
    print(result)
