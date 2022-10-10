from typing import Optional

import yaml
import ujson


class Settings:
    """
    Class for easily access configuration.
    No slashes are permitted in config keys
    """
    def __init__(self):
        self.file_path = ""
        self.settings_data = None

    def load(self, file_path: str) -> bool:
        """
        Loads a YAML file
        :param file_path: Path to the file
        :returns: True if the file was loaded successfully, false otherwise
        """
        try:
            with open(file_path, 'r') as f:
                self.settings_data = yaml.safe_load(f)
            self.file_path = file_path
            return True
        except (IOError, yaml.YAMLError) as ex:
            self.settings_data = None
            self.file_path = ""
            return False

    def get(self, path: str, default_value: object = None):
        """
        Browse settings specifying a path and a default value
        :param path: Settings key path
        :param default_value: Value to return if the path is not found
        :return: Value at the specified path, or default value
        """
        assert path is not None
        args = path.split("/")
        data = self.settings_data
        for arg in args:
            if not isinstance(data, dict) or arg not in data:
                return default_value
            else:
                data = data[arg]
        return data

    """
    def save(self, file_path: str = None) -> bool:
        if file_path is not None:
            path = file_path
        else:
            path = self.file_path

        try:
            with open(path, "w") as f:
                yaml.dump(self.settings_data, f, default_flow_style=False, allow_unicode=True)
            return True
        except (IOError, yaml.YAMLError) as ex:
            return False
    """

    def dump(self) -> Optional[str]:
        """
        Prints out settings
        :return: String with settings values
        """
        if self.settings_data is not None:
            return ujson.dumps(self.settings_data)
        else:
            return None
