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

from copy import copy, deepcopy
from lingua import DetectionResult, Language


def test_detection_result_copy():
    result = DetectionResult(
        start_index=0,
        end_index=5,
        word_count=1,
        language=Language.ENGLISH,
    )
    result_copy = copy(result)
    assert result_copy == result
    assert result_copy is not result
    assert result_copy.start_index == result.start_index
    assert result_copy.start_index is result.start_index
    assert result_copy.end_index == result.end_index
    assert result_copy.end_index is result.end_index
    assert result_copy.word_count == result.word_count
    assert result_copy.word_count is result.word_count
    assert result_copy.language == result.language
    assert result_copy.language is not result.language


def test_detection_result_deepcopy():
    result = DetectionResult(
        start_index=0,
        end_index=5,
        word_count=1,
        language=Language.ENGLISH,
    )
    result_copy = deepcopy(result)
    assert result_copy == result
    assert result_copy is not result
    assert result_copy.start_index == result.start_index
    assert result_copy.start_index is result.start_index
    assert result_copy.end_index == result.end_index
    assert result_copy.end_index is result.end_index
    assert result_copy.word_count == result.word_count
    assert result_copy.word_count is result.word_count
    assert result_copy.language == result.language
    assert result_copy.language is not result.language
