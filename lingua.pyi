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

from enum import Enum
from pathlib import Path
from typing import FrozenSet, Optional, List


class ConfidenceValue:
    """This class describes a language's confidence value."""

    def __init__(self, language: Language, value: float) -> "ConfidenceValue":
        ...

    @property
    def language(self) -> Language:
        """Return the language associated with this confidence value."""

    @property
    def value(self) -> float:
        """Return the language's confidence value which lies between 0.0 and 1.0."""


class DetectionResult:
    """This class describes a contiguous single-language
    text section within a possibly mixed-language text.
    """

    def __init__(
        self,
        start_index: int,
        end_index: int,
        word_count: int,
        language: Language
    ) -> "DetectionResult":
        ...

    @property
    def start_index(self) -> int:
        """Return the start index of the identified single-language substring."""

    @property
    def end_index(self) -> int:
        """Return the end index of the identified single-language substring."""

    @property
    def word_count(self) -> int:
        """Return the number of words being part of the identified
        single-language substring.
        """

    @property
    def language(self) -> Language:
        """Return the detected language of the identified
        single-language substring.
        """

class Language(Enum):
    """This enum specifies the so far 75 supported languages which can be
    detected by *Lingua*.
    """

    AFRIKAANS = 1
    ALBANIAN = 2
    ARABIC = 3
    ARMENIAN = 4
    AZERBAIJANI = 5
    BASQUE = 6
    BELARUSIAN = 7
    BENGALI = 8
    BOKMAL = 9
    BOSNIAN = 10
    BULGARIAN = 11
    CATALAN = 12
    CHINESE = 13
    CROATIAN = 14
    CZECH = 15
    DANISH = 16
    DUTCH = 17
    ENGLISH = 18
    ESPERANTO = 19
    ESTONIAN = 20
    FINNISH = 21
    FRENCH = 22
    GANDA = 23
    GEORGIAN = 24
    GERMAN = 25
    GREEK = 26
    GUJARATI = 27
    HEBREW = 28
    HINDI = 29
    HUNGARIAN = 30
    ICELANDIC = 31
    INDONESIAN = 32
    IRISH = 33
    ITALIAN = 34
    JAPANESE = 35
    KAZAKH = 36
    KOREAN = 37
    LATIN = 38
    LATVIAN = 39
    LITHUANIAN = 40
    MACEDONIAN = 41
    MALAY = 42
    MAORI = 43
    MARATHI = 44
    MONGOLIAN = 45
    NYNORSK = 46
    PERSIAN = 47
    POLISH = 48
    PORTUGUESE = 49
    PUNJABI = 50
    ROMANIAN = 51
    RUSSIAN = 52
    SERBIAN = 53
    SHONA = 54
    SLOVAK = 55
    SLOVENE = 56
    SOMALI = 57
    SOTHO = 58
    SPANISH = 59
    SWAHILI = 60
    SWEDISH = 61
    TAGALOG = 62
    TAMIL = 63
    TELUGU = 64
    THAI = 65
    TSONGA = 66
    TSWANA = 67
    TURKISH = 68
    UKRAINIAN = 69
    URDU = 70
    VIETNAMESE = 71
    WELSH = 72
    XHOSA = 73
    YORUBA = 74
    ZULU = 75

    @property
    def iso_code_639_1(self) -> IsoCode639_1:
        """Return the ISO 639-1 code of this language."""

    @property
    def iso_code_639_3(self) -> IsoCode639_3:
        """Return the ISO 639-3 code of this language."""

    @classmethod
    def all(cls) -> FrozenSet["Language"]:
        """Return a set of all supported languages."""

    @classmethod
    def all_spoken_ones(cls) -> FrozenSet["Language"]:
        """Return a set of all supported spoken languages."""

    @classmethod
    def all_with_arabic_script(cls) -> FrozenSet["Language"]:
        """Return a set of all languages supporting the Arabic script."""

    @classmethod
    def all_with_cyrillic_script(cls) -> FrozenSet["Language"]:
        """Return a set of all languages supporting the Cyrillic script."""

    @classmethod
    def all_with_devanagari_script(cls) -> FrozenSet["Language"]:
        """Return a set of all languages supporting the Devanagari script."""

    @classmethod
    def all_with_latin_script(cls) -> FrozenSet["Language"]:
        """Return a set of all languages supporting the Latin script."""

    @classmethod
    def from_iso_code_639_1(cls, iso_code: IsoCode639_1) -> "Language":
        """Return the language associated with the ISO 639-1 code
        passed to this method.

        Raises:
            ValueError: if there is no language for the given ISO code
        """

    @classmethod
    def from_iso_code_639_3(cls, iso_code: IsoCode639_3) -> "Language":
        """Return the language associated with the ISO 639-3 code
        passed to this method.

        Raises:
            ValueError: if there is no language for the given ISO code
        """


class IsoCode639_1(Enum):
    """This enum specifies the ISO 639-1 code representations for the
    supported languages.

    ISO 639 is a standardized nomenclature used to classify languages.
    """

    AF = 1
    AR = 2
    AZ = 3
    BE = 4
    BG = 5
    BN = 6
    BS = 7
    CA = 8
    CS = 9
    CY = 10
    DA = 11
    DE = 12
    EL = 13
    EN = 14
    EO = 15
    ES = 16
    ET = 17
    EU = 18
    FA = 19
    FI = 20
    FR = 21
    GA = 22
    GU = 23
    HE = 24
    HI = 25
    HR = 26
    HU = 27
    HY = 28
    ID = 29
    IS = 30
    IT = 31
    JA = 32
    KA = 33
    KK = 34
    KO = 35
    LA = 36
    LG = 37
    LT = 38
    LV = 39
    MI = 40
    MK = 41
    MN = 42
    MR = 43
    MS = 44
    NB = 45
    NL = 46
    NN = 47
    PA = 48
    PL = 49
    PT = 50
    RO = 51
    RU = 52
    SK = 53
    SL = 54
    SN = 55
    SO = 56
    SQ = 57
    SR = 58
    ST = 59
    SV = 60
    SW = 61
    TA = 62
    TE = 63
    TH = 64
    TL = 65
    TN = 66
    TR = 67
    TS = 68
    UK = 69
    UR = 70
    VI = 71
    XH = 72
    YO = 73
    ZH = 74
    ZU = 75


class IsoCode639_3(Enum):
    """This enum specifies the ISO 639-3 code representations for the
    supported languages.

    ISO 639 is a standardized nomenclature used to classify languages.
    """

    AFR = 1
    ARA = 2
    AZE = 3
    BEL = 4
    BEN = 5
    BOS = 6
    BUL = 7
    CAT = 8
    CES = 9
    CYM = 10
    DAN = 11
    DEU = 12
    ELL = 13
    ENG = 14
    EPO = 15
    EST = 16
    EUS = 17
    FAS = 18
    FIN = 19
    FRA = 20
    GLE = 21
    GUJ = 22
    HEB = 23
    HIN = 24
    HRV = 25
    HUN = 26
    HYE = 27
    IND = 28
    ISL = 29
    ITA = 30
    JPN = 31
    KAT = 32
    KAZ = 33
    KOR = 34
    LAT = 35
    LAV = 36
    LIT = 37
    LUG = 38
    MAR = 39
    MKD = 40
    MON = 41
    MRI = 42
    MSA = 43
    NLD = 44
    NNO = 45
    NOB = 46
    PAN = 47
    POL = 48
    POR = 49
    RON = 50
    RUS = 51
    SLK = 52
    SLV = 53
    SNA = 54
    SOM = 55
    SOT = 56
    SPA = 57
    SQI = 58
    SRP = 59
    SWA = 60
    SWE = 61
    TAM = 62
    TEL = 63
    TGL = 64
    THA = 65
    TSN = 66
    TSO = 67
    TUR = 68
    UKR = 69
    URD = 70
    VIE = 71
    XHO = 72
    YOR = 73
    ZHO = 74
    ZUL = 75


class LanguageDetector:
    """This class detects the language of text."""

    def unload_language_models(self):
        """Clear all language models loaded by this LanguageDetector instance.

        This helps to free allocated memory previously consumed by the models.
        """

    def detect_language_of(self, text: str) -> Optional[Language]:
        """Detect the language of text.

        If the language cannot be reliably detected, `None` is returned.

        This method operates in a single thread. If you want to classify
        a very large set of texts, you will probably want to use method
        `detect_languages_in_parallel_of` instead.

        Args:
            text (str): The text whose language should be identified.

        Returns:
            The identified language. If the language cannot be
            reliably detected, `None` is returned.
        """

    def detect_languages_in_parallel_of(self, texts: List[str]) -> List[Optional[Language]]:
        """Detect the languages of all given input texts.

        If the language cannot be reliably detected for a text,
        `None` is put into the result list.

        This method is a good fit if you want to classify a
        very large set of texts. It potentially operates in
        multiple threads, depending on how many idle CPU cores
        are available and how many texts are passed to this method.

        If you do not want or need parallel execution, use method
        `detect_language_of` instead.

        Args:
            texts (List[str]): The texts whose languages should be identified.

        Returns:
            The identified languages. If a language cannot be
            reliably detected, `None` is returned.
        """

    def detect_multiple_languages_of(self, text: str) -> List[DetectionResult]:
        """Attempt to detect multiple languages in mixed-language text.

        This feature is experimental and under continuous development.

        A list of `DetectionResult` is returned containing an entry for each
        contiguous single-language text section as identified by the library.
        Each entry consists of the identified language, a start index and an
        end index. The indices denote the substring that has been identified
        as a contiguous single-language text section.

        This method operates in a single thread. If you want to classify
        a very large set of texts, you will probably want to use method
        `detect_multiple_languages_in_parallel_of` instead.

        Args:
            text (str): The text whose language should be identified.

        Returns:
            A list of detection results. Each result contains the
            identified language, the start index and end index of
            the identified single-language substring.
        """

    def detect_multiple_languages_in_parallel_of(
        self,
        texts: List[str]
    ) -> List[List[DetectionResult]]:
        """Attempt to detect multiple languages in mixed-language text.

        This feature is experimental and under continuous development.

        A list of `DetectionResult` is returned for each text containing an
        entry for each contiguous single-language text section as identified by
        the library. Each entry consists of the identified language, a start index
        and an end index. The indices denote the substring that has been identified
        as a contiguous single-language text section.

        This method is a good fit if you want to classify a very large set of texts.
        It potentially operates in multiple threads, depending on how many idle CPU
        cores are available and how many texts are passed to this method.

        If you do not want or need parallel execution, use method
        `detect_multiple_languages_of` instead.

        Args:
            texts (List[str]): The texts whose language should be identified.

        Returns:
            A list of lists of detection results. Each result contains the
            identified language, the start index and end index of
            the identified single-language substring.
        """

    def compute_language_confidence_values(self, text: str) -> List[ConfidenceValue]:
        """Compute confidence values for each language supported
        by this detector for the given text.

        The confidence values denote how likely it is that the
        given text has been written in any of the languages
        supported by this detector.

        A list is returned containing those languages which the
        calling instance of LanguageDetector has been built from.
        The entries are sorted by their confidence value in
        descending order. Each value is a probability between
        0.0 and 1.0. The probabilities of all languages will sum to 1.0.
        If the language is unambiguously identified by the rule engine,
        the value 1.0 will always be returned for this language. The
        other languages will receive a value of 0.0.

        This method operates in a single thread. If you want to classify
        a very large set of texts, you will probably want to use method
        `compute_language_confidence_values_in_parallel` instead.

        Args:
            text (str): The text for which to compute confidence values.

        Returns:
            A list of confidence values. Each entry contains a language
            and the associated confidence value.
        """

    def compute_language_confidence_values_in_parallel(
        self,
        texts: List[str]
    ) -> List[List[ConfidenceValue]]:
        """Compute confidence values for each language supported
        by this detector for all the given input texts.

        The confidence values denote how likely it is that the
        given text has been written in any of the languages
        supported by this detector.

        This method is a good fit if you want to classify a very large set of texts.
        It potentially operates in multiple threads, depending on how many idle CPU
        cores are available and how many texts are passed to this method.

        If you do not want or need parallel execution, use method
        `compute_language_confidence_values` instead.

        Args:
            texts (List[str]): The texts for which to compute confidence values.

        Returns:
            A list of lists of confidence values. Each entry contains a language
            and the associated confidence value.
        """

    def compute_language_confidence(self, text: str, language: Language) -> float:
        """Compute the confidence value for the given language and input text.

        The confidence value denotes how likely it is that the given text
        has been written in the given language. The value that this method
        computes is a number between 0.0 and 1.0. If the language is
        unambiguously identified by the rule engine, the value 1.0 will
        always be returned. If the given language is not supported by this
        detector instance, the value 0.0 will always be returned.

        This method operates in a single thread. If you want to classify
        a very large set of texts, you will probably want to use method
        `compute_language_confidence_in_parallel` instead.

        Args:
            text (str): The text for which to compute the confidence value.

            language (Language):
                The language for which to compute the confidence value.

        Returns:
            A float value between 0.0 and 1.0.
        """

    def compute_language_confidence_in_parallel(
        self,
        texts: List[str],
        language: Language
    ) -> List[float]:
        """Compute the confidence values of all input texts for the given language.

        A confidence value denotes how likely it is that a given text has been
        written in a given language.

        The values that this method computes are numbers between 0.0 and 1.0.
        If the language is unambiguously identified by the rule engine, the
        value 1.0 will always be returned. If the given language is not
        supported by this detector instance, the value 0.0 will always be
        returned.

        This method is a good fit if you want to classify a very large set of texts.
        It potentially operates in multiple threads, depending on how many idle CPU
        cores are available and how many texts are passed to this method.

        If you do not want or need parallel execution, use method
        `compute_language_confidence` instead.

        Args:
            texts (List[str]): The texts for which to compute the confidence values.

            language (Language):
                The language for which to compute the confidence values.

        Returns:
            A list of float values between 0.0 and 1.0.
        """


class LanguageDetectorBuilder:
    """This class configures and creates an instance of LanguageDetector."""

    @classmethod
    def from_all_languages(cls) -> "LanguageDetectorBuilder":
        """Create and return an instance of LanguageDetectorBuilder
        with all built-in languages.
        """

    @classmethod
    def from_all_spoken_languages(cls) -> "LanguageDetectorBuilder":
        """Create and return an instance of LanguageDetectorBuilder
        with all built-in spoken languages.
        """

    @classmethod
    def from_all_languages_with_arabic_script(cls) -> "LanguageDetectorBuilder":
        """Create and return an instance of LanguageDetectorBuilder
        with all built-in languages supporting the Arabic script.
        """

    @classmethod
    def from_all_languages_with_cyrillic_script(cls) -> "LanguageDetectorBuilder":
        """Create and return an instance of LanguageDetectorBuilder
        with all built-in languages supporting the Cyrillic script.
        """

    @classmethod
    def from_all_languages_with_devanagari_script(cls) -> "LanguageDetectorBuilder":
        """Create and return an instance of LanguageDetectorBuilder
        with all built-in languages supporting the Devanagari script.
        """

    @classmethod
    def from_all_languages_with_latin_script(cls) -> "LanguageDetectorBuilder":
        """Create and return an instance of LanguageDetectorBuilder
        with all built-in languages supporting the Latin script.
        """

    @classmethod
    def from_all_languages_without(cls, *languages: Language) -> "LanguageDetectorBuilder":
        """Create and return an instance of LanguageDetectorBuilder
        with all built-in languages except those passed to this method.
        """

    @classmethod
    def from_languages(cls, *languages: Language) -> "LanguageDetectorBuilder":
        """Create and return an instance of LanguageDetectorBuilder
        with the languages passed to this method.
        """

    @classmethod
    def from_iso_codes_639_1(cls, *iso_codes: IsoCode639_1) -> "LanguageDetectorBuilder":
        """Create and return an instance of LanguageDetectorBuilder
        with the languages specified by the ISO 639-1 codes passed
        to this method.

        Raises:
            ValueError: if less than two ISO codes are specified
        """

    @classmethod
    def from_iso_codes_639_3(cls, *iso_codes: IsoCode639_3) -> "LanguageDetectorBuilder":
        """Create and return an instance of LanguageDetectorBuilder
        with the languages specified by the ISO 639-3 codes passed
        to this method.

        Raises:
            ValueError: if less than two ISO codes are specified
        """

    def with_minimum_relative_distance(self, distance: float) -> "LanguageDetectorBuilder":
        """Set the desired value for the minimum relative distance measure.

        By default, Lingua returns the most likely language for a given
        input text. However, there are certain words that are spelled the
        same in more than one language. The word 'prologue', for instance,
        is both a valid English and French word. Lingua would output either
        English or French which might be wrong in the given context.
        For cases like that, it is possible to specify a minimum relative
        distance that the logarithmized and summed up probabilities for
        each possible language have to satisfy.

        Be aware that the distance between the language probabilities is
        dependent on the length of the input text. The longer the input
        text, the larger the distance between the languages. So if you
        want to classify very short text phrases, do not set the minimum
        relative distance too high. Otherwise you will get most results
        returned as None which is the return value for cases where
        language detection is not reliably possible.

        Raises:
            ValueError: if distance is smaller than 0.0 or greater than 0.99
        """

    def with_preloaded_language_models(self) -> "LanguageDetectorBuilder":
        """Preload all language models when creating the LanguageDetector
        instance.

        By default, Lingua uses lazy-loading to load only those language
        models on demand which are considered relevant by the rule-based
        filter engine. For web services, for instance, it is rather
        beneficial to preload all language models into memory to avoid
        unexpected latency while waiting for the service response. This
        method allows to switch between these two loading modes.
        """

    def with_low_accuracy_mode(self) -> "LanguageDetectorBuilder":
        """Disable the high accuracy mode in order to save memory
        and increase performance.

        By default, Lingua's high detection accuracy comes at the cost
        of loading large language models into memory which might not be
        feasible for systems running low on resources.

        This method disables the high accuracy mode so that only a small
        subset of language models is loaded into memory. The downside of
        this approach is that detection accuracy for short texts consisting
        of less than 120 characters will drop significantly. However,
        detection accuracy for texts which are longer than 120 characters
        will remain mostly unaffected.
        """

    def build(self) -> LanguageDetector:
        """Create and return the configured LanguageDetector instance."""


class LanguageModelFilesWriter:
    """This class creates language model files and writes them to a directory."""

    @classmethod
    def create_and_write_language_model_files(
        cls,
        input_file_path: Path,
        output_directory_path: Path,
        language: Language,
        char_class: str,
    ):
        """Create language model files and write them to a directory.

        Args:
            input_file_path: The path to a txt file used for language
                model creation. The assumed encoding of the txt file is UTF-8.
            output_directory_path: The path to an existing directory where the
                language model files are to be written.
            language: The language for which to create language models.
            char_class: A regex character class such as \\p{L} to restrict the
                set of characters that the language models are built from.

        Raises:
            Exception: if the input file path is not absolute or does not point
                to an existing txt file; if the input file's encoding is not
                UTF-8; if the output directory path is not absolute or does not
                point to an existing directory; if the character class cannot
                be compiled to a valid regular expression
        """


class TestDataFilesWriter:
    """This class creates test data files for accuracy report generation
    and writes them to a directory.
    """

    @classmethod
    def create_and_write_test_data_files(
        cls,
        input_file_path: Path,
        output_directory_path: Path,
        char_class: str,
        maximum_lines: int,
    ):
        """Create test data files for accuracy report generation and
        write them to a directory.

        Args:
            input_file_path: The path to a txt file used for test data
                creation. The assumed encoding of the txt file is UTF-8.
            output_directory_path: The path to an existing directory where
                the test data files are to be written.
            char_class: A regex character class such as \\p{L} to restrict
                the set of characters that the test data are built from.
            maximum_lines: The maximum number of lines each test data file
                should have.

        Raises:
            Exception: if the input file path is not absolute or does not point
                to an existing txt file; if the input file's encoding is not
                UTF-8; if the output directory path is not absolute or does not
                point to an existing directory; if the character class cannot
                be compiled to a valid regular expression
        """
