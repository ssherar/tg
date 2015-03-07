"""Tests the tg cli"""

import functools
import json

import click.testing
import pytest

from tg.cli import main

PROJECTS = {
    'example': {
        'path': '/example/path'
    }
}


@pytest.fixture(scope="module")
def runner():
    return click.testing.CliRunner(env={'TG_HOME': '.'})


@pytest.yield_fixture
def invoke(runner, request):
    with runner.isolated_filesystem():
        yield functools.partial(runner.invoke, main, catch_exceptions=False)


@pytest.yield_fixture
def data(runner):
    """Write example PROJECTS to a file, and return a function to read them"""

    with open('data', 'w') as f:
        json.dump(PROJECTS, f)

    with open('data', 'r') as f:
        yield f.read


def test_add(invoke, data):
    invoke(['add', '.', 'PROJECT'])
    assert 'PROJECT' in data()


def test_list(invoke, data):
    for name in PROJECTS.keys():
        assert name in invoke(['list']).output


def test_remove(invoke, data):
    for name in PROJECTS.keys():
        invoke(['remove', name])

    for name in PROJECTS.keys():
        assert name not in data()
