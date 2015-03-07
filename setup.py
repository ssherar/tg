from setuptools import setup

setup(
    name='tg',
    version='0.2.0',

    description='A command line tool for managing your repositories',
    long_description=open('README.md').read(),

    author='Sam Clements',
    author_email='sam@borntyping.co.uk',
    url='https://github.com/borntyping/tg',
    license='MIT License',

    packages=[
        'tg'
    ],

    install_requires=[
        'click >=3.3, <4.0',
        'GitPython >=0.3.6, <4.0.0'
    ],

    entry_points={
        'console_scripts': [
            'tg = tg.cli:main',
        ]
    },

    classifiers=[
        'Development Status :: 3 - Alpha',
        'License :: OSI Approved :: MIT License',
        'Programming Language :: Python',
        'Programming Language :: Python :: 3',
        'Programming Language :: Python :: 3.3',
        'Programming Language :: Python :: 3.4'
    ]
)
