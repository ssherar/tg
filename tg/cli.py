# coding=utf8
"""Command line interface to tg"""

import os.path

import click

import tg
import tg.state


def checkmark(value):
    return "✗" if bool(value) else "✓"


def max_len(values, minimum=0):
    return max(max(len(x) for x in values), minimum)


@click.group()
@click.option(
    '--home', default=os.path.expanduser('~/.tg'), envvar='TG_HOME',
    type=click.Path(file_okay=False, readable=True, writable=True),
    help="Path to a directory that tg will store projects in")
@click.version_option(tg.__version__)
@click.pass_context
def main(ctx, home):
    ctx.obj = tg.state.Tg.load(home)


@main.command()
@click.argument('name')
@click.argument('path')
@click.pass_obj
def add(tg, name, path):
    tg.projects[name] = os.path.abspath(path)
    tg.save()


@main.command()
@click.pass_obj
def list(tg):
    for name, path in tg.projects.items():
        print("{}: {}".format(name, path))


@main.command()
@click.argument('name')
@click.pass_obj
def remove(tg, name):
    del tg.projects[name]
    tg.save()


@main.command()
@click.pass_obj
def status(tg):
    width = max_len(tg.projects.keys(), minimum=10)

    for name, repo in tg.repos():
        statuses = []

        if repo.is_dirty():
            statuses.append("git repo is dirty")

        click.echo("{status} {name:{width}}  {statuses}".format(
            status=checkmark(statuses), name=name, width=width,
            statuses=', '.join(statuses)))
