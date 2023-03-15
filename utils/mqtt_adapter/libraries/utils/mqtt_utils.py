# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

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
