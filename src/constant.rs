/*
 * Copyright © 2020-today Peter M. Stahl pemistahl@gmail.com
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

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::alphabet::CharSet;
use crate::language::Language;

pub(crate) static JAPANESE_CHARACTER_SET: Lazy<CharSet> =
    Lazy::new(|| CharSet::from_classes(&["Hiragana", "Katakana", "Han"]));
pub(crate) static MULTIPLE_WHITESPACE: Lazy<Regex> = Lazy::new(|| Regex::new("\\s+").unwrap());
pub(crate) static NUMBERS: Lazy<Regex> = Lazy::new(|| Regex::new("\\p{N}").unwrap());
pub(crate) static PUNCTUATION: Lazy<Regex> = Lazy::new(|| Regex::new("\\p{P}").unwrap());
pub(crate) static LETTERS: Lazy<Regex> =
    Lazy::new(|| Regex::new("\\p{Han}|\\p{Hangul}|\\p{Hiragana}|\\p{Katakana}|\\p{L}+").unwrap());
pub(crate) static TOKENS_WITH_OPTIONAL_WHITESPACE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        "\\s*(?:\\p{Han}|\\p{Hangul}|\\p{Hiragana}|\\p{Katakana}|[\\p{L}'-]+)[\\p{N}\\p{P}]*\\s*",
    )
    .unwrap()
});
pub(crate) static TOKENS_WITHOUT_WHITESPACE: Lazy<Regex> =
    Lazy::new(|| Regex::new("\\p{Han}|\\p{Hangul}|\\p{Hiragana}|\\p{Katakana}|\\p{L}+").unwrap());

pub(crate) static CHARS_TO_LANGUAGES_MAPPING: Lazy<HashMap<&'static str, HashSet<Language>>> =
    Lazy::new(|| {
        let mut mapping = hashmap!();

        if cfg!(feature = "portuguese") || cfg!(feature = "vietnamese") {
            mapping.insert("Ãã", {
                let mut languages = hashset!();
                if cfg!(feature = "portuguese") {
                    languages.insert(Language::from_str("Portuguese").unwrap());
                }
                if cfg!(feature = "vietnamese") {
                    languages.insert(Language::from_str("Vietnamese").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "lithuanian") || cfg!(feature = "polish") {
            mapping.insert("ĄąĘę", {
                let mut languages = hashset!();
                if cfg!(feature = "lithuanian") {
                    languages.insert(Language::from_str("Lithuanian").unwrap());
                }
                if cfg!(feature = "polish") {
                    languages.insert(Language::from_str("Polish").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "polish") || cfg!(feature = "romanian") {
            mapping.insert("Żż", {
                let mut languages = hashset!();
                if cfg!(feature = "polish") {
                    languages.insert(Language::from_str("Polish").unwrap());
                }
                if cfg!(feature = "romanian") {
                    languages.insert(Language::from_str("Romanian").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "french") || cfg!(feature = "romanian") {
            mapping.insert("Îî", {
                let mut languages = hashset!();
                if cfg!(feature = "french") {
                    languages.insert(Language::from_str("French").unwrap());
                }
                if cfg!(feature = "romanian") {
                    languages.insert(Language::from_str("Romanian").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "basque") || cfg!(feature = "spanish") {
            mapping.insert("Ññ", {
                let mut languages = hashset!();
                if cfg!(feature = "basque") {
                    languages.insert(Language::from_str("Basque").unwrap());
                }
                if cfg!(feature = "spanish") {
                    languages.insert(Language::from_str("Spanish").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "czech") || cfg!(feature = "slovak") {
            mapping.insert("ŇňŤť", {
                let mut languages = hashset!();
                if cfg!(feature = "czech") {
                    languages.insert(Language::from_str("Czech").unwrap());
                }
                if cfg!(feature = "slovak") {
                    languages.insert(Language::from_str("Slovak").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "romanian") || cfg!(feature = "vietnamese") {
            mapping.insert("Ăă", {
                let mut languages = hashset!();
                if cfg!(feature = "romanian") {
                    languages.insert(Language::from_str("Romanian").unwrap());
                }
                if cfg!(feature = "vietnamese") {
                    languages.insert(Language::from_str("Vietnamese").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "azerbaijani") || cfg!(feature = "turkish") {
            mapping.insert("İıĞğ", {
                let mut languages = hashset!();
                if cfg!(feature = "azerbaijani") {
                    languages.insert(Language::from_str("Azerbaijani").unwrap());
                }
                if cfg!(feature = "turkish") {
                    languages.insert(Language::from_str("Turkish").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "macedonian") || cfg!(feature = "serbian") {
            mapping.insert("ЈјЉљЊњ", {
                let mut languages = hashset!();
                if cfg!(feature = "macedonian") {
                    languages.insert(Language::from_str("Macedonian").unwrap());
                }
                if cfg!(feature = "serbian") {
                    languages.insert(Language::from_str("Serbian").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "vietnamese") || cfg!(feature = "yoruba") {
            mapping.insert("ẸẹỌọ", {
                let mut languages = hashset!();
                if cfg!(feature = "vietnamese") {
                    languages.insert(Language::from_str("Vietnamese").unwrap());
                }
                if cfg!(feature = "yoruba") {
                    languages.insert(Language::from_str("Yoruba").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "icelandic") || cfg!(feature = "turkish") {
            mapping.insert("ÐðÞþ", {
                let mut languages = hashset!();
                if cfg!(feature = "icelandic") {
                    languages.insert(Language::from_str("Icelandic").unwrap());
                }
                if cfg!(feature = "turkish") {
                    languages.insert(Language::from_str("Turkish").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "french") || cfg!(feature = "hungarian") {
            mapping.insert("Ûû", {
                let mut languages = hashset!();
                if cfg!(feature = "french") {
                    languages.insert(Language::from_str("French").unwrap());
                }
                if cfg!(feature = "hungarian") {
                    languages.insert(Language::from_str("Hungarian").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "maori") || cfg!(feature = "yoruba") {
            mapping.insert("Ōō", {
                let mut languages = hashset!();
                if cfg!(feature = "maori") {
                    languages.insert(Language::from_str("Maori").unwrap());
                }
                if cfg!(feature = "yoruba") {
                    languages.insert(Language::from_str("Yoruba").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "latvian") || cfg!(feature = "maori") || cfg!(feature = "yoruba") {
            mapping.insert("ĀāĒēĪī", {
                let mut languages = hashset!();
                if cfg!(feature = "latvian") {
                    languages.insert(Language::from_str("Latvian").unwrap());
                }
                if cfg!(feature = "maori") {
                    languages.insert(Language::from_str("Maori").unwrap());
                }
                if cfg!(feature = "yoruba") {
                    languages.insert(Language::from_str("Yoruba").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "azerbaijani") || cfg!(feature = "romanian") || cfg!(feature = "turkish")
        {
            mapping.insert("Şş", {
                let mut languages = hashset!();
                if cfg!(feature = "azerbaijani") {
                    languages.insert(Language::from_str("Azerbaijani").unwrap());
                }
                if cfg!(feature = "romanian") {
                    languages.insert(Language::from_str("Romanian").unwrap());
                }
                if cfg!(feature = "turkish") {
                    languages.insert(Language::from_str("Turkish").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "czech") || cfg!(feature = "romanian") || cfg!(feature = "slovak") {
            mapping.insert("Ďď", {
                let mut languages = hashset!();
                if cfg!(feature = "czech") {
                    languages.insert(Language::from_str("Czech").unwrap());
                }
                if cfg!(feature = "romanian") {
                    languages.insert(Language::from_str("Romanian").unwrap());
                }
                if cfg!(feature = "slovak") {
                    languages.insert(Language::from_str("Slovak").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "bosnian") || cfg!(feature = "croatian") || cfg!(feature = "polish") {
            mapping.insert("Ćć", {
                let mut languages = hashset!();
                if cfg!(feature = "bosnian") {
                    languages.insert(Language::from_str("Bosnian").unwrap());
                }
                if cfg!(feature = "croatian") {
                    languages.insert(Language::from_str("Croatian").unwrap());
                }
                if cfg!(feature = "polish") {
                    languages.insert(Language::from_str("Polish").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "bosnian") || cfg!(feature = "croatian") || cfg!(feature = "vietnamese") {
            mapping.insert("Đđ", {
                let mut languages = hashset!();
                if cfg!(feature = "bosnian") {
                    languages.insert(Language::from_str("Bosnian").unwrap());
                }
                if cfg!(feature = "croatian") {
                    languages.insert(Language::from_str("Croatian").unwrap());
                }
                if cfg!(feature = "vietnamese") {
                    languages.insert(Language::from_str("Vietnamese").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "belarusian") || cfg!(feature = "kazakh") || cfg!(feature = "ukrainian") {
            mapping.insert("Іі", {
                let mut languages = hashset!();
                if cfg!(feature = "belarusian") {
                    languages.insert(Language::from_str("Belarusian").unwrap());
                }
                if cfg!(feature = "kazakh") {
                    languages.insert(Language::from_str("Kazakh").unwrap());
                }
                if cfg!(feature = "ukrainian") {
                    languages.insert(Language::from_str("Ukrainian").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "italian") || cfg!(feature = "vietnamese") || cfg!(feature = "yoruba") {
            mapping.insert("Ìì", {
                let mut languages = hashset!();
                if cfg!(feature = "italian") {
                    languages.insert(Language::from_str("Italian").unwrap());
                }
                if cfg!(feature = "vietnamese") {
                    languages.insert(Language::from_str("Vietnamese").unwrap());
                }
                if cfg!(feature = "yoruba") {
                    languages.insert(Language::from_str("Yoruba").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "bokmal") || cfg!(feature = "danish") || cfg!(feature = "nynorsk") {
            mapping.insert("Øø", {
                let mut languages = hashset!();
                if cfg!(feature = "bokmal") {
                    languages.insert(Language::from_str("Bokmal").unwrap());
                }
                if cfg!(feature = "danish") {
                    languages.insert(Language::from_str("Danish").unwrap());
                }
                if cfg!(feature = "nynorsk") {
                    languages.insert(Language::from_str("Nynorsk").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "latvian")
            || cfg!(feature = "lithuanian")
            || cfg!(feature = "maori")
            || cfg!(feature = "yoruba")
        {
            mapping.insert("Ūū", {
                let mut languages = hashset!();
                if cfg!(feature = "latvian") {
                    languages.insert(Language::from_str("Latvian").unwrap());
                }
                if cfg!(feature = "lithuanian") {
                    languages.insert(Language::from_str("Lithuanian").unwrap());
                }
                if cfg!(feature = "maori") {
                    languages.insert(Language::from_str("Maori").unwrap());
                }
                if cfg!(feature = "yoruba") {
                    languages.insert(Language::from_str("Yoruba").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "afrikaans")
            || cfg!(feature = "albanian")
            || cfg!(feature = "dutch")
            || cfg!(feature = "french")
        {
            mapping.insert("Ëë", {
                let mut languages = hashset!();
                if cfg!(feature = "afrikaans") {
                    languages.insert(Language::from_str("Afrikaans").unwrap());
                }
                if cfg!(feature = "albanian") {
                    languages.insert(Language::from_str("Albanian").unwrap());
                }
                if cfg!(feature = "dutch") {
                    languages.insert(Language::from_str("Dutch").unwrap());
                }
                if cfg!(feature = "french") {
                    languages.insert(Language::from_str("French").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "french")
            || cfg!(feature = "italian")
            || cfg!(feature = "vietnamese")
            || cfg!(feature = "yoruba")
        {
            mapping.insert("ÈèÙù", {
                let mut languages = hashset!();
                if cfg!(feature = "french") {
                    languages.insert(Language::from_str("French").unwrap());
                }
                if cfg!(feature = "italian") {
                    languages.insert(Language::from_str("Italian").unwrap());
                }
                if cfg!(feature = "vietnamese") {
                    languages.insert(Language::from_str("Vietnamese").unwrap());
                }
                if cfg!(feature = "yoruba") {
                    languages.insert(Language::from_str("Yoruba").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "afrikaans")
            || cfg!(feature = "french")
            || cfg!(feature = "portuguese")
            || cfg!(feature = "vietnamese")
        {
            mapping.insert("Êê", {
                let mut languages = hashset!();
                if cfg!(feature = "afrikaans") {
                    languages.insert(Language::from_str("Afrikaans").unwrap());
                }
                if cfg!(feature = "french") {
                    languages.insert(Language::from_str("French").unwrap());
                }
                if cfg!(feature = "portuguese") {
                    languages.insert(Language::from_str("Portuguese").unwrap());
                }
                if cfg!(feature = "vietnamese") {
                    languages.insert(Language::from_str("Vietnamese").unwrap());
                }
                languages
            });
        }

        if cfg!(feature = "estonian")
            || cfg!(feature = "hungarian")
            || cfg!(feature = "portuguese")
            || cfg!(feature = "vietnamese")
        {
            mapping.insert("Õõ", {
                let mut languages = hashset!();
                if cfg!(feature = "estonian") {
                    languages.insert(Language::from_str("Estonian").unwrap());
                }
                if cfg!(feature = "hungarian") {
                    languages.insert(Language::from_str("Hungarian").unwrap());
                }
                if cfg!(feature = "portuguese") {
                    languages.insert(Language::from_str("Portuguese").unwrap());
                }
                if cfg!(feature = "vietnamese") {
                    languages.insert(Language::from_str("Vietnamese").unwrap());
                }
                languages
            });

            if cfg!(feature = "french")
                || cfg!(feature = "portuguese")
                || cfg!(feature = "slovak")
                || cfg!(feature = "vietnamese")
            {
                mapping.insert("Ôô", {
                    let mut languages = hashset!();
                    if cfg!(feature = "french") {
                        languages.insert(Language::from_str("French").unwrap());
                    }
                    if cfg!(feature = "portuguese") {
                        languages.insert(Language::from_str("Portuguese").unwrap());
                    }
                    if cfg!(feature = "slovak") {
                        languages.insert(Language::from_str("Slovak").unwrap());
                    }
                    if cfg!(feature = "vietnamese") {
                        languages.insert(Language::from_str("Vietnamese").unwrap());
                    }
                    languages
                });
            }

            if cfg!(feature = "belarusian")
                || cfg!(feature = "kazakh")
                || cfg!(feature = "mongolian")
                || cfg!(feature = "russian")
            {
                mapping.insert("ЁёЫыЭэ", {
                    let mut languages = hashset!();
                    if cfg!(feature = "belarusian") {
                        languages.insert(Language::from_str("Belarusian").unwrap());
                    }
                    if cfg!(feature = "kazakh") {
                        languages.insert(Language::from_str("Kazakh").unwrap());
                    }
                    if cfg!(feature = "mongolian") {
                        languages.insert(Language::from_str("Mongolian").unwrap());
                    }
                    if cfg!(feature = "russian") {
                        languages.insert(Language::from_str("Russian").unwrap());
                    }
                    languages
                });
            }

            if cfg!(feature = "bulgarian")
                || cfg!(feature = "kazakh")
                || cfg!(feature = "mongolian")
                || cfg!(feature = "russian")
            {
                mapping.insert("ЩщЪъ", {
                    let mut languages = hashset!();
                    if cfg!(feature = "bulgarian") {
                        languages.insert(Language::from_str("Bulgarian").unwrap());
                    }
                    if cfg!(feature = "kazakh") {
                        languages.insert(Language::from_str("Kazakh").unwrap());
                    }
                    if cfg!(feature = "mongolian") {
                        languages.insert(Language::from_str("Mongolian").unwrap());
                    }
                    if cfg!(feature = "russian") {
                        languages.insert(Language::from_str("Russian").unwrap());
                    }
                    languages
                });
            }

            if cfg!(feature = "catalan")
                || cfg!(feature = "italian")
                || cfg!(feature = "vietnamese")
                || cfg!(feature = "yoruba")
            {
                mapping.insert("Òò", {
                    let mut languages = hashset!();
                    if cfg!(feature = "catalan") {
                        languages.insert(Language::from_str("Catalan").unwrap());
                    }
                    if cfg!(feature = "italian") {
                        languages.insert(Language::from_str("Italian").unwrap());
                    }
                    if cfg!(feature = "vietnamese") {
                        languages.insert(Language::from_str("Vietnamese").unwrap());
                    }
                    if cfg!(feature = "yoruba") {
                        languages.insert(Language::from_str("Yoruba").unwrap());
                    }
                    languages
                });
            }

            if cfg!(feature = "french")
                || cfg!(feature = "portuguese")
                || cfg!(feature = "romanian")
                || cfg!(feature = "turkish")
                || cfg!(feature = "vietnamese")
            {
                mapping.insert("Ââ", {
                    let mut languages = hashset!();
                    if cfg!(feature = "french") {
                        languages.insert(Language::from_str("French").unwrap());
                    }
                    if cfg!(feature = "portuguese") {
                        languages.insert(Language::from_str("Portuguese").unwrap());
                    }
                    if cfg!(feature = "romanian") {
                        languages.insert(Language::from_str("Romanian").unwrap());
                    }
                    if cfg!(feature = "turkish") {
                        languages.insert(Language::from_str("Turkish").unwrap());
                    }
                    if cfg!(feature = "vietnamese") {
                        languages.insert(Language::from_str("Vietnamese").unwrap());
                    }
                    languages
                });
            }

            if cfg!(feature = "bokmal")
                || cfg!(feature = "danish")
                || cfg!(feature = "icelandic")
                || cfg!(feature = "nynorsk")
            {
                mapping.insert("Ææ", {
                    let mut languages = hashset!();
                    if cfg!(feature = "bokmal") {
                        languages.insert(Language::from_str("Bokmal").unwrap());
                    }
                    if cfg!(feature = "danish") {
                        languages.insert(Language::from_str("Danish").unwrap());
                    }
                    if cfg!(feature = "icelandic") {
                        languages.insert(Language::from_str("Icelandic").unwrap());
                    }
                    if cfg!(feature = "nynorsk") {
                        languages.insert(Language::from_str("Nynorsk").unwrap());
                    }
                    languages
                });
            }

            if cfg!(feature = "bokmal")
                || cfg!(feature = "danish")
                || cfg!(feature = "nynorsk")
                || cfg!(feature = "swedish")
            {
                mapping.insert("Åå", {
                    let mut languages = hashset!();
                    if cfg!(feature = "bokmal") {
                        languages.insert(Language::from_str("Bokmal").unwrap());
                    }
                    if cfg!(feature = "danish") {
                        languages.insert(Language::from_str("Danish").unwrap());
                    }
                    if cfg!(feature = "nynorsk") {
                        languages.insert(Language::from_str("Nynorsk").unwrap());
                    }
                    if cfg!(feature = "swedish") {
                        languages.insert(Language::from_str("Swedish").unwrap());
                    }
                    languages
                });
            }

            if cfg!(feature = "czech")
                || cfg!(feature = "icelandic")
                || cfg!(feature = "slovak")
                || cfg!(feature = "turkish")
                || cfg!(feature = "vietnamese")
            {
                mapping.insert("Ýý", {
                    let mut languages = hashset!();
                    if cfg!(feature = "czech") {
                        languages.insert(Language::from_str("Czech").unwrap());
                    }
                    if cfg!(feature = "icelandic") {
                        languages.insert(Language::from_str("Icelandic").unwrap());
                    }
                    if cfg!(feature = "slovak") {
                        languages.insert(Language::from_str("Slovak").unwrap());
                    }
                    if cfg!(feature = "turkish") {
                        languages.insert(Language::from_str("Turkish").unwrap());
                    }
                    if cfg!(feature = "vietnamese") {
                        languages.insert(Language::from_str("Vietnamese").unwrap());
                    }
                    languages
                });
            }

            if cfg!(feature = "estonian")
                || cfg!(feature = "finnish")
                || cfg!(feature = "german")
                || cfg!(feature = "slovak")
                || cfg!(feature = "swedish")
            {
                mapping.insert("Ää", {
                    let mut languages = hashset!();
                    if cfg!(feature = "estonian") {
                        languages.insert(Language::from_str("Estonian").unwrap());
                    }
                    if cfg!(feature = "finnish") {
                        languages.insert(Language::from_str("Finnish").unwrap());
                    }
                    if cfg!(feature = "german") {
                        languages.insert(Language::from_str("German").unwrap());
                    }
                    if cfg!(feature = "slovak") {
                        languages.insert(Language::from_str("Slovak").unwrap());
                    }
                    if cfg!(feature = "swedish") {
                        languages.insert(Language::from_str("Swedish").unwrap());
                    }
                    languages
                });
            }

            if cfg!(feature = "catalan")
                || cfg!(feature = "french")
                || cfg!(feature = "italian")
                || cfg!(feature = "portuguese")
                || cfg!(feature = "vietnamese")
            {
                mapping.insert("Àà", {
                    let mut languages = hashset!();
                    if cfg!(feature = "catalan") {
                        languages.insert(Language::from_str("Catalan").unwrap());
                    }
                    if cfg!(feature = "french") {
                        languages.insert(Language::from_str("French").unwrap());
                    }
                    if cfg!(feature = "italian") {
                        languages.insert(Language::from_str("Italian").unwrap());
                    }
                    if cfg!(feature = "portuguese") {
                        languages.insert(Language::from_str("Portuguese").unwrap());
                    }
                    if cfg!(feature = "vietnamese") {
                        languages.insert(Language::from_str("Vietnamese").unwrap());
                    }
                    languages
                });
            }

            if cfg!(feature = "azerbaijani")
                || cfg!(feature = "catalan")
                || cfg!(feature = "estonian")
                || cfg!(feature = "german")
                || cfg!(feature = "hungarian")
                || cfg!(feature = "spanish")
                || cfg!(feature = "turkish")
            {
                mapping.insert("Üü", {
                    let mut languages = hashset!();
                    if cfg!(feature = "azerbaijani") {
                        languages.insert(Language::from_str("Azerbaijani").unwrap());
                    }
                    if cfg!(feature = "catalan") {
                        languages.insert(Language::from_str("Catalan").unwrap());
                    }
                    if cfg!(feature = "estonian") {
                        languages.insert(Language::from_str("Estonian").unwrap());
                    }
                    if cfg!(feature = "german") {
                        languages.insert(Language::from_str("German").unwrap());
                    }
                    if cfg!(feature = "hungarian") {
                        languages.insert(Language::from_str("Hungarian").unwrap());
                    }
                    if cfg!(feature = "spanish") {
                        languages.insert(Language::from_str("Spanish").unwrap());
                    }
                    if cfg!(feature = "turkish") {
                        languages.insert(Language::from_str("Turkish").unwrap());
                    }
                    languages
                });
            }

            if cfg!(feature = "bosnian")
                || cfg!(feature = "czech")
                || cfg!(feature = "croatian")
                || cfg!(feature = "latvian")
                || cfg!(feature = "lithuanian")
                || cfg!(feature = "slovak")
                || cfg!(feature = "slovene")
            {
                mapping.insert("ČčŠšŽž", {
                    let mut languages = hashset!();
                    if cfg!(feature = "bosnian") {
                        languages.insert(Language::from_str("Bosnian").unwrap());
                    }
                    if cfg!(feature = "czech") {
                        languages.insert(Language::from_str("Czech").unwrap());
                    }
                    if cfg!(feature = "croatian") {
                        languages.insert(Language::from_str("Croatian").unwrap());
                    }
                    if cfg!(feature = "latvian") {
                        languages.insert(Language::from_str("Latvian").unwrap());
                    }
                    if cfg!(feature = "lithuanian") {
                        languages.insert(Language::from_str("Lithuanian").unwrap());
                    }
                    if cfg!(feature = "slovak") {
                        languages.insert(Language::from_str("Slovak").unwrap());
                    }
                    if cfg!(feature = "slovene") {
                        languages.insert(Language::from_str("Slovene").unwrap());
                    }
                    languages
                });
            }

            if cfg!(feature = "albanian")
                || cfg!(feature = "azerbaijani")
                || cfg!(feature = "basque")
                || cfg!(feature = "catalan")
                || cfg!(feature = "french")
                || cfg!(feature = "portuguese")
                || cfg!(feature = "turkish")
            {
                mapping.insert("Çç", {
                    let mut languages = hashset!();
                    if cfg!(feature = "albanian") {
                        languages.insert(Language::from_str("Albanian").unwrap());
                    }
                    if cfg!(feature = "azerbaijani") {
                        languages.insert(Language::from_str("Azerbaijani").unwrap());
                    }
                    if cfg!(feature = "basque") {
                        languages.insert(Language::from_str("Basque").unwrap());
                    }
                    if cfg!(feature = "catalan") {
                        languages.insert(Language::from_str("Catalan").unwrap());
                    }
                    if cfg!(feature = "french") {
                        languages.insert(Language::from_str("French").unwrap());
                    }
                    if cfg!(feature = "portuguese") {
                        languages.insert(Language::from_str("Portuguese").unwrap());
                    }
                    if cfg!(feature = "turkish") {
                        languages.insert(Language::from_str("Turkish").unwrap());
                    }
                    languages
                });
            }

            if cfg!(feature = "azerbaijani")
                || cfg!(feature = "estonian")
                || cfg!(feature = "finnish")
                || cfg!(feature = "german")
                || cfg!(feature = "hungarian")
                || cfg!(feature = "icelandic")
                || cfg!(feature = "swedish")
                || cfg!(feature = "turkish")
            {
                mapping.insert("Öö", {
                    let mut languages = hashset!();
                    if cfg!(feature = "azerbaijani") {
                        languages.insert(Language::from_str("Azerbaijani").unwrap());
                    }
                    if cfg!(feature = "estonian") {
                        languages.insert(Language::from_str("Estonian").unwrap());
                    }
                    if cfg!(feature = "finnish") {
                        languages.insert(Language::from_str("Finnish").unwrap());
                    }
                    if cfg!(feature = "german") {
                        languages.insert(Language::from_str("German").unwrap());
                    }
                    if cfg!(feature = "hungarian") {
                        languages.insert(Language::from_str("Hungarian").unwrap());
                    }
                    if cfg!(feature = "icelandic") {
                        languages.insert(Language::from_str("Icelandic").unwrap());
                    }
                    if cfg!(feature = "swedish") {
                        languages.insert(Language::from_str("Swedish").unwrap());
                    }
                    if cfg!(feature = "turkish") {
                        languages.insert(Language::from_str("Turkish").unwrap());
                    }
                    languages
                });
            }

            if cfg!(feature = "catalan")
                || cfg!(feature = "hungarian")
                || cfg!(feature = "icelandic")
                || cfg!(feature = "irish")
                || cfg!(feature = "polish")
                || cfg!(feature = "portuguese")
                || cfg!(feature = "slovak")
                || cfg!(feature = "spanish")
                || cfg!(feature = "vietnamese")
                || cfg!(feature = "yoruba")
            {
                mapping.insert("Óó", {
                    let mut languages = hashset!();
                    if cfg!(feature = "catalan") {
                        languages.insert(Language::from_str("Catalan").unwrap());
                    }
                    if cfg!(feature = "hungarian") {
                        languages.insert(Language::from_str("Hungarian").unwrap());
                    }
                    if cfg!(feature = "icelandic") {
                        languages.insert(Language::from_str("Icelandic").unwrap());
                    }
                    if cfg!(feature = "irish") {
                        languages.insert(Language::from_str("Irish").unwrap());
                    }
                    if cfg!(feature = "polish") {
                        languages.insert(Language::from_str("Polish").unwrap());
                    }
                    if cfg!(feature = "portuguese") {
                        languages.insert(Language::from_str("Portuguese").unwrap());
                    }
                    if cfg!(feature = "slovak") {
                        languages.insert(Language::from_str("Slovak").unwrap());
                    }
                    if cfg!(feature = "spanish") {
                        languages.insert(Language::from_str("Spanish").unwrap());
                    }
                    if cfg!(feature = "vietnamese") {
                        languages.insert(Language::from_str("Vietnamese").unwrap());
                    }
                    if cfg!(feature = "yoruba") {
                        languages.insert(Language::from_str("Yoruba").unwrap());
                    }
                    languages
                });
            }

            if cfg!(feature = "catalan")
                || cfg!(feature = "czech")
                || cfg!(feature = "icelandic")
                || cfg!(feature = "irish")
                || cfg!(feature = "hungarian")
                || cfg!(feature = "portuguese")
                || cfg!(feature = "slovak")
                || cfg!(feature = "spanish")
                || cfg!(feature = "vietnamese")
                || cfg!(feature = "yoruba")
            {
                mapping.insert("ÁáÍíÚú", {
                    let mut languages = hashset!();
                    if cfg!(feature = "catalan") {
                        languages.insert(Language::from_str("Catalan").unwrap());
                    }
                    if cfg!(feature = "czech") {
                        languages.insert(Language::from_str("Czech").unwrap());
                    }
                    if cfg!(feature = "icelandic") {
                        languages.insert(Language::from_str("Icelandic").unwrap());
                    }
                    if cfg!(feature = "irish") {
                        languages.insert(Language::from_str("Irish").unwrap());
                    }
                    if cfg!(feature = "hungarian") {
                        languages.insert(Language::from_str("Hungarian").unwrap());
                    }
                    if cfg!(feature = "portuguese") {
                        languages.insert(Language::from_str("Portuguese").unwrap());
                    }
                    if cfg!(feature = "slovak") {
                        languages.insert(Language::from_str("Slovak").unwrap());
                    }
                    if cfg!(feature = "spanish") {
                        languages.insert(Language::from_str("Spanish").unwrap());
                    }
                    if cfg!(feature = "vietnamese") {
                        languages.insert(Language::from_str("Vietnamese").unwrap());
                    }
                    if cfg!(feature = "yoruba") {
                        languages.insert(Language::from_str("Yoruba").unwrap());
                    }
                    languages
                });
            }

            if cfg!(feature = "catalan")
                || cfg!(feature = "czech")
                || cfg!(feature = "french")
                || cfg!(feature = "hungarian")
                || cfg!(feature = "icelandic")
                || cfg!(feature = "irish")
                || cfg!(feature = "italian")
                || cfg!(feature = "portuguese")
                || cfg!(feature = "slovak")
                || cfg!(feature = "spanish")
                || cfg!(feature = "vietnamese")
                || cfg!(feature = "yoruba")
            {
                mapping.insert("Éé", {
                    let mut languages = hashset!();
                    if cfg!(feature = "catalan") {
                        languages.insert(Language::from_str("Catalan").unwrap());
                    }
                    if cfg!(feature = "czech") {
                        languages.insert(Language::from_str("Czech").unwrap());
                    }
                    if cfg!(feature = "french") {
                        languages.insert(Language::from_str("French").unwrap());
                    }
                    if cfg!(feature = "hungarian") {
                        languages.insert(Language::from_str("Hungarian").unwrap());
                    }
                    if cfg!(feature = "icelandic") {
                        languages.insert(Language::from_str("Icelandic").unwrap());
                    }
                    if cfg!(feature = "irish") {
                        languages.insert(Language::from_str("Irish").unwrap());
                    }
                    if cfg!(feature = "italian") {
                        languages.insert(Language::from_str("Italian").unwrap());
                    }
                    if cfg!(feature = "portuguese") {
                        languages.insert(Language::from_str("Portuguese").unwrap());
                    }
                    if cfg!(feature = "slovak") {
                        languages.insert(Language::from_str("Slovak").unwrap());
                    }
                    if cfg!(feature = "spanish") {
                        languages.insert(Language::from_str("Spanish").unwrap());
                    }
                    if cfg!(feature = "vietnamese") {
                        languages.insert(Language::from_str("Vietnamese").unwrap());
                    }
                    if cfg!(feature = "yoruba") {
                        languages.insert(Language::from_str("Yoruba").unwrap());
                    }
                    languages
                });
            }
        }

        /*
        hashmap!(
            "Éé" => hashset!(
                Catalan, Czech, French, Hungarian, Icelandic, Irish, Italian, Portuguese, Slovak,
                Spanish, Vietnamese, Yoruba
            )
        )
        */
        mapping
    });
