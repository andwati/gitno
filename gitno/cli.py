import click
import os

BASE_URL = "https://api.github.com/"


@click.group()
def cli():
    pass


def create_gitignore_folder():
    home_directory = os.path.expanduser("~")
    gitignore_folder = os.path.join(home_directory, ".gitno")

    if not os.path.exists(gitignore_folder):
        os.mkdirs(gitignore_folder)

    return gitignore_folder
