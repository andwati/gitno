import click
import os
import requests
import json
from tqdm import tqdm

BASE_URL = "https://api.github.com/"


@click.group()
def cli():
    pass


def create_gitignore_folder():
    home_directory = os.path.expanduser("~")
    gitignore_folder = os.path.join(home_directory, ".gitno/templates")

    if not os.path.exists(gitignore_folder):
        os.mkdir(gitignore_folder)

    return gitignore_folder


def download_gitignore_templates(output_path):
    GITHUB_ACCESS_TOKEN = os.environ.get("GITHUB_ACCESS_TOKEN")
    headers = {"Authorization": f"token {GITHUB_ACCESS_TOKEN}"}

    # Send a GET request to the GitHub API to retrieve a list of available gitignore templates
    response = requests.get(f"{BASE_URL}gitignore/templates", headers=headers)

    if response.status_code == 200:
        response_string = response.content.decode("utf-8")
        json_string = json.loads(response_string)

        templates = json_string
        total_templates = len(templates)
        for i, template in enumerate(templates):
            url = f"{BASE_URL}gitignore/templates/{template}"
            response = requests.get(url, headers=headers)

            response_data = json.loads(response.content)
            source = response_data["source"]
            if response.status_code == 200:
                with open(os.path.join(output_path, f"{template}.gitignore"), "w") as f:
                    f.write(source)
            yield i + 1, total_templates
    else:
        click.echo(f"Error retrieving templates: {response.status_code}")
        raise click.Abort()


def list_gitignore_templates():
    gitignore_folder = create_gitignore_folder()
    templates = [
        template.replace(".gitignore", "") for template in os.listdir(gitignore_folder)
    ]
    templates.sort()

    # print available templates
    click.echo("Available gitignore templates:")
    for i, template in enumerate(templates):
        click.echo(f"{i+1}: {template}")


@cli.command()
def help():
    click.echo("Help  text")


@cli.command()
def update():
    # Create the .gitno folder in the user's home directory
    gitignore_folder = create_gitignore_folder()

    # Download all gitignore templates from GitHub to the .gitno folder
    click.echo("Downloading gitignore templates...")
    progress = tqdm(
        download_gitignore_templates(gitignore_folder),
        total=len(os.listdir(gitignore_folder)),
        colour="green",
    )

    for i, total in progress:
        progress.set_description(f"Downloading gitignore template {i+1}/{total}")

    progress.close()
    click.echo("Done.")


@cli.command()
def list():
    list_gitignore_templates()


@cli.command()
def generate():
    gitignore_folder = create_gitignore_folder()
    templates = [
        template.replace(".gitignore", "") for template in os.listdir(gitignore_folder)
    ]

    template_choices = [f"{i+1}. {template}" for i, template in enumerate(templates)]

    if not template_choices:
        click.echo("No gitignore templates found.")
        return

    selected_template = click.prompt(
        "Choose a template to generate a .gitignore file",
        type=click.Choice(templates),
        show_choices=True,
    )

    template_path = os.path.join(gitignore_folder, f"{selected_template}.gitignore")

    with open(template_path, "r") as f:
        template_contents = f.read()

    with open(".gitignore", "w") as f:
        f.write(template_contents)
    click.echo(
        f"Generated .gitignore file based on the '{selected_template}' template."
    )


cli.add_command(update)
cli.add_command(list)
cli.add_command(generate)
cli.add_command(help)


# if __name__ == "__main__":
#     cli()
