"""Serializable state for tg"""

import json
import os.path


class Tg:
    @classmethod
    def load(cls, home):
        home = os.path.abspath(home)
        data_path = os.path.join(home, 'data')

        if os.path.exists(data_path):
            with open(data_path, 'r') as f:
                projects = json.load(f)
        else:
            projects = {}

        return cls(projects, home)

    def __init__(self, projects={}, home=None):
        self.projects = projects
        self.home = home

    def save(self):
        if self.home is None:
            raise Exception("No home directory set")

        if not os.path.exists(self.home):
            os.makedirs(self.home)

        with open(os.path.join(self.home, 'data'), 'w') as f:
            json.dump(self.projects, f)
