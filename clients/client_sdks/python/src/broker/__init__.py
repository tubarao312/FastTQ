from broker.config import BrokerConfig
from broker.core import BrokerClient

# TODO: Import only rabbit if tacoq[amqp] is installed
from broker.rabbitmq import RabbitMQBroker
# from broker.redis import RedisBroker


def create_broker_instance(
    config: BrokerConfig, exchange_name: str, worker_id: str
) -> BrokerClient:
    """Create a broker instance based on the configuration.

    ### Arguments
    - `config`: The configuration for the broker.
    - `exchange_name`: The name of the exchange.
    - `worker_id`: Unique identifier for this worker.

    ### Returns
    - `BrokerClient`: The broker instance.
    """

    if config.url.startswith("amqp"):
        return RabbitMQBroker(config, exchange_name, worker_id)
    elif config.url.startswith("redis"):
        return RedisBroker(config, exchange_name, worker_id)
    else:
        raise ValueError(f"Unsupported broker URL: {config.url}")
