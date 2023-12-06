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


detector_for_all_languages = (
    LanguageDetectorBuilder.from_all_languages()
    .with_preloaded_language_models()
    .build()
)


def test_detect_language():
    assert (
        detector_for_english_and_german
        .detect_language_of("Sprachen sind großartig")
        == Language.GERMAN
    )


def test_detect_languages_in_parallel():
    assert (
        detector_for_english_and_german
        .detect_languages_in_parallel_of([
            "languages are awesome",
            "Sprachen sind großartig"
        ])
        == [Language.ENGLISH, Language.GERMAN]
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
    "sentence,expected_word_count,expected_language",
    [
        pytest.param(
            "I'm really not sure whether multi-language detection is a good idea.",
            11,
            Language.ENGLISH,
            id="ENGLISH 1"
        ),
        pytest.param("I'm frightened! 🙈", 3, Language.ENGLISH, id="ENGLISH 2"),
        pytest.param("V төзімділік спорт", 3, Language.KAZAKH, id="KAZAKH"),
    ],
)
def test_detect_multiple_languages_with_one_language(
    sentence, expected_word_count, expected_language
):
    results = detector_for_all_languages.detect_multiple_languages_of(sentence)
    assert len(results) == 1

    result = results[0]
    substring = sentence[result.start_index : result.end_index]
    assert substring == sentence
    assert result.word_count == expected_word_count
    assert result.language == expected_language


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
            id="ENGLISH,GERMAN"
        ),
        pytest.param(
            "上海大学是一个好大学. It is such a great university.",
            "上海大学是一个好大学. ",
            10,
            Language.CHINESE,
            "It is such a great university.",
            6,
            Language.ENGLISH,
            id="CHINESE,ENGLISH"
        ),
        pytest.param(
            "English German French - Английский язык",
            "English German French - ",
            4,
            Language.ENGLISH,
            "Английский язык",
            2,
            Language.RUSSIAN,
            id="ENGLISH,RUSSIAN"
        ),
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
    results = detector_for_all_languages.detect_multiple_languages_of(sentence)
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
    ",".join(
        [
            "sentence",
            "expected_first_substring",
            "expected_first_word_count",
            "expected_first_language",
            "expected_second_substring",
            "expected_second_word_count",
            "expected_second_language",
            "expected_third_substring",
            "expected_third_word_count",
            "expected_third_language",
        ]
    ),
    [
        pytest.param(
            "Parlez-vous français? Ich spreche Französisch nur ein bisschen. A little bit is better than nothing.",
            "Parlez-vous français? ",
            2,
            Language.FRENCH,
            "Ich spreche Französisch nur ein bisschen. ",
            6,
            Language.GERMAN,
            "A little bit is better than nothing.",
            7,
            Language.ENGLISH,
            id="FRENCH,GERMAN,ENGLISH"
        ),
        pytest.param(
            "Płaszczowo-rurowe wymienniki ciepła Uszczelkowe der blaue himmel über berlin 中文 the quick brown fox jumps over the lazy dog",
            "Płaszczowo-rurowe wymienniki ciepła Uszczelkowe ",
            4,
            Language.POLISH,
            "der blaue himmel über berlin 中文 ",
            7,
            Language.GERMAN,
            "the quick brown fox jumps over the lazy dog",
            9,
            Language.ENGLISH,
            id="POLISH,GERMAN,ENGLISH"
        ),
    ],
)
def test_detect_multiple_languages_with_three_languages(
    sentence,
    expected_first_substring,
    expected_first_word_count,
    expected_first_language,
    expected_second_substring,
    expected_second_word_count,
    expected_second_language,
    expected_third_substring,
    expected_third_word_count,
    expected_third_language,
):
    results = detector_for_all_languages.detect_multiple_languages_of(sentence)
    assert len(results) == 3

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

    third_result = results[2]
    third_substring = sentence[third_result.start_index : third_result.end_index]
    assert third_substring == expected_third_substring
    assert third_result.word_count == expected_third_word_count
    assert third_result.language == expected_third_language


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
    "texts,expected_confidence_values",
    [
        pytest.param(
            ["groß", "Alter", "проарплап"],
            [
                [
                    ConfidenceValue(Language.GERMAN, 1.0),
                    ConfidenceValue(Language.ENGLISH, 0.0),
                ],
                [
                    ConfidenceValue(Language.GERMAN, 0.68),
                    ConfidenceValue(Language.ENGLISH, 0.32),
                ],
                [
                    ConfidenceValue(Language.ENGLISH, 0.0),
                    ConfidenceValue(Language.GERMAN, 0.0),
                ],
            ]
        )
    ]
)
def test_compute_language_confidence_values_in_parallel(
    texts, expected_confidence_values
):
    confidence_values = (
        detector_for_english_and_german
        .compute_language_confidence_values_in_parallel(texts)
    )

    assert len(confidence_values) == 3
    assert len(confidence_values[0]) == 2
    assert len(confidence_values[1]) == 2
    assert len(confidence_values[2]) == 2

    for i, values in enumerate(confidence_values):
        first, second = values
        expected_first, expected_second = expected_confidence_values[i]

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


@pytest.mark.parametrize(
    "texts,expected_confidence_values_for_german,expected_confidence_values_for_english",
    [
        pytest.param(
            ["groß", "Alter", "проарплап"],
            [1.0, 0.68, 0.0],
            [0.0, 0.32, 0.0]
        )
    ]
)
def test_compute_language_confidence_in_parallel(
    texts,
    expected_confidence_values_for_german,
    expected_confidence_values_for_english
):
    confidence_values_for_german = (
        detector_for_english_and_german
        .compute_language_confidence_in_parallel(texts, Language.GERMAN)
    )
    rounded_values_for_german = list(map(
        lambda v: round(v, 2),
        confidence_values_for_german
    ))
    assert rounded_values_for_german == expected_confidence_values_for_german

    confidence_values_for_english = (
        detector_for_english_and_german
        .compute_language_confidence_in_parallel(texts, Language.ENGLISH)
    )
    rounded_values_for_english = list(map(
        lambda v: round(v, 2),
        confidence_values_for_english
    ))
    assert rounded_values_for_english == expected_confidence_values_for_english
