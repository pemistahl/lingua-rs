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
import pytest

from copy import copy, deepcopy
from lingua import IsoCode639_1, IsoCode639_3


def test_iso_code_639_1_name():
    assert IsoCode639_1.EN.name == "EN"


def test_iso_code_639_1_from_str():
    assert IsoCode639_1.from_str("EN") == IsoCode639_1.EN
    assert IsoCode639_1.from_str("en") == IsoCode639_1.EN
    assert IsoCode639_1.from_str("eN") == IsoCode639_1.EN
    with pytest.raises(ValueError, match="Matching enum member not found"):
        IsoCode639_1.from_str("12")


def test_iso_code_639_1_is_comparable():
    assert IsoCode639_1.EN == IsoCode639_1.EN
    assert IsoCode639_1.EN != IsoCode639_1.DE
    assert IsoCode639_1.EN > IsoCode639_1.DE
    assert IsoCode639_1.DE < IsoCode639_1.EN


def test_iso_code_639_1_copy():
    iso_code_copy = copy(IsoCode639_1.EN)
    assert iso_code_copy == IsoCode639_1.EN
    assert iso_code_copy is not IsoCode639_1.EN


def test_iso_code_639_1_deepcopy():
    iso_code_copy = deepcopy(IsoCode639_1.EN)
    assert iso_code_copy == IsoCode639_1.EN
    assert iso_code_copy is not IsoCode639_1.EN


def test_iso_code_639_1_pickle():
    serialized = pickle.dumps(IsoCode639_1.EN)
    deserialized = pickle.loads(serialized)
    assert deserialized == IsoCode639_1.EN


def test_iso_code_639_3_name():
    assert IsoCode639_3.ENG.name == "ENG"


def test_iso_code_639_3_from_str():
    assert IsoCode639_3.from_str("ENG") == IsoCode639_3.ENG
    assert IsoCode639_3.from_str("eng") == IsoCode639_3.ENG
    assert IsoCode639_3.from_str("eNg") == IsoCode639_3.ENG
    with pytest.raises(ValueError, match="Matching enum member not found"):
        IsoCode639_3.from_str("123")


def test_iso_code_639_3_is_comparable():
    assert IsoCode639_3.ENG == IsoCode639_3.ENG
    assert IsoCode639_3.ENG != IsoCode639_3.DEU
    assert IsoCode639_3.ENG > IsoCode639_3.DEU
    assert IsoCode639_3.DEU < IsoCode639_3.ENG


def test_iso_code_639_3_copy():
    iso_code_copy = copy(IsoCode639_3.ENG)
    assert iso_code_copy == IsoCode639_3.ENG
    assert iso_code_copy is not IsoCode639_3.ENG


def test_iso_code_639_3_deepcopy():
    iso_code_copy = deepcopy(IsoCode639_3.ENG)
    assert iso_code_copy == IsoCode639_3.ENG
    assert iso_code_copy is not IsoCode639_3.ENG


def test_iso_code_639_3_pickle():
    serialized = pickle.dumps(IsoCode639_3.ENG)
    deserialized = pickle.loads(serialized)
    assert deserialized == IsoCode639_3.ENG
