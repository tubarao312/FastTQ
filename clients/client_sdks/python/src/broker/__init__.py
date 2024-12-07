from dataclasses import dataclass


@dataclass
class BrokerConfig:
    """Configuration for a broker."""

    url: str


class Broker:
    """A broker that can send and receive tasks."""

    config: BrokerConfig

    # TODO - Figure out which functions a broker should have.
    # TODO - Add a RabbitMQ and a Redis broker implementation.
