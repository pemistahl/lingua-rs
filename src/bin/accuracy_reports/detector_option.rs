/*
 * Copyright © 2020-present Peter M. Stahl pemistahl@gmail.com
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either expressed or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use clap::ValueEnum;
use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Copy, Clone, Eq, PartialEq, Hash, EnumIter, ValueEnum, Display)]
pub(crate) enum DetectorOption {
    #[strum(to_string = "cld2")]
    Cld2,

    #[strum(to_string = "lingua-high-accuracy")]
    LinguaHighAccuracy,

    #[strum(to_string = "lingua-low-accuracy")]
    LinguaLowAccuracy,

    #[strum(to_string = "lingua-all-single-language-detectors")]
    LinguaAllSingleLanguageDetectors,

    #[strum(to_string = "lingua-afrikaans-detector")]
    LinguaAfrikaansDetector,

    #[strum(to_string = "lingua-albanian-detector")]
    LinguaAlbanianDetector,

    #[strum(to_string = "lingua-arabic-detector")]
    LinguaArabicDetector,

    #[strum(to_string = "lingua-armenian-detector")]
    LinguaArmenianDetector,

    #[strum(to_string = "lingua-azerbaijani-detector")]
    LinguaAzerbaijaniDetector,

    #[strum(to_string = "lingua-basque-detector")]
    LinguaBasqueDetector,

    #[strum(to_string = "lingua-belarusian-detector")]
    LinguaBelarusianDetector,

    #[strum(to_string = "lingua-bengali-detector")]
    LinguaBengaliDetector,

    #[strum(to_string = "lingua-bokmal-detector")]
    LinguaBokmalDetector,

    #[strum(to_string = "lingua-bosnian-detector")]
    LinguaBosnianDetector,

    #[strum(to_string = "lingua-bulgarian-detector")]
    LinguaBulgarianDetector,

    #[strum(to_string = "lingua-catalan-detector")]
    LinguaCatalanDetector,

    #[strum(to_string = "lingua-chinese-detector")]
    LinguaChineseDetector,

    #[strum(to_string = "lingua-croatian-detector")]
    LinguaCroatianDetector,

    #[strum(to_string = "lingua-czech-detector")]
    LinguaCzechDetector,

    #[strum(to_string = "lingua-danish-detector")]
    LinguaDanishDetector,

    #[strum(to_string = "lingua-dutch-detector")]
    LinguaDutchDetector,

    #[strum(to_string = "lingua-english-detector")]
    LinguaEnglishDetector,

    #[strum(to_string = "lingua-esperanto-detector")]
    LinguaEsperantoDetector,

    #[strum(to_string = "lingua-estonian-detector")]
    LinguaEstonianDetector,

    #[strum(to_string = "lingua-finnish-detector")]
    LinguaFinnishDetector,

    #[strum(to_string = "lingua-french-detector")]
    LinguaFrenchDetector,

    #[strum(to_string = "lingua-ganda-detector")]
    LinguaGandaDetector,

    #[strum(to_string = "lingua-georgian-detector")]
    LinguaGeorgianDetector,

    #[strum(to_string = "lingua-german-detector")]
    LinguaGermanDetector,

    #[strum(to_string = "lingua-greek-detector")]
    LinguaGreekDetector,

    #[strum(to_string = "lingua-gujarati-detector")]
    LinguaGujaratiDetector,

    #[strum(to_string = "lingua-hebrew-detector")]
    LinguaHebrewDetector,

    #[strum(to_string = "lingua-hindi-detector")]
    LinguaHindiDetector,

    #[strum(to_string = "lingua-hungarian-detector")]
    LinguaHungarianDetector,

    #[strum(to_string = "lingua-icelandic-detector")]
    LinguaIcelandicDetector,

    #[strum(to_string = "lingua-indonesian-detector")]
    LinguaIndonesianDetector,

    #[strum(to_string = "lingua-irish-detector")]
    LinguaIrishDetector,

    #[strum(to_string = "lingua-italian-detector")]
    LinguaItalianDetector,

    #[strum(to_string = "lingua-japanese-detector")]
    LinguaJapaneseDetector,

    #[strum(to_string = "lingua-kazakh-detector")]
    LinguaKazakhDetector,

    #[strum(to_string = "lingua-korean-detector")]
    LinguaKoreanDetector,

    #[strum(to_string = "lingua-latin-detector")]
    LinguaLatinDetector,

    #[strum(to_string = "lingua-latvian-detector")]
    LinguaLatvianDetector,

    #[strum(to_string = "lingua-lithuanian-detector")]
    LinguaLithuanianDetector,

    #[strum(to_string = "lingua-macedonian-detector")]
    LinguaMacedonianDetector,

    #[strum(to_string = "lingua-malay-detector")]
    LinguaMalayDetector,

    #[strum(to_string = "lingua-maori-detector")]
    LinguaMaoriDetector,

    #[strum(to_string = "lingua-marathi-detector")]
    LinguaMarathiDetector,

    #[strum(to_string = "lingua-mongolian-detector")]
    LinguaMongolianDetector,

    #[strum(to_string = "lingua-nynorsk-detector")]
    LinguaNynorskDetector,

    #[strum(to_string = "lingua-persian-detector")]
    LinguaPersianDetector,

    #[strum(to_string = "lingua-polish-detector")]
    LinguaPolishDetector,

    #[strum(to_string = "lingua-portuguese-detector")]
    LinguaPortugueseDetector,

    #[strum(to_string = "lingua-punjabi-detector")]
    LinguaPunjabiDetector,

    #[strum(to_string = "lingua-romanian-detector")]
    LinguaRomanianDetector,

    #[strum(to_string = "lingua-russian-detector")]
    LinguaRussianDetector,

    #[strum(to_string = "lingua-serbian-detector")]
    LinguaSerbianDetector,

    #[strum(to_string = "lingua-shona-detector")]
    LinguaShonaDetector,

    #[strum(to_string = "lingua-slovak-detector")]
    LinguaSlovakDetector,

    #[strum(to_string = "lingua-slovene-detector")]
    LinguaSloveneDetector,

    #[strum(to_string = "lingua-somali-detector")]
    LinguaSomaliDetector,

    #[strum(to_string = "lingua-sotho-detector")]
    LinguaSothoDetector,

    #[strum(to_string = "lingua-spanish-detector")]
    LinguaSpanishDetector,

    #[strum(to_string = "lingua-swahili-detector")]
    LinguaSwahiliDetector,

    #[strum(to_string = "lingua-swedish-detector")]
    LinguaSwedishDetector,

    #[strum(to_string = "lingua-tagalog-detector")]
    LinguaTagalogDetector,

    #[strum(to_string = "lingua-tamil-detector")]
    LinguaTamilDetector,

    #[strum(to_string = "lingua-telugu-detector")]
    LinguaTeluguDetector,

    #[strum(to_string = "lingua-thai-detector")]
    LinguaThaiDetector,

    #[strum(to_string = "lingua-tsonga-detector")]
    LinguaTsongaDetector,

    #[strum(to_string = "lingua-tswana-detector")]
    LinguaTswanaDetector,

    #[strum(to_string = "lingua-turkish-detector")]
    LinguaTurkishDetector,

    #[strum(to_string = "lingua-ukrainian-detector")]
    LinguaUkrainianDetector,

    #[strum(to_string = "lingua-urdu-detector")]
    LinguaUrduDetector,

    #[strum(to_string = "lingua-venetian-detector")]
    LinguaVenetianDetector,

    #[strum(to_string = "lingua-vietnamese-detector")]
    LinguaVietnameseDetector,

    #[strum(to_string = "lingua-welsh-detector")]
    LinguaWelshDetector,

    #[strum(to_string = "lingua-xhosa-detector")]
    LinguaXhosaDetector,

    #[strum(to_string = "lingua-yoruba-detector")]
    LinguaYorubaDetector,

    #[strum(to_string = "lingua-zulu-detector")]
    LinguaZuluDetector,

    #[strum(to_string = "whatlang")]
    Whatlang,

    #[strum(to_string = "whichlang")]
    Whichlang,
}

impl DetectorOption {
    pub(crate) fn is_single_language_detector(&self) -> bool {
        let detector_name = self.to_string();
        detector_name.starts_with("lingua-") && detector_name.ends_with("-detector")
    }
}

pub(crate) fn default_detector_options() -> Vec<DetectorOption> {
    DetectorOption::iter()
        .filter(|detector_option| {
            *detector_option != DetectorOption::LinguaAllSingleLanguageDetectors
        })
        .collect_vec()
}
