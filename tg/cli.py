# coding=utf8
"""Command line interface to tg"""

import os.path

import click

from tg import __version__
from tg.state import Tg, Project


def checkmark(value):
    if value:
        return click.style('✓', fg='green')
    else:
        return click.style('✗', fg='red')


@click.group()
@click.option(
    '--home', default=os.path.expanduser('~/.tg'), envvar='TG_HOME',
    type=click.Path(
        file_okay=False, resolve_path=True,
        exists=True, readable=True, writable=True),
    help="Path to a directory that tg will store obj in")
@click.version_option(__version__)
@click.pass_context
def main(ctx, home):
    ctx.obj = Tg.load(home)


@main.command(help="Add a new project")
@click.argument('path', type=click.Path(
    file_okay=False, exists=True, readable=True, resolve_path=True))
@click.argument('name', required=False)
@click.pass_obj
def add(tg, path, name):
    tg.projects[name or os.path.basename(path)] = Project(path=path)
    tg.save()


@main.command(help="List all projects and their paths")
@click.pass_obj
def list(tg):
    for name, project in tg.display():
        print("{}: {}".format(name, project.path))


@main.command(help="Remove a project")
@click.argument('name')
@click.pass_obj
def remove(tg, name):
    del tg.projects[name]
    tg.save()


@main.command(help="Rename a project")
@click.argument('old_name')
@click.argument('new_name')
@click.pass_obj
def rename(tg, old_name, new_name):
    tg.projects[new_name] = tg.projects[old_name]
    del tg.projects[old_name]
    tg.save()


@main.command(help="Show the status of each project")
@click.pass_obj
def status(tg):
    for name, project in tg.display():
        statuses = project.statuses()
        click.echo("{} {}  {}".format(
            checkmark(not statuses), name, ', '.join(statuses)))
