#
# Copyright © 2020-present Peter M. Stahl pemistahl@gmail.com
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
# http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either expressed or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import os
import pytest

from pathlib import Path
from tempfile import NamedTemporaryFile, TemporaryDirectory

from lingua import Language, LanguageModelFilesWriter, TestDataFilesWriter

# prevent pytest from trying to collect methods in TestDataFilesWriter as tests
TestDataFilesWriter.__test__ = False


@pytest.fixture
def language_model_files_text():
    return (
        "These sentences are intended for testing purposes.\n"
        "Do not use them in production!\n"
        "By the way, they consist of 23 words in total."
    )


@pytest.fixture
def test_data_files_text():
    return (
        "There are many attributes associated with good software.\n"
        "Some of these can be mutually contradictory, and different customers and participants may have different priorities.\n"
        "Weinberg provides an example of how different goals can have a dramatic effect on both effort required and efficiency.\n"
        "Furthermore, he notes that programmers will generally aim to achieve any explicit goals which may be set, probably at the expense of any other quality attributes.\n"
        "Sommerville has identified four generalised attributes which are not concerned with what a program does, but how well the program does it:\n"
        "Maintainability, Dependability, Efficiency, Usability\n"
    )


def test_language_model_files_writer(language_model_files_text):
    input_file = create_temp_input_file(language_model_files_text)
    input_file_path = Path(input_file.name)

    output_directory = TemporaryDirectory()
    output_directory_path = Path(output_directory.name)

    LanguageModelFilesWriter.create_and_write_language_model_files(
        input_file_path=input_file_path,
        output_directory_path=output_directory_path,
        language=Language.ENGLISH,
        char_class="\\p{L}",
    )

    files = read_directory_content(output_directory_path)

    assert len(files) == 5
    assert files[4] == "unigrams.json.br"
    assert files[0] == "bigrams.json.br"
    assert files[3] == "trigrams.json.br"
    assert files[2] == "quadrigrams.json.br"
    assert files[1] == "fivegrams.json.br"


def test_test_data_files_writer(test_data_files_text):
    input_file = create_temp_input_file(test_data_files_text)
    input_file_path = Path(input_file.name)

    output_directory = TemporaryDirectory()
    output_directory_path = Path(output_directory.name)

    TestDataFilesWriter.create_and_write_test_data_files(
        input_file_path=input_file_path,
        output_directory_path=output_directory_path,
        char_class="\\p{L}",
        maximum_lines=4,
    )

    files = read_directory_content(output_directory_path)

    assert len(files) == 3
    assert files[0] == "sentences.txt"
    assert files[1] == "single-words.txt"
    assert files[2] == "word-pairs.txt"


def test_relative_input_file_path_raises_exception():
    relative_input_file_path = Path("some/relative/path/file.txt")
    expected_error_message = (
        f"Input file path '{relative_input_file_path}' is not absolute"
    )

    with pytest.raises(Exception) as exception_info1:
        LanguageModelFilesWriter.create_and_write_language_model_files(
            input_file_path=relative_input_file_path,
            output_directory_path=Path("/some/output/directory"),
            language=Language.ENGLISH,
            char_class="\\p{L}",
        )
    assert exception_info1.value.args[0] == expected_error_message

    with pytest.raises(Exception) as exception_info2:
        TestDataFilesWriter.create_and_write_test_data_files(
            input_file_path=relative_input_file_path,
            output_directory_path=Path("/some/output/directory"),
            char_class="\\p{L}",
            maximum_lines=4,
        )
    assert exception_info2.value.args[0] == expected_error_message


def test_non_existing_input_file_raises_exception():
    non_existing_input_file_path = (
        Path.cwd() / "some" / "non-existing" / "path" / "file.txt"
    )
    expected_error_message = (
        f"Input file '{non_existing_input_file_path}' does not exist"
    )

    with pytest.raises(Exception) as exception_info1:
        LanguageModelFilesWriter.create_and_write_language_model_files(
            input_file_path=non_existing_input_file_path,
            output_directory_path=Path("/some/output/directory"),
            language=Language.ENGLISH,
            char_class="\\p{L}",
        )
    assert exception_info1.value.args[0] == expected_error_message

    with pytest.raises(Exception) as exception_info2:
        TestDataFilesWriter.create_and_write_test_data_files(
            input_file_path=non_existing_input_file_path,
            output_directory_path=Path("/some/output/directory"),
            char_class="\\p{L}",
            maximum_lines=4,
        )
    assert exception_info2.value.args[0] == expected_error_message


def test_directory_as_input_file_raises_exception():
    input_file = TemporaryDirectory()
    input_file_path = Path(input_file.name)
    expected_error_message = (
        f"Input file path '{input_file_path}' does not represent a regular file"
    )

    with pytest.raises(Exception) as exception_info1:
        LanguageModelFilesWriter.create_and_write_language_model_files(
            input_file_path=input_file_path,
            output_directory_path=Path("/some/output/directory"),
            language=Language.ENGLISH,
            char_class="\\p{L}",
        )
    assert exception_info1.value.args[0] == expected_error_message

    with pytest.raises(Exception) as exception_info2:
        TestDataFilesWriter.create_and_write_test_data_files(
            input_file_path=input_file_path,
            output_directory_path=Path("/some/output/directory"),
            char_class="\\p{L}",
            maximum_lines=4,
        )
    assert exception_info2.value.args[0] == expected_error_message


def test_relative_output_directory_path_raises_exception():
    input_file = create_temp_input_file("some content")
    input_file_path = Path(input_file.name)

    relative_output_directory_path = Path("some/relative/path")
    expected_error_message = (
        f"Output directory path '{relative_output_directory_path}' is not absolute"
    )

    with pytest.raises(Exception) as exception_info1:
        LanguageModelFilesWriter.create_and_write_language_model_files(
            input_file_path=input_file_path,
            output_directory_path=relative_output_directory_path,
            language=Language.ENGLISH,
            char_class="\\p{L}",
        )
    assert exception_info1.value.args[0] == expected_error_message

    with pytest.raises(Exception) as exception_info2:
        TestDataFilesWriter.create_and_write_test_data_files(
            input_file_path=input_file_path,
            output_directory_path=relative_output_directory_path,
            char_class="\\p{L}",
            maximum_lines=4,
        )
    assert exception_info2.value.args[0] == expected_error_message


def test_non_existing_output_directory_path_raises_exception():
    input_file = create_temp_input_file("some content")
    input_file_path = Path(input_file.name)

    non_existing_output_directory_path = (
        Path.cwd() / "some" / "non-existing" / "directory"
    )
    expected_error_message = (
        f"Output directory path '{non_existing_output_directory_path}' does not exist"
    )

    with pytest.raises(Exception) as exception_info1:
        LanguageModelFilesWriter.create_and_write_language_model_files(
            input_file_path=input_file_path,
            output_directory_path=non_existing_output_directory_path,
            language=Language.ENGLISH,
            char_class="\\p{L}",
        )
    assert exception_info1.value.args[0] == expected_error_message

    with pytest.raises(Exception) as exception_info2:
        TestDataFilesWriter.create_and_write_test_data_files(
            input_file_path=input_file_path,
            output_directory_path=non_existing_output_directory_path,
            char_class="\\p{L}",
            maximum_lines=4,
        )
    assert exception_info2.value.args[0] == expected_error_message


def test_file_as_output_directory_raises_exception():
    input_file = create_temp_input_file("some content")
    input_file_path = Path(input_file.name)
    expected_error_message = (
        f"Output directory path '{input_file_path}' does not represent a directory"
    )

    with pytest.raises(Exception) as exception_info1:
        LanguageModelFilesWriter.create_and_write_language_model_files(
            input_file_path=input_file_path,
            output_directory_path=input_file_path,
            language=Language.ENGLISH,
            char_class="\\p{L}",
        )
    assert exception_info1.value.args[0] == expected_error_message

    with pytest.raises(Exception) as exception_info2:
        TestDataFilesWriter.create_and_write_test_data_files(
            input_file_path=input_file_path,
            output_directory_path=input_file_path,
            char_class="\\p{L}",
            maximum_lines=4,
        )
    assert exception_info2.value.args[0] == expected_error_message


def create_temp_input_file(content: str):
    input_file = NamedTemporaryFile()
    input_file.write(bytes(content, "utf-8"))
    input_file.seek(0)
    return input_file


def read_directory_content(directory):
    files = os.listdir(directory)
    files.sort()
    return files
