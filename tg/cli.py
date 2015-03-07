# coding=utf8
"""Command line interface to tg"""

import os.path

import click
import termcolor

from tg import __version__
from tg.state import Tg, Project


def checkmark(value):
    if value:
        return termcolor.colored('✓', 'green')
    else:
        return termcolor.colored('✗', 'red')


@click.group()
@click.option(
    '--home', default=os.path.expanduser('~/.tg'), envvar='TG_HOME',
    type=click.Path(file_okay=False, readable=True, writable=True),
    help="Path to a directory that tg will store obj in")
@click.version_option(__version__)
@click.pass_context
def main(ctx, home):
    ctx.obj = Tg.load(home)


@main.command()
@click.argument('name')
@click.argument('path')
@click.pass_obj
def add(tg, name, path):
    tg.projects[name] = Project(path=os.path.abspath(path))
    tg.save()


@main.command()
@click.pass_obj
def list(tg):
    for name, project in tg():
        print("{}: {}".format(name, project.path))


@main.command()
@click.argument('name')
@click.pass_obj
def remove(tg, name):
    del tg.projects[name]
    tg.save()


@main.command()
@click.argument('old_name')
@click.argument('new_name')
@click.pass_obj
def rename(tg, old_name, new_name):
    tg.projects[new_name] = tg.projects[old_name]
    del tg.projects[old_name]
    tg.save()


@main.command()
@click.pass_obj
def status(tg):
    for name, project in tg.display():
        statuses = project.statuses()
        click.echo("{} {}  {}".format(
            checkmark(not statuses), name, ', '.join(statuses)))
