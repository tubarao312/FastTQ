from dataclasses import dataclass


@dataclass
class ManagerConfig:
    """Configuration for the manager.

    ### Attributes
    - `url`: The URL of the manager.
    """

    url: str
