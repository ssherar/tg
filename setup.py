from setuptools import setup

setup(
    name='tg',
    version='0.1.0',

    description='A command line tool for managing your repositories',
    long_description=open('README.rst').read(),

    author='Sam Clements',
    author_email='sam@borntyping.co.uk',
    url='https://github.com/borntyping/tg',
    license='MIT License',

    packages=[
        'tg'
    ],

    entry_points={
        'console_scripts': [
            'tg = tg:main',
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
