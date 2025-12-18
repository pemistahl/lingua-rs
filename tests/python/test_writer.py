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

import brotli
import pytest
import re

from pathlib import Path
from tempfile import NamedTemporaryFile, TemporaryDirectory

from lingua import (
    Language,
    LanguageModelFilesWriter,
    TestDataFilesWriter,
    MostCommonNgramsWriter,
    UniqueNgramsWriter,
)

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


@pytest.fixture
def expected_unigram_model():
    return """
    {
        "language":"ENGLISH",
        "ngrams":{
            "1/10":"n o s",
            "1/100":"b g l m",
            "1/20":"d r",
            "1/25":"h",
            "1/50":"f w",
            "13/100":"t",
            "3/100":"a c p u y",
            "3/50":"i",
            "7/50":"e"
        }
    }
    """


@pytest.fixture
def expected_bigram_model():
    return """
    {
        "language":"ENGLISH",
        "ngrams":{
            "1/1":"by he",
            "1/10":"nc nd ng no ns od of os si",
            "1/13":"ta to",
            "1/14":"ed em ey",
            "1/2":"fo wa wo",
            "1/3":"al ar ay ce co ct po pr pu uc ur us",
            "1/5":"de do ds du nt on or ot rd re ro rp st",
            "1/6":"io is",
            "2/13":"ti",
            "2/3":"in",
            "2/5":"se",
            "2/7":"es",
            "3/13":"te",
            "3/14":"en",
            "4/13":"th"
        }
    }
    """


@pytest.fixture
def expected_trigram_model():
    return """
    {
        "language":"ENGLISH",
        "ngrams":{
            "1/1":"are ces con cti ded duc for ion ist nce nde not nsi nte odu ose pos pro pur rds rod rpo sis tal the tot uct urp use way wor",
            "1/2":"ons ord ota sti tin tio",
            "1/3":"enc end ent tes",
            "1/4":"ese est hem hes hey ing int sen ses",
            "2/3":"ten"
        }
    }
    """


@pytest.fixture
def expected_quadrigram_model():
    return """
    {
        "language":"ENGLISH",
        "ngrams":{
            "1/1":"cons ctio duct ence ende ente esti hese inte nces nded nsis nten oduc onsi ords oses otal pose prod purp rodu rpos sent sist stin test ting tion tota ucti urpo word",
            "1/2":"tenc tend",
            "1/4":"them thes they"
        }
    }
    """


@pytest.fixture
def expected_fivegram_model():
    return """
    {
        "language":"ENGLISH",
        "ngrams":{
            "1/1":"consi ction ducti ences ended enten estin inten nsist oduct onsis poses produ purpo roduc rpose sente sting tence tende testi these total uctio urpos words",
            "1/2":"ntenc ntend"
        }
    }
    """


@pytest.fixture
def expected_sentences_file_content():
    return (
        "There are many attributes associated with good software.\n"
        "Some of these can be mutually contradictory, and different customers and participants may have different priorities.\n"
        "Weinberg provides an example of how different goals can have a dramatic effect on both effort required and efficiency.\n"
        "Furthermore, he notes that programmers will generally aim to achieve any explicit goals which may be set, probably at the expense of any other quality attributes.\n"
    )


@pytest.fixture
def expected_single_words_file_content():
    return "there\n" "attributes\n" "associated\n" "software\n"


@pytest.fixture
def expected_word_pairs_file_content():
    return (
        "there attributes\n"
        "associated software\n"
        "these mutually\n"
        "contradictory different\n"
    )


def test_language_model_files_writer(
    language_model_files_text,
    expected_unigram_model,
    expected_bigram_model,
    expected_trigram_model,
    expected_quadrigram_model,
    expected_fivegram_model,
):
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

    check_brotli_file(files[0], "bigrams.json.br", expected_bigram_model)
    check_brotli_file(files[1], "fivegrams.json.br", expected_fivegram_model)
    check_brotli_file(files[2], "quadrigrams.json.br", expected_quadrigram_model)
    check_brotli_file(files[3], "trigrams.json.br", expected_trigram_model)
    check_brotli_file(files[4], "unigrams.json.br", expected_unigram_model)


def test_test_data_files_writer(
    test_data_files_text,
    expected_sentences_file_content,
    expected_single_words_file_content,
    expected_word_pairs_file_content,
):
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

    check_test_data_sentences_file(files[0], "sentences.txt", test_data_files_text)
    check_test_data_file(files[1], "single-words.txt", expected_single_words_file_content)
    check_test_data_file(files[2], "word-pairs.txt", expected_word_pairs_file_content)


def test_most_common_ngrams_writer():
    output_directory = TemporaryDirectory()
    output_directory_path = Path(output_directory.name)

    MostCommonNgramsWriter.create_and_write_most_common_ngram_files(
        output_directory_path=output_directory_path,
        languages={Language.ENGLISH, Language.GERMAN},
        most_common=5
    )

    subdirectories = read_directory_content(output_directory_path)
    assert len(subdirectories) == 2

    german_dir_name = Language.GERMAN.iso_code_639_1.name.lower()
    german_dir_path = output_directory_path / german_dir_name

    english_dir_name = Language.ENGLISH.iso_code_639_1.name.lower()
    english_dir_path = output_directory_path / english_dir_name

    assert subdirectories[0] == german_dir_path
    assert subdirectories[1] == english_dir_path

    assert german_dir_path.is_dir()
    assert english_dir_path.is_dir()

    german_most_common_ngram_files = read_directory_content(german_dir_path)
    assert len(german_most_common_ngram_files) == 5

    check_brotli_file(
        german_most_common_ngram_files[0],
        "mostcommon_bigrams.json.br",
        """{"language":"GERMAN","ngrams":["ch","de","ei","en","er"]}"""
    )
    check_brotli_file(
        german_most_common_ngram_files[1],
        "mostcommon_fivegrams.json.br",
        """{"language":"GERMAN","ngrams":["diese","ische","nicht","schen","ungen"]}"""
    )
    check_brotli_file(
        german_most_common_ngram_files[2],
        "mostcommon_quadrigrams.json.br",
        """{"language":"GERMAN","ngrams":["chen","eine","icht","lich","sche"]}"""
    )
    check_brotli_file(
        german_most_common_ngram_files[3],
        "mostcommon_trigrams.json.br",
        """{"language":"GERMAN","ngrams":["der","die","ein","ich","sch"]}"""
    )
    check_brotli_file(
        german_most_common_ngram_files[4],
        "mostcommon_unigrams.json.br",
        """{"language":"GERMAN","ngrams":["e","i","n","r","s"]}"""
    )

    english_most_common_ngram_files = read_directory_content(english_dir_path)
    assert len(english_most_common_ngram_files) == 5

    check_brotli_file(
        english_most_common_ngram_files[0],
        "mostcommon_bigrams.json.br",
        """{"language":"ENGLISH","ngrams":["an","he","in","re","th"]}"""
    )
    check_brotli_file(
        english_most_common_ngram_files[1],
        "mostcommon_fivegrams.json.br",
        """{"language":"ENGLISH","ngrams":["ation","canad","ction","ement","tions"]}"""
    )
    check_brotli_file(
        english_most_common_ngram_files[2],
        "mostcommon_quadrigrams.json.br",
        """{"language":"ENGLISH","ngrams":["atio","ment","that","tion","with"]}"""
    )
    check_brotli_file(
        english_most_common_ngram_files[3],
        "mostcommon_trigrams.json.br",
        """{"language":"ENGLISH","ngrams":["and","ing","ion","the","tio"]}"""
    )
    check_brotli_file(
        english_most_common_ngram_files[4],
        "mostcommon_unigrams.json.br",
        """{"language":"ENGLISH","ngrams":["a","e","i","o","t"]}"""
    )


def test_most_common_ngrams_writer_without_languages():
    output_directory = TemporaryDirectory()
    output_directory_path = Path(output_directory.name)

    with pytest.raises(ValueError) as error_info:
        MostCommonNgramsWriter.create_and_write_most_common_ngram_files(
            output_directory_path=output_directory_path,
            languages=set(),
            most_common=5
        )
        assert error_info.value.args[0] == "Set of languages must not be empty"


def test_most_common_ngrams_writer_without_most_common():
    output_directory = TemporaryDirectory()
    output_directory_path = Path(output_directory.name)

    with pytest.raises(ValueError) as error_info:
        MostCommonNgramsWriter.create_and_write_most_common_ngram_files(
            output_directory_path=output_directory_path,
            languages={Language.ENGLISH, Language.GERMAN},
            most_common=0
        )
        assert error_info.value.args[0] == "Amount of most common ngrams must be greater than zero"


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

    with pytest.raises(Exception) as exception_info3:
        UniqueNgramsWriter.create_and_write_unique_ngram_files(
            output_directory_path=relative_output_directory_path,
        )
        assert exception_info3.value.args[0] == expected_error_message

    with pytest.raises(Exception) as exception_info4:
        MostCommonNgramsWriter.create_and_write_most_common_ngram_files(
            output_directory_path=relative_output_directory_path,
            languages={Language.ENGLISH, Language.GERMAN},
            most_common=5
        )
        assert exception_info4.value.args[0] == expected_error_message


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

    with pytest.raises(Exception) as exception_info3:
        UniqueNgramsWriter.create_and_write_unique_ngram_files(
            output_directory_path=non_existing_output_directory_path,
        )
        assert exception_info3.value.args[0] == expected_error_message

    with pytest.raises(Exception) as exception_info4:
        MostCommonNgramsWriter.create_and_write_most_common_ngram_files(
            output_directory_path=non_existing_output_directory_path,
            languages={Language.ENGLISH, Language.GERMAN},
            most_common=5
        )
        assert exception_info4.value.args[0] == expected_error_message


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

    with pytest.raises(Exception) as exception_info3:
        UniqueNgramsWriter.create_and_write_unique_ngram_files(
            output_directory_path=input_file_path,
        )
        assert exception_info3.value.args[0] == expected_error_message

    with pytest.raises(Exception) as exception_info4:
        MostCommonNgramsWriter.create_and_write_most_common_ngram_files(
            output_directory_path=input_file_path,
            languages={Language.ENGLISH, Language.GERMAN},
            most_common=5
        )
        assert exception_info4.value.args[0] == expected_error_message


def create_temp_input_file(content: str):
    input_file = NamedTemporaryFile()
    input_file.write(bytes(content, "utf-8"))
    input_file.seek(0)
    return input_file


def read_directory_content(directory: Path) -> list[Path]:
    return sorted([child for child in directory.iterdir()])


def check_brotli_file(file_path: Path, expected_file_name: str, expected_file_content: str):
    assert file_path.is_file()
    assert file_path.name == expected_file_name
    with open(file_path, mode="rb") as compressed_file:
        uncompressed_file_content = brotli.decompress(compressed_file.read()).decode("utf-8")
        assert uncompressed_file_content == minify(expected_file_content)


def check_test_data_sentences_file(
    file_path: Path,
    expected_file_name: str,
    test_data_files_text: str
):
    check_file_name(file_path, expected_file_name)
    with file_path.open() as sentences_file:
        sentences = [sentence.strip() for sentence in sentences_file.readlines()]
        assert len(sentences) == 4
        # Sentences are chosen randomly, so we just check
        # if the chosen sentences are part of the original text
        original_sentences = test_data_files_text.split("\n")
        for sentence in sentences:
            assert sentence in original_sentences


def check_test_data_file(file_path: Path, expected_file_name: str, expected_file_content: str):
    check_file_name(file_path, expected_file_name)
    with file_path.open() as txt_file:
        assert txt_file.read() == expected_file_content


def check_file_name(file_path: Path, expected_file_name: str):
    assert file_path.is_file()
    assert file_path.name == expected_file_name


def minify(json: str):
    return re.sub(r"\n\s*", "", json)
