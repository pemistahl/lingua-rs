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

from lingua import IsoCode639_1, IsoCode639_3, Language, LanguageDetectorBuilder


def test_build_from_blacklist_does_not_panic():
    languages = {Language.TURKISH, Language.ROMANIAN}
    LanguageDetectorBuilder.from_all_languages_without(*languages)


def test_cannot_build_from_blacklist():
    with pytest.raises(ValueError) as exception_info:
        LanguageDetectorBuilder.from_all_languages_without(*Language.all())
    assert (
        exception_info.value.args[0]
        == "LanguageDetector needs at least 1 language to choose from"
    )


def test_build_from_whitelist_does_not_panic():
    languages = {Language.GERMAN, Language.ENGLISH}
    LanguageDetectorBuilder.from_languages(*languages)


def test_cannot_build_from_whitelist():
    with pytest.raises(ValueError) as exception_info:
        LanguageDetectorBuilder.from_languages()
    assert (
        exception_info.value.args[0]
        == "LanguageDetector needs at least 1 language to choose from"
    )


def test_build_from_iso_639_1_codes_does_not_panic():
    LanguageDetectorBuilder.from_iso_codes_639_1(
        IsoCode639_1.DE, IsoCode639_1.SV
    )


def test_cannot_build_from_iso_639_1_codes():
    with pytest.raises(ValueError) as exception_info:
        LanguageDetectorBuilder.from_iso_codes_639_1()
    assert (
        exception_info.value.args[0]
        == "LanguageDetector needs at least 1 language to choose from"
    )


def test_build_from_iso_639_3_codes_does_not_panic():
    LanguageDetectorBuilder.from_iso_codes_639_3(
        IsoCode639_3.DEU, IsoCode639_3.SWE
    )


def test_cannot_build_from_iso_639_3_codes():
    with pytest.raises(ValueError) as exception_info:
        LanguageDetectorBuilder.from_iso_codes_639_3()
    assert (
        exception_info.value.args[0]
        == "LanguageDetector needs at least 1 language to choose from"
    )


def test_build_with_minimum_relative_distance_does_not_panic():
    (
        LanguageDetectorBuilder
        .from_all_languages()
        .with_minimum_relative_distance(0.2)
    )


def test_cannot_build_with_minimum_relative_distance():
    builder = LanguageDetectorBuilder.from_all_languages()
    for value in (-0.01, -2.3, 1.0, 1.7):
        with pytest.raises(ValueError) as exception_info:
            builder.with_minimum_relative_distance(value)
        assert (
            exception_info.value.args[0]
            == "Minimum relative distance must lie in between 0.0 and 0.99"
        )


def test_build_with_low_accuracy_mode_does_not_panic():
    (
        LanguageDetectorBuilder
        .from_all_languages()
        .with_low_accuracy_mode()
    )
