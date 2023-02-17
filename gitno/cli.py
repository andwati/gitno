import click
import os
import requests

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


def download_gitignore_templates(output_path):
    GITHUB_ACCESS_TOKEN = os.environ.get("GITHUB_ACCESS_TOKEN")
    headers = {"Authorization": f"token {GITHUB_ACCESS_TOKEN}"}

    # Send a GET request to the GitHub API to retrieve a list of available gitignore templates
    response = requests.get(f"{BASE_URL}gitignore/templates", headers=headers)

    if response.status_code == 200:
        templates = [template["name"] for template in response.json()]

        for template in templates:
            url = f"{BASE_URL}gitignore/templates/{template}"
            response = requests.get(url, headers=headers)
            if response.status_code == 200:
                with open(os.path.join(output_path, f"{template}.gitignore"), "w") as f:
                    f.write(response.text)
    else:
        click.echo(f"Error retrieving templates: {response.status_code}")
        raise click.Abort()
