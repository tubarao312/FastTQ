from src.manager import ManagerClient, ManagerConfig, ManagerStates
import pytest

MANAGER_TEST_URL = "http://localhost:3000"

pytest_plugins = ["pytest_asyncio"]


@pytest.fixture
async def manager_config() -> ManagerConfig:
    """Fixture that provides a configured ManagerConfig instance."""
    return ManagerConfig(url=MANAGER_TEST_URL)


@pytest.fixture
async def manager_client(manager_config: ManagerConfig) -> ManagerClient:
    """Fixture that provides a configured ManagerClient instance. Also checks
    whether the manager client is running and healthy before starting the tests."""

    client = ManagerClient(config=manager_config)

    # Check if the manager is healthy
    client_health = await client.check_health()

    if client_health != ManagerStates.HEALTHY:
        raise RuntimeError(f"Manager is not healthy. Current state: {client_health}")

    return client
