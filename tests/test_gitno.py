import os
import shutil
import tempfile
import unittest

from gitno import cli

class TestScript(unittest.TestCase):
    def setUp(self):
        self.temp_dir = tempfile.mkdtemp()
        self.output_path = os.path.join(self.temp_dir, "templates")
        os.environ["GITHUB_ACCESS_TOKEN"] = "test_token"

    def tearDown(self):
        shutil.rmtree(self.temp_dir)
        del os.environ["GITHUB_ACCESS_TOKEN"]

    def test_create_gitignore_folder(self):
        
        folder_path = cli.create_gitignore_folder()
        self.assertTrue(os.path.exists(folder_path))

   