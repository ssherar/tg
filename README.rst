``tg``
======

.. image:: https://img.shields.io/pypi/v/tg.svg
    :target: https://warehouse.python.org/project/tg/
    :alt: tg on PyPI

.. image:: https://img.shields.io/pypi/l/tg.svg
    :target: https://warehouse.python.org/project/tg/
    :alt: tg on PyPI

.. image:: https://img.shields.io/travis/borntyping/tg/master.svg
    :target: https://travis-ci.org/borntyping/tg
    :alt: Travis-CI build status for tg

.. image:: https://img.shields.io/github/issues/borntyping/tg.svg
    :target: https://github.com/borntyping/tg/issues
    :alt: GitHub issues for tg

|

A command line tool for managing your repositories.

* `Source on GitHub <https://github.com/borntyping/tg>`_
* `Packages on PyPI <https://warehouse.python.org/project/tg/>`_
* `Builds on Travis CI <https://travis-ci.org/borntyping/tg>`_

*``tg`` is in development and does not yet have many of the proposed features. Ideas and suggestions for features are welcome - create a feature request on `the issue tracker <https://github.com/borntyping/tg/issues>`_.*

Usage
-----

Run `tg --help` for a list of availible subcommands.

.. code::

    Usage:  tg [options] add <name> <path>
            tg [options] list
            tg [options] remove <name>
            tg [options] status
            tg (--help | --version)

    Show information about tagged repositories

    Subcommands:
        add                 Add a repository to the tag
        list                Show repository paths
        remove              Remove a repository from the tag
        status              Show repository statuses

    Options:
        -t, --tags TAGS     Filter by tags (comma seperated)
        -c, --config PATH   Path to configuration file (default: ~/.tg)
        -v, --verbose       Show debug output
        --version           Show the version number
        -h, --help          Show this message

Installation
------------

.. code:: bash

    pip install tg

Licence
-------

`tg` is licenced under the [MIT Licence](http://opensource.org/licenses/MIT).

Authors
-------

Written by [Sam Clements](sam@borntyping.co.uk).
