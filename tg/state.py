"""Serializable state for tg"""

import json
import os.path

import git


class JSONEncoder(json.JSONEncoder):
    def default(self, o):
        if hasattr(o, '__json__') and callable(o.__json__):
            return o.__json__()
        return super(JSONEncoder, self).default(o)


class Project:
    def __init__(self, path):
        self.path = path

    def __repr__(self):
        return "<Project path={!r}>".format(self.path)

    def __json__(self):
        return {
            'path': self.path
        }

    @property
    def repo(self):
        return git.Repo(self.path)

    def statuses(self):
        statuses = []

        if self.repo.is_dirty():
            statuses.append("git repo is dirty")

        return statuses


class Tg:
    @staticmethod
    def max_len(values, minimum=0):
        return

    @classmethod
    def load(cls, home):
        home = os.path.abspath(home)
        data_path = os.path.join(home, 'data')

        if os.path.exists(data_path):
            with open(data_path, 'r') as f:
                projects = json.load(f)
        else:
            projects = {}

        return cls({k: Project(**v) for k, v in projects.items()}, home)

    def __init__(self, projects={}, home=None):
        self.projects = projects
        self.home = home

    def __json__(self):
        return {
            'projects': self.projects
        }

    def display(self, minimum=10):
        width = max([len(x) for x in self.projects.keys()] + [minimum])

        for name, project in self.projects.items():
            yield ("{:{}}".format(name, width), project)

    def save(self):
        if self.home is None:
            raise Exception("No home directory set")

        if not os.path.exists(self.home):
            os.makedirs(self.home)

        with open(os.path.join(self.home, 'data'), 'w') as f:
            json.dump(self.projects, f, cls=JSONEncoder)
