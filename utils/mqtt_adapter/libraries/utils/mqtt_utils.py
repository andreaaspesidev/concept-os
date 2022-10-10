import re
from typing import Pattern


def topic_to_pattern(topic: str) -> Pattern:
    # Manage # wildcard (only one allowed)
    if "#" in topic:
        # Filter all after #
        topic = topic.split("#", 1)[0]
        topic = topic + ".+"
    # Manage + wildcard
    topic = topic.replace("+", ".+")
    return re.compile(pattern=f"^{topic}$")
