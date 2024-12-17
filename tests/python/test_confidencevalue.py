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

import pickle

from copy import copy, deepcopy
from lingua import ConfidenceValue, Language


def test_confidence_value_copy():
    confidence = ConfidenceValue(Language.ENGLISH, 0.95)
    confidence_copy = copy(confidence)
    assert confidence_copy == confidence
    assert confidence_copy is not confidence
    assert confidence_copy.language == confidence.language
    assert confidence_copy.language is not confidence.language
    assert confidence_copy.value == confidence.value
    assert confidence_copy.value is not confidence.value


def test_confidence_value_deepcopy():
    confidence = ConfidenceValue(Language.ENGLISH, 0.95)
    confidence_copy = deepcopy(confidence)
    assert confidence_copy == confidence
    assert confidence_copy is not confidence
    assert confidence_copy.language == confidence.language
    assert confidence_copy.language is not confidence.language
    assert confidence_copy.value == confidence.value
    assert confidence_copy.value is not confidence.value


def test_confidence_value_pickle():
    confidence = ConfidenceValue(Language.ENGLISH, 0.95)
    serialized = pickle.dumps(confidence)
    deserialized = pickle.loads(serialized)
    assert  deserialized == confidence
