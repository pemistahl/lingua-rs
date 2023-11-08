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

import pytest

from lingua import (
    ConfidenceValue,
    Language,
    LanguageDetector,
    LanguageDetectorBuilder
)

detector_for_english_and_german = (
    LanguageDetectorBuilder.from_languages(
        Language.ENGLISH, Language.GERMAN)
    .with_preloaded_language_models()
    .build()
)


def test_detect_language():
    assert (
        detector_for_english_and_german
        .detect_language_of("Alter")
        == Language.GERMAN
    )


def test_no_language_is_returned():
    assert (
        detector_for_english_and_german
        .detect_language_of("проарплап")
        is None
    )


def test_detect_multiple_languages_for_empty_string():
    assert (
        detector_for_english_and_german
        .detect_multiple_languages_of("")
        == []
    )


@pytest.mark.parametrize(
    ",".join(
        [
            "sentence",
            "expected_first_substring",
            "expected_first_word_count",
            "expected_first_language",
            "expected_second_substring",
            "expected_second_word_count",
            "expected_second_language",
        ]
    ),
    [
        pytest.param(
            '  He   turned around and asked: "Entschuldigen Sie, sprechen Sie Deutsch?"',
            "  He   turned around and asked: ",
            5,
            Language.ENGLISH,
            '"Entschuldigen Sie, sprechen Sie Deutsch?"',
            5,
            Language.GERMAN,
        )
    ],
)
def test_detect_multiple_languages_with_two_languages(
    sentence,
    expected_first_substring,
    expected_first_word_count,
    expected_first_language,
    expected_second_substring,
    expected_second_word_count,
    expected_second_language,
):
    results = detector_for_english_and_german.detect_multiple_languages_of(sentence)
    assert len(results) == 2

    first_result = results[0]
    first_substring = sentence[first_result.start_index : first_result.end_index]
    assert first_substring == expected_first_substring
    assert first_result.word_count == expected_first_word_count
    assert first_result.language == expected_first_language

    second_result = results[1]
    second_substring = sentence[second_result.start_index : second_result.end_index]
    assert second_substring == expected_second_substring
    assert second_result.word_count == expected_second_word_count
    assert second_result.language == expected_second_language


@pytest.mark.parametrize(
    "text,expected_confidence_values",
    [
        pytest.param(
            "groß",
            [
                ConfidenceValue(Language.GERMAN, 1.0),
                ConfidenceValue(Language.ENGLISH, 0.0),
            ],
        ),
        pytest.param(
            "Alter",
            [
                ConfidenceValue(Language.GERMAN, 0.68),
                ConfidenceValue(Language.ENGLISH, 0.32),
            ],
        ),
        pytest.param(
            "проарплап",
            [
                ConfidenceValue(Language.ENGLISH, 0.0),
                ConfidenceValue(Language.GERMAN, 0.0),
            ],
        ),
    ],
)
def test_compute_language_confidence_values(
    text, expected_confidence_values
):
    confidence_values = (
        detector_for_english_and_german.compute_language_confidence_values(
            text
        )
    )
    assert len(confidence_values) == 2

    first, second = confidence_values
    expected_first, expected_second = expected_confidence_values

    assert first.language == expected_first.language
    assert round(first.value, 2) == expected_first.value

    assert second.language == expected_second.language
    assert round(second.value, 2) == expected_second.value


@pytest.mark.parametrize(
    "text,expected_confidence_for_german,expected_confidence_for_english",
    [
        pytest.param("groß", 1.0, 0.0),
        pytest.param("Alter", 0.68, 0.32),
        pytest.param("проарплап", 0.0, 0.0),
    ],
)
def test_compute_language_confidence(
    text,
    expected_confidence_for_german,
    expected_confidence_for_english,
):
    confidence_for_german = (
        detector_for_english_and_german.compute_language_confidence(
            text, Language.GERMAN
        )
    )
    assert round(confidence_for_german, 2) == expected_confidence_for_german

    confidence_for_english = (
        detector_for_english_and_german.compute_language_confidence(
            text, Language.ENGLISH
        )
    )
    assert round(confidence_for_english, 2) == expected_confidence_for_english

    confidence_for_french = (
        detector_for_english_and_german.compute_language_confidence(
            text, Language.FRENCH
        )
    )
    assert confidence_for_french == 0.0
