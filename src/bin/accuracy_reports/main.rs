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

use crate::cld2::CLD2Detector;
use crate::dataframe::{
    dataframe_contains_detector, dataframe_contains_language, get_dataframe_language_name,
    sort_dataframe, update_dataframe_with_new_detector, update_dataframe_with_new_language,
    update_dataframe_with_new_probability,
};
use crate::detector_option::{DetectorOption, default_detector_options};
use crate::language_detection::LanguageDetection;
use crate::lingua_high_accuracy::LinguaHighAccuracyDetector;
use crate::lingua_low_accuracy::LinguaLowAccuracyDetector;
use crate::lingua_single_language::LinguaSingleLanguageDetector;
use crate::statistic::Category;
use crate::whatlang::WhatlangDetector;
use crate::whichlang::WhichlangDetector;
use clap::{Parser, ValueEnum};
use itertools::Itertools;
use lingua::Language;
use polars::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Instant;
use strum::IntoEnumIterator;

mod cld2;
mod dataframe;
mod detector_option;
mod language_detection;
mod lingua_high_accuracy;
mod lingua_low_accuracy;
mod lingua_single_language;
mod statistic;
mod testdata;
mod whatlang;
mod whichlang;

fn create_detector_instance(
    detector_option: DetectorOption,
    languages: &[Language],
) -> Option<Box<dyn LanguageDetection>> {
    match detector_option {
        DetectorOption::Cld2 => Some(Box::new(CLD2Detector::new(languages))),
        DetectorOption::LinguaHighAccuracy => {
            Some(Box::new(LinguaHighAccuracyDetector::new(languages)))
        }
        DetectorOption::LinguaLowAccuracy => {
            Some(Box::new(LinguaLowAccuracyDetector::new(languages)))
        }
        detector if detector.is_single_language_detector() => {
            let detector_name = detector.to_string();
            let name_parts = detector_name.split('-').collect_vec();
            let language_name = name_parts.get(1).unwrap();
            let language = <Language as FromStr>::from_str(language_name).unwrap();
            Some(Box::new(LinguaSingleLanguageDetector::new(
                language, languages,
            )))
        }
        DetectorOption::Whatlang => Some(Box::new(WhatlangDetector::new(languages))),
        DetectorOption::Whichlang => Some(Box::new(WhichlangDetector::new(languages))),
        _ => None,
    }
}

#[derive(Parser)]
struct Cli {
    #[arg(value_enum, short, long, num_args = 1.., default_values_t = default_detector_options())]
    detectors: Vec<DetectorOption>,

    #[arg(value_enum, short, long, num_args = 1.., default_values_t = Language::all())]
    languages: Vec<Language>,
}

fn main() {
    let total_start = Instant::now();
    let cli = Cli::parse();
    let mut detector_options = cli.detectors.iter().cloned().collect::<HashSet<_>>();
    let language_names = cli.languages.iter().map(|it| it.to_string()).collect_vec();
    let mut all_statistics = HashMap::new();

    if detector_options.contains(&DetectorOption::LinguaAllSingleLanguageDetectors) {
        detector_options.remove(&DetectorOption::LinguaAllSingleLanguageDetectors);

        for language_name in language_names.iter() {
            let detector_name = format!("lingua-{}-detector", language_name.to_lowercase());
            let detector_option = DetectorOption::from_str(&detector_name, true).unwrap();

            if !detector_options.contains(&detector_option) {
                detector_options.insert(detector_option);
            }
        }
    }

    for detector_option in detector_options.iter() {
        if let Some(detector) = create_detector_instance(*detector_option, &cli.languages) {
            let start = Instant::now();
            let mut statistics = detector.collect_statistics();
            detector.write_reports(&mut statistics);
            let stop = Instant::now();
            println!(
                "{detector_option} statistics written in {:.2} seconds\n",
                stop.duration_since(start).as_secs_f64()
            );
            all_statistics.insert(detector_option, statistics);
        }
    }

    println!("Updating aggregated reports...");
    let start = Instant::now();

    for category in Category::iter() {
        let report_file_path = format!(
            "accuracy-reports/{}",
            category.aggregated_report_file_name()
        );

        let mut dataframe = match CsvReadOptions::default()
            .try_into_reader_with_file_path(Some(PathBuf::from(&report_file_path)))
        {
            Ok(csv_reader) => csv_reader.finish().unwrap(),
            Err(_) => df!("language" => &language_names).unwrap(),
        };

        for detector_option in detector_options.iter() {
            let detector_name = detector_option.to_string();
            let statistics = all_statistics.get(detector_option).unwrap();
            let mut lazy_dataframe = dataframe.clone().lazy();

            if !dataframe_contains_detector(&dataframe, &detector_name) {
                lazy_dataframe = update_dataframe_with_new_detector(lazy_dataframe, &detector_name);
            }

            for stat in statistics {
                let df = stat.to_dataframe(category.clone());
                let language_name = get_dataframe_language_name(&df);

                lazy_dataframe = if dataframe_contains_language(&dataframe, &language_name) {
                    update_dataframe_with_new_probability(lazy_dataframe, df)
                } else {
                    update_dataframe_with_new_language(lazy_dataframe, df)
                };
            }
            dataframe = sort_dataframe(lazy_dataframe.collect().unwrap());
        }

        let mut aggregated_report_file =
            fs::File::create(&report_file_path).expect("CSV file could not be created");

        CsvWriter::new(&mut aggregated_report_file)
            .with_null_value("NaN".to_string())
            .finish(&mut dataframe)
            .expect("Data frame could not be written to CSV file");
    }

    let total_stop = Instant::now();
    let total_time = total_stop.duration_since(total_start);
    let total_minutes = total_time.as_secs() / 60;
    let total_seconds = total_time.as_secs() % 60;

    println!(
        "Aggregated reports updated in {:.2} seconds\n",
        total_stop.duration_since(start).as_secs_f64()
    );
    println!("All reports written in {total_minutes} minutes, {total_seconds} seconds");
}
