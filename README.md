# gitno

[![PyPI version](https://img.shields.io/pypi/v/gitno.svg)](https://pypi.org/project/gitno/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/andwati>/gitno/blob/main/LICENSE)
[![Python versions](https://img.shields.io/pypi/pyversions/gitno.svg)](https://pypi.org/project/gitno/)

Gitno is a command-line tool that generates `.gitignore` files based on the templates available in the [github/gitignore](https://github.com/github/gitignore) repository.

## Installation

You can install `gitno` using pip:

```sh
pip install gitno
```
## Usage

Create a `GITHUB_ACCESS_TOKEN` in your settings then add it to your environment variables

Initialize a local copy of the .gitignore files by running:

```sh
gitno update
```

To generate a .gitignore file on the current working directory, run the generate command followed by the template number or name:

```sh
gitno generate
```

To see a list of available templates, run the list command:

```sh
gitno list
```
## Contributing
Contributions are welcome! Please see CONTRIBUTING.md for details.

## License
This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.