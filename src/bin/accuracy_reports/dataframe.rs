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

use itertools::Itertools;
use polars::datatypes::DataType;
use polars::frame::DataFrame;
use polars::prelude::{IntoLazy, LazyFrame, UnionArgs, col, concat, lit, when};

pub(crate) fn get_dataframe_language_name(df: &DataFrame) -> String {
    df.column("language")
        .unwrap()
        .get(0)
        .unwrap()
        .get_str()
        .unwrap()
        .to_string()
}

pub(crate) fn get_dataframe_detector_name(df: &DataFrame) -> String {
    df.get_column_names().get(1).unwrap().to_string()
}

pub(crate) fn get_dataframe_probability(df: &DataFrame) -> f64 {
    df.get_columns()
        .get(1)
        .unwrap()
        .f64()
        .unwrap()
        .get(0)
        .unwrap()
}

pub(crate) fn dataframe_contains_language(df: &DataFrame, language_name: &str) -> bool {
    if df.column("language").is_err() {
        return false;
    }
    let column_name = "contains_language";
    let contains_language_df = df
        .clone()
        .lazy()
        .select([col("language")
            .eq(lit(language_name))
            .any(true)
            .alias(column_name)])
        .collect()
        .unwrap();

    contains_language_df
        .column(column_name)
        .unwrap()
        .bool()
        .unwrap()
        .get(0)
        .unwrap()
}

pub(crate) fn dataframe_contains_detector(df: &DataFrame, detector_name: &str) -> bool {
    df.column(detector_name).is_ok()
}

pub(crate) fn update_dataframe_with_new_language(main_df: LazyFrame, df: DataFrame) -> LazyFrame {
    let union_args = UnionArgs {
        diagonal: true,
        ..Default::default()
    };
    concat([main_df, df.lazy()], union_args).unwrap()
}

pub(crate) fn update_dataframe_with_new_detector(
    main_df: LazyFrame,
    detector_name: &str,
) -> LazyFrame {
    main_df.with_column(lit("NaN").cast(DataType::Float64).alias(detector_name))
}

pub(crate) fn update_dataframe_with_new_probability(
    main_df: LazyFrame,
    df: DataFrame,
) -> LazyFrame {
    let language_name = get_dataframe_language_name(&df);
    let detector_name = get_dataframe_detector_name(&df);
    let probability = get_dataframe_probability(&df);

    main_df.with_column(
        when(col("language").eq(lit(language_name)))
            .then(lit(probability))
            .otherwise(col(&detector_name))
            .alias(detector_name),
    )
}

pub(crate) fn sort_dataframe(df: DataFrame) -> DataFrame {
    let sorted_columns = &mut df.get_column_names_str()[1..]
        .iter()
        .sorted()
        .map(|&it| col(it))
        .collect_vec();

    sorted_columns.insert(0, col("language"));

    df.lazy()
        .select(sorted_columns)
        .sort(["language"], Default::default())
        .collect()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use lingua::Language;
    use polars::df;
    use rstest::*;

    // ##############################
    // FIXTURES
    // ##############################

    #[fixture]
    fn main_dataframe() -> DataFrame {
        df!(
            "language" => [
                Language::English.to_string(),
                Language::German.to_string(),
                Language::Yoruba.to_string()
            ],
            "lingua-high-accuracy" => [73.56271, 66.49882, 12.16833],
            "whatlang" => [43.18733, 27.94481, 35.62811]
        )
        .unwrap()
    }

    #[fixture]
    fn dataframe_with_unknown_language() -> DataFrame {
        df!(
            "language" => [Language::Danish.to_string()],
            "lingua-high-accuracy" => [32.77125]
        )
        .unwrap()
    }

    #[fixture]
    fn dataframe_with_unknown_detector() -> DataFrame {
        df!(
            "language" => [Language::German.to_string()],
            "cld2" => [17.39446]
        )
        .unwrap()
    }

    #[fixture]
    fn dataframe_with_new_probability() -> DataFrame {
        df!(
            "language" => [Language::English.to_string()],
            "lingua-high-accuracy" => [12.34567]
        )
        .unwrap()
    }

    #[fixture]
    fn dataframe_with_unsorted_columns() -> DataFrame {
        df!(
            "language" => [
                Language::Yoruba.to_string(),
                Language::German.to_string(),
                Language::English.to_string(),
            ],
            "cld2" => [92.34567, 55.23456, 53.12345],
            "whatlang" => [35.62811, 27.94481, 43.18733],
            "lingua-high-accuracy" => [12.16833, 66.49882, 73.56271],
        )
        .unwrap()
    }

    // ##############################
    // TESTS
    // ##############################

    #[rstest]
    fn test_get_dataframe_language_name(dataframe_with_unknown_language: DataFrame) {
        assert_eq!(
            get_dataframe_language_name(&dataframe_with_unknown_language),
            "Danish"
        );
    }

    #[rstest]
    fn test_get_dataframe_detector_name(dataframe_with_unknown_detector: DataFrame) {
        assert_eq!(
            get_dataframe_detector_name(&dataframe_with_unknown_detector),
            "cld2"
        );
    }

    #[rstest]
    fn test_get_dataframe_probability(dataframe_with_new_probability: DataFrame) {
        assert_eq!(
            get_dataframe_probability(&dataframe_with_new_probability),
            12.34567
        );
    }

    #[rstest]
    fn test_dataframe_contains_language(main_dataframe: DataFrame) {
        assert!(dataframe_contains_language(&main_dataframe, "English"));
        assert!(dataframe_contains_language(&main_dataframe, "German"));
        assert!(dataframe_contains_language(&main_dataframe, "Yoruba"));
        assert!(!dataframe_contains_language(&main_dataframe, "Hindi"));
    }

    #[rstest]
    fn test_dataframe_contains_detector(main_dataframe: DataFrame) {
        assert!(dataframe_contains_detector(
            &main_dataframe,
            "lingua-high-accuracy"
        ));
        assert!(dataframe_contains_detector(&main_dataframe, "whatlang"));
        assert!(!dataframe_contains_detector(&main_dataframe, "cld2"));
    }

    #[rstest]
    fn test_update_dataframe_with_new_probability(
        main_dataframe: DataFrame,
        dataframe_with_new_probability: DataFrame,
    ) {
        let result = update_dataframe_with_new_probability(
            main_dataframe.lazy(),
            dataframe_with_new_probability,
        )
        .collect()
        .unwrap();

        assert_eq!(
            result,
            df!(
                "language" => [
                    Language::English.to_string(),
                    Language::German.to_string(),
                    Language::Yoruba.to_string()
                ],
                "lingua-high-accuracy" => [12.34567, 66.49882, 12.16833],
                "whatlang" => [43.18733, 27.94481, 35.62811]
            )
            .unwrap()
        );
    }

    #[rstest]
    fn test_sort_dataframe_columns(dataframe_with_unsorted_columns: DataFrame) {
        assert_eq!(
            sort_dataframe(dataframe_with_unsorted_columns),
            df!(
                "language" => [
                    Language::English.to_string(),
                    Language::German.to_string(),
                    Language::Yoruba.to_string()
                ],
                "cld2" => [53.12345, 55.23456, 92.34567],
                "lingua-high-accuracy" => [73.56271, 66.49882, 12.16833],
                "whatlang" => [43.18733, 27.94481, 35.62811],
            )
            .unwrap()
        )
    }
}
