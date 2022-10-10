import sys

from loguru import logger
from .settings import Settings


def init_logger(config: Settings) -> None:
    try:
        # Remove loggers
        logger.remove()
        # Check if logger must be enabled
        if not config.get('logger/enable', True):
            return
        # Enable standard logger
        logger.add(sys.stderr, level=config.get('logger/level', "DEBUG"))
        # Enable file logger if required
        logger_file = config.get('logger/file', False)
        if logger_file:
            # Enable rotation if required
            logger_max_size = config.get('logger/max_size', False)
            if not logger_max_size:
                logger.add(logger_file, level=config.get('logger/level', "DEBUG"))
            else:
                logger.add(logger_file, rotation=logger_max_size, level=config.get('logger/level', "DEBUG"))
            logger.info(f"File logger enabled on '{logger_file}'")
    except (TypeError, ValueError) as ex:
        logger.error("Invalid logger configuration. Please check settings. Fallback logger is used")
