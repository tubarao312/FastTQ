from broker.core import BrokerClient, BrokerConfig

# TODO: Import only rabbit if tacoq[amqp] is installed
from broker.rabbitmq import RabbitMQBroker
from broker.redis import RedisBroker


def create_broker_instance(config: BrokerConfig) -> BrokerClient:
    """Create a broker instance based on the configuration.

    ### Arguments
    - `config`: The configuration for the broker.

    ### Returns
    - `BrokerClient`: The broker instance.
    """

    if config.url.startswith("amqp"):
        return RabbitMQBroker(config.url, config.exchange_name)
    elif config.url.startswith("redis"):
        return RedisBroker(config.url, config.exchange_name)
    else:
        raise ValueError(f"Unsupported broker URL: {config.url}")
