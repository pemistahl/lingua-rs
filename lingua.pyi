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
from typing import FrozenSet, Optional, List, Set


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
    """This enum specifies the so far 82 supported languages which can be
    detected by *Lingua*.
    """

    AFRIKAANS = 1
    ALBANIAN = 2
    AMHARIC = 3
    ARABIC = 4
    ARMENIAN = 5
    ASSAMESE = 6
    AZERBAIJANI = 7
    BASQUE = 8
    BELARUSIAN = 9
    BENGALI = 10
    BOKMAL = 11
    BOSNIAN = 12
    BULGARIAN = 13
    CATALAN = 14
    CHINESE = 15
    CROATIAN = 16
    CZECH = 17
    DANISH = 18
    DUTCH = 19
    ENGLISH = 20
    ESPERANTO = 21
    ESTONIAN = 22
    FINNISH = 23
    FRENCH = 24
    GANDA = 25
    GEORGIAN = 26
    GERMAN = 27
    GREEK = 28
    GUJARATI = 29
    HEBREW = 30
    HINDI = 31
    HUNGARIAN = 32
    ICELANDIC = 33
    INDONESIAN = 34
    IRISH = 35
    ITALIAN = 36
    JAPANESE = 37
    KANNADA = 38
    KAZAKH = 39
    KOREAN = 40
    KURDISH = 41
    LAO = 42
    LATIN = 43
    LATVIAN = 44
    LITHUANIAN = 45
    MACEDONIAN = 46
    MALAY = 47
    MALAYALAM = 48
    MAORI = 49
    MARATHI = 50
    MONGOLIAN = 51
    NYNORSK = 52
    PERSIAN = 53
    POLISH = 54
    PORTUGUESE = 55
    PUNJABI = 56
    ROMANIAN = 57
    RUSSIAN = 58
    SERBIAN = 59
    SHONA = 60
    SLOVAK = 61
    SLOVENE = 62
    SOMALI = 63
    SOTHO = 64
    SPANISH = 65
    SWAHILI = 66
    SWEDISH = 67
    TAGALOG = 68
    TAMIL = 69
    TELUGU = 70
    THAI = 71
    TSONGA = 72
    TSWANA = 73
    TURKISH = 74
    UKRAINIAN = 75
    URDU = 76
    UZBEK = 77
    VIETNAMESE = 78
    WELSH = 79
    XHOSA = 80
    YORUBA = 81
    ZULU = 82

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

    @classmethod
    def from_str(cls, string: str) -> "Language":
        """Return the language associated with the string representation
        passed to this method.

        Raises:
            ValueError: if there is no language for the given string representation
        """


class IsoCode639_1(Enum):
    """This enum specifies the ISO 639-1 code representations for the
    supported languages.

    ISO 639 is a standardized nomenclature used to classify languages.
    """

    AF = 1
    AM = 2
    AR = 3
    AS = 4
    AZ = 5
    BE = 6
    BG = 7
    BN = 8
    BS = 9
    CA = 10
    CS = 11
    CY = 12
    DA = 13
    DE = 14
    EL = 15
    EN = 16
    EO = 17
    ES = 18
    ET = 19
    EU = 20
    FA = 21
    FI = 22
    FR = 23
    GA = 24
    GU = 25
    HE = 26
    HI = 27
    HR = 28
    HU = 29
    HY = 30
    ID = 31
    IS = 32
    IT = 33
    JA = 34
    KA = 35
    KK = 36
    KN = 37
    KO = 38
    KU = 39
    LA = 40
    LG = 41
    LO = 42
    LT = 43
    LV = 44
    MI = 45
    MK = 46
    ML = 47
    MN = 48
    MR = 49
    MS = 50
    NB = 51
    NL = 52
    NN = 53
    PA = 54
    PL = 55
    PT = 56
    RO = 57
    RU = 58
    SK = 59
    SL = 60
    SN = 61
    SO = 62
    SQ = 63
    SR = 64
    ST = 65
    SV = 66
    SW = 67
    TA = 68
    TE = 69
    TH = 70
    TL = 71
    TN = 72
    TR = 73
    TS = 74
    UK = 75
    UR = 76
    UZ = 77
    VI = 78
    XH = 79
    YO = 80
    ZH = 81
    ZU = 82

    @classmethod
    def from_str(cls, string: str) -> "IsoCode639_1":
        """Return the ISO 639-1 code associated with the string representation
        passed to this method.

        Raises:
            ValueError: if there is no ISO 639-1 code for the given string representation
        """


class IsoCode639_3(Enum):
    """This enum specifies the ISO 639-3 code representations for the
    supported languages.

    ISO 639 is a standardized nomenclature used to classify languages.
    """

    AFR = 1
    AMH = 2
    ARA = 3
    ASM = 4
    AZE = 5
    BEL = 6
    BEN = 7
    BOS = 8
    BUL = 9
    CAT = 10
    CES = 11
    CYM = 12
    DAN = 13
    DEU = 14
    ELL = 15
    ENG = 16
    EPO = 17
    EST = 18
    EUS = 19
    FAS = 20
    FIN = 21
    FRA = 22
    GLE = 23
    GUJ = 24
    HEB = 25
    HIN = 26
    HRV = 27
    HUN = 28
    HYE = 29
    IND = 30
    ISL = 31
    ITA = 32
    JPN = 33
    KAN = 34
    KAT = 35
    KAZ = 36
    KOR = 37
    KUR = 38
    LAO = 39
    LAT = 40
    LAV = 41
    LIT = 42
    LUG = 43
    MAL = 44
    MAR = 45
    MKD = 46
    MON = 47
    MRI = 48
    MSA = 49
    NLD = 50
    NNO = 51
    NOB = 52
    PAN = 53
    POL = 54
    POR = 55
    RON = 56
    RUS = 57
    SLK = 58
    SLV = 59
    SNA = 60
    SOM = 61
    SOT = 62
    SPA = 63
    SQI = 64
    SRP = 65
    SWA = 66
    SWE = 67
    TAM = 68
    TEL = 69
    TGL = 70
    THA = 71
    TSN = 72
    TSO = 73
    TUR = 74
    UKR = 75
    URD = 76
    UZB = 77
    VIE = 78
    XHO = 79
    YOR = 80
    ZHO = 81
    ZUL = 82

    @classmethod
    def from_str(cls, string: str) -> "IsoCode639_3":
        """Return the ISO 639-3 code associated with the string representation
        passed to this method.

        Raises:
            ValueError: if there is no ISO 639-3 code for the given string representation
        """


class LanguageDetector:
    """This class detects the language of text.

    A single instance of `LanguageDetector` can be used safely in multiple threads.
    Multiple instances of `LanguageDetector` share thread-safe access to the
    language models, so every language model is loaded into memory just once,
    no matter how many instances of `LanguageDetector` have been created.
    """

    def unload_language_models(self):
        """Clear all language models loaded by this `LanguageDetector` instance.

        This helps to free allocated memory previously consumed by the models.
        The freed memory will not be returned back to the operating system
        but will be reused e.g. for language models loaded by different
        LanguageDetector instances.
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


class UniqueNgramsWriter:
    """This class determines ngrams being unique to any specific
    language and writes them to a directory."""

    @classmethod
    def create_and_write_unique_ngram_files(
        cls,
        output_directory_path: Path,
    ):
        """Create unique ngram files from the current language
        models and writes them to a directory.

        Args:
            output_directory_path: The path to an existing directory where
                the unique ngram files are to be written.

        Raises:
            Exception: if the output directory path is not absolute or does not
                point to an existing directory.
        """


class MostCommonNgramsWriter:
    """This class determines the most common ngrams for each supported
    language and writes them to a directory."""

    @classmethod
    def create_and_write_most_common_ngram_files(
        cls,
        output_directory_path: Path,
        languages: Set["Language"],
        most_common: int,
    ):
        """Create most common ngram files from the current language
        models and writes them to a directory.

        Args:
            output_directory_path: The path to an existing directory where
                the most common ngram files are to be written.
            languages: The languages to determine the most common ngrams for.
            most_common: The amount of most common ngrams to be identified.

        Raises:
            Exception: if the output directory path is not absolute or does not
                point to an existing directory.
            ValueError: if `languages` is empty or `most_common` is less than or equal to zero
        """
