<div align="center">

  ![lingua](https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/logo.png)

  [![rust build status](https://github.com/pemistahl/lingua-rs/actions/workflows/rust-build.yml/badge.svg)](https://github.com/pemistahl/lingua-rs/actions/workflows/rust-build.yml)
  [![python build status](https://github.com/pemistahl/lingua-rs/actions/workflows/python-build.yml/badge.svg)](https://github.com/pemistahl/lingua-rs/actions/workflows/python-build.yml)
  [![docs.rs](https://docs.rs/lingua/badge.svg)](https://docs.rs/lingua)
  [![codecov](https://codecov.io/gh/pemistahl/lingua-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/pemistahl/lingua-rs)
  [![supported languages](https://img.shields.io/badge/supported%20languages-75-green.svg)](#3-which-languages-are-supported)
  [![dependency status](https://deps.rs/crate/lingua/1.6.2/status.svg)](https://deps.rs/crate/lingua/1.6.2)
  [![downloads](https://img.shields.io/crates/d/lingua.svg)](https://crates.io/crates/lingua)
  [![crates.io](https://img.shields.io/crates/v/lingua.svg)](https://crates.io/crates/lingua)
  [![lib.rs](https://img.shields.io/badge/lib.rs-v1.6.2-blue)](https://lib.rs/crates/lingua)
  [![license](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
</div>

<br>

## 1. What does this library do?

Its task is simple: It tells you which language some text is written in.
This is very useful as a preprocessing step for linguistic data in natural language
processing applications such as text classification and spell checking.
Other use cases, for instance, might include routing e-mails to the right geographically
located customer service department, based on the e-mails' languages.

## 2. Why does this library exist?

Language detection is often done as part of large machine learning frameworks or natural
language processing applications. In cases where you don't need the full-fledged
functionality of those systems or don't want to learn the ropes of those,
a small flexible library comes in handy.

So far, other comprehensive open source libraries in the Rust ecosystem for
this task are [*CLD2*](https://github.com/emk/rust-cld2),
[*Whatlang*](https://github.com/greyblake/whatlang-rs) and 
[*Whichlang*](https://github.com/quickwit-oss/whichlang).
Unfortunately, most of them have two major drawbacks:

1. Detection only works with quite lengthy text fragments. For very short text snippets
such as Twitter messages, they do not provide adequate results.
2. The more languages take part in the decision process, the less accurate are the
detection results.

*Lingua* aims at eliminating these problems. She nearly does not need any configuration and
yields pretty accurate results on both long and short text, even on single words and phrases.
She draws on both rule-based and statistical methods but does not use any dictionaries of words.
She does not need a connection to any external API or service either.
Once the library has been downloaded, it can be used completely offline.

## 3. Which languages are supported?

Compared to other language detection libraries, *Lingua's* focus is on *quality over quantity*, that is, 
getting detection right for a small set of languages first before adding new ones. 
Currently, the following 75 languages are supported:

- A
  - Afrikaans
  - Albanian
  - Arabic
  - Armenian
  - Azerbaijani
- B
  - Basque
  - Belarusian
  - Bengali
  - Norwegian Bokmal
  - Bosnian
  - Bulgarian
- C
  - Catalan
  - Chinese
  - Croatian
  - Czech
- D
  - Danish
  - Dutch
- E
  - English
  - Esperanto
  - Estonian
- F
  - Finnish
  - French
- G
  - Ganda
  - Georgian
  - German
  - Greek
  - Gujarati
- H
  - Hebrew
  - Hindi
  - Hungarian
- I
  - Icelandic
  - Indonesian
  - Irish
  - Italian
- J
  - Japanese
- K
  - Kazakh
  - Korean
- L
  - Latin
  - Latvian
  - Lithuanian
- M
  - Macedonian
  - Malay
  - Maori
  - Marathi
  - Mongolian
- N
  - Norwegian Nynorsk
- P
  - Persian
  - Polish
  - Portuguese
  - Punjabi
- R
  - Romanian
  - Russian
- S
  - Serbian
  - Shona
  - Slovak
  - Slovene
  - Somali
  - Sotho
  - Spanish
  - Swahili
  - Swedish
- T
  - Tagalog
  - Tamil
  - Telugu
  - Thai
  - Tsonga
  - Tswana
  - Turkish
- U
  - Ukrainian
  - Urdu
- V
  - Vietnamese
- W
  - Welsh
- X
  - Xhosa
- Y
  - Yoruba
- Z
  - Zulu
  
## 4. How accurate is it?

*Lingua* is able to report accuracy statistics for some bundled test data available for each
supported language. The test data for each language is split into three parts:

1. a list of single words with a minimum length of 5 characters
2. a list of word pairs with a minimum length of 10 characters
3. a list of complete grammatical sentences of various lengths

Both the language models and the test data have been created from separate documents of the
[Wortschatz corpora](https://wortschatz.uni-leipzig.de) offered by Leipzig University, Germany.
Data crawled from various news websites have been used for training, each corpus comprising one
million sentences. For testing, corpora made of arbitrarily chosen websites have been used,
each comprising ten thousand sentences. From each test corpus, a random unsorted subset of
1000 single words, 1000 word pairs and 1000 sentences has been extracted, respectively.

Given the generated test data, I have compared the detection results of *Lingua*, *CLD2*, *Whatlang*
and *Whichlang* running over the data of *Lingua's* supported 75 languages. Languages that are not supported
by the other libraries are simply ignored for the respective library during the detection process.

Each of the following sections contains four plots. The bar plots show the detailed accuracy
results for each supported language. The box plots illustrate the distributions of the
accuracy values for each classifier. The boxes themselves represent the areas which the
middle 50 % of data lie within. Within the colored boxes, the horizontal lines mark the
median of the distributions.

The first two plots in each section show the results for all supported languages
in each classifier, respectively. The last two plots are restricted to the common subset
of currently 16 languages that is supported by all compared classifiers. This distinction
makes sense because the first box plot creates the impression that Whichlang is the most
accurate classifier, but it is not. Whichlang supports only 16 languages whereas Lingua
supports 75 languages. For the second box plot, the supported languages in Whatlang and
Lingua have been restricted to those 16 languages supported by Whichlang. This provides
for a more accurate comparison and shows that overall, Lingua is the most accurate
language detection library in this comparison.

### 4.1 Single word detection

#### 4.1.1 All languages

<br/>

<img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/plots/boxplot-single-words-all-languages.png" alt="Single Word Detection Performance" />

<br/>

<details>
    <summary>Bar plot</summary>
    <img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/plots/barplot-single-words-all-languages.png" alt="Single Word Detection Performance" />
</details>

<br/><br/>

#### 4.1.2 Common languages

<br/>

<img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/plots/boxplot-single-words-common-languages.png" alt="Single Word Detection Performance" />

<br/>

<details>
    <summary>Bar plot</summary>
    <img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/plots/barplot-single-words-common-languages.png" alt="Single Word Detection Performance" />
</details>

<br/><br/>

### 4.2 Word pair detection

#### 4.2.1 All languages

<br/>

<img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/plots/boxplot-word-pairs-all-languages.png" alt="Word Pair Detection Performance" />

<br/>

<details>
    <summary>Bar plot</summary>
    <img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/plots/barplot-word-pairs-all-languages.png" alt="Word Pair Detection Performance" />
</details>

<br/><br/>

#### 4.2.2 Common languages

<br/>

<img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/plots/boxplot-word-pairs-common-languages.png" alt="Word Pair Detection Performance" />

<br/>

<details>
    <summary>Bar plot</summary>
    <img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/plots/barplot-word-pairs-common-languages.png" alt="Word Pair Detection Performance" />
</details>

<br/><br/>

### 4.3 Sentence detection

#### 4.3.1 All languages

<br/>

<img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/plots/boxplot-sentences-all-languages.png" alt="Sentence Detection Performance" />

<br/>

<details>
    <summary>Bar plot</summary>
    <img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/plots/barplot-sentences-all-languages.png" alt="Sentence Detection Performance" />
</details>

<br/><br/>

#### 4.3.2 Common languages

<br/>

<img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/plots/boxplot-sentences-common-languages.png" alt="Sentence Detection Performance" />

<br/>

<details>
    <summary>Bar plot</summary>
    <img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/plots/barplot-sentences-common-languages.png" alt="Sentence Detection Performance" />
</details>

<br/><br/>

### 4.4 Average detection

#### 4.4.1 All languages

<br/>

<img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/plots/boxplot-average-all-languages.png" alt="Average Detection Performance" />

<br/>

<details>
    <summary>Bar plot</summary>
    <img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/plots/barplot-average-all-languages.png" alt="Average Detection Performance" />
</details>

<br/><br/>

#### 4.4.2 Common languages

<br/>

<img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/plots/boxplot-average-common-languages.png" alt="Average Detection Performance" />

<br/>

<details>
    <summary>Bar plot</summary>
    <img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/plots/barplot-average-common-languages.png" alt="Average Detection Performance" />
</details>

<br/><br/>

### 4.5 Mean, median and standard deviation

The tables below show detailed statistics for each language and classifier
including mean, median and standard deviation.

#### 4.5.1 All languages

<details>
  <summary>Open table</summary>
  <table>
    <tr>
        <th>Language</th>
        <th colspan="5">Average</th>
        <th colspan="5">Single Words</th>
        <th colspan="5">Word Pairs</th>
        <th colspan="5">Sentences</th>
    </tr>
    <tr>
        <th></th>
        <th>Lingua<br>(high accuracy mode)</th>
        <th>Lingua<br>(low accuracy mode)</th>
        <th>Whichlang</th>
        <th>Whatlang</th>
        <th>&nbsp;&nbsp;CLD2&nbsp;&nbsp;</th>
        <th>Lingua<br>(high accuracy mode)</th>
        <th>Lingua<br>(low accuracy mode)</th>
        <th>Whichlang</th>
        <th>Whatlang</th>
        <th>&nbsp;&nbsp;CLD2&nbsp;&nbsp;</th>
        <th>Lingua<br>(high accuracy mode)</th>
        <th>Lingua<br>(low accuracy mode)</th>
        <th>Whichlang</th>
        <th>Whatlang</th>
        <th>&nbsp;&nbsp;CLD2&nbsp;&nbsp;</th>
        <th>Lingua<br>(high accuracy mode)</th>
        <th>Lingua<br>(low accuracy mode)</th>
        <th>Whichlang</th>
        <th>Whatlang</th>
        <th>&nbsp;&nbsp;CLD2&nbsp;&nbsp;</th>
    </tr>
    	<tr>
		<td>Afrikaans</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 78</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 64</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 51</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 55</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 58</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 37</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 22</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 12</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 80</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 62</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 41</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 56</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
	</tr>
	<tr>
		<td>Albanian</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 79</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 64</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 68</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 54</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 18</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 86</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 76</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Arabic</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 66</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 88</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 88</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 19</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 81</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Armenian</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
	</tr>
	<tr>
		<td>Azerbaijani</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 89</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 82</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 79</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 71</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 77</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 71</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 61</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 34</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 78</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 81</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 81</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Basque</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 83</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 74</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 61</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 71</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 55</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 22</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 76</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 69</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 91</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 91</td>
	</tr>
	<tr>
		<td>Belarusian</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 91</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 91</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 75</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 91</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 80</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 81</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 42</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
	</tr>
	<tr>
		<td>Bengali</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 62</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 19</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 69</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Bokmal</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 58</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 49</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 49</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 38</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 27</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 24</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 58</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 47</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 44</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 76</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 74</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 80</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
	</tr>
	<tr>
		<td>Bosnian</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 34</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 29</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 18</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 29</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 22</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 4</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 34</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 28</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 15</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 40</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 36</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 36</td>
	</tr>
	<tr>
		<td>Bulgarian</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 86</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 77</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 69</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 65</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 56</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 44</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 31</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 91</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 80</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 67</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 72</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
	</tr>
	<tr>
		<td>Catalan</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 58</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 51</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 37</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 50</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 33</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 29</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 4</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 73</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 60</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 45</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 29</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 86</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 81</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 80</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 79</td>
	</tr>
	<tr>
		<td>Chinese</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 33</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 2</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
	</tr>
	<tr>
		<td>Croatian</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 72</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 59</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 61</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 51</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 53</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 36</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 34</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 33</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 74</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 57</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 54</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 46</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 90</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 85</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 72</td>
	</tr>
	<tr>
		<td>Czech</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 80</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 63</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 73</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 65</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 54</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 42</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 50</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 84</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 71</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 66</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 79</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 91</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 82</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 90</td>
	</tr>
	<tr>
		<td>Danish</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 81</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 53</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 59</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 61</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 45</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 31</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 26</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 83</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 45</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 56</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 84</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
	</tr>
	<tr>
		<td>Dutch</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 77</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 63</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 73</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 47</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 47</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 55</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 35</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 53</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 22</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 10</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 80</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 61</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 37</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 41</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 82</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 90</td>
	</tr>
	<tr>
		<td>English</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 80</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 62</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 66</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 49</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 55</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 54</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 29</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 34</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 17</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 11</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 88</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 62</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 65</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 35</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 55</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Esperanto</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 83</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 65</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 58</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 50</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 67</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 43</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 32</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 7</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 85</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 60</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 53</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 45</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 90</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
	</tr>
	<tr>
		<td>Estonian</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 91</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 83</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 67</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 65</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 79</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 62</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 43</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 23</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 62</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 73</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Finnish</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 90</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 76</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 77</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 90</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 77</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 53</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 43</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 78</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 89</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>French</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 89</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 77</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 68</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 63</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 51</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 74</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 51</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 36</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 35</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 12</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 82</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 59</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 47</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
	</tr>
	<tr>
		<td>Ganda</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 91</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 83</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 61</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 79</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 64</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 23</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 86</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 62</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
	</tr>
	<tr>
		<td>Georgian</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
	</tr>
	<tr>
		<td>German</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 89</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 79</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 79</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 69</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 63</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 73</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 56</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 55</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 44</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 26</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 83</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 82</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 66</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 66</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
	</tr>
	<tr>
		<td>Greek</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
	</tr>
	<tr>
		<td>Gujarati</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
	</tr>
	<tr>
		<td>Hebrew</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
	</tr>
	<tr>
		<td>Hindi</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 73</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 32</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 68</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 76</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 60</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 11</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 54</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 55</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 64</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 19</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 54</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 75</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 67</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Hungarian</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 90</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 72</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 75</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 86</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 77</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 50</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 40</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 85</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Icelandic</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 65</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 82</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 71</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 25</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 73</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Indonesian</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 60</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 47</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 69</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 62</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 39</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 25</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 42</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 36</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 60</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 45</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 69</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 62</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 82</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 71</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 88</td>
	</tr>
	<tr>
		<td>Irish</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 90</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 85</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 66</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 81</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 29</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 90</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 77</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
	</tr>
	<tr>
		<td>Italian</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 86</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 71</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 85</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 56</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 44</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 69</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 41</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 68</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 25</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 6</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 91</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 73</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 48</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 32</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
	</tr>
	<tr>
		<td>Japanese</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 33</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
	</tr>
	<tr>
		<td>Kazakh</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 76</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 89</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 43</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Korean</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Latin</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 73</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 55</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 46</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 72</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 49</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 36</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 8</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 75</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 51</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 42</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 79</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 88</td>
	</tr>
	<tr>
		<td>Latvian</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 85</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 72</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 84</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 74</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 33</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 90</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 86</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 84</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
	</tr>
	<tr>
		<td>Lithuanian</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 79</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 86</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 75</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 59</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 29</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 88</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 81</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 81</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Macedonian</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 83</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 72</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 66</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 60</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 65</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 51</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 43</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 27</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 86</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 60</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 69</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 83</td>
	</tr>
	<tr>
		<td>Malay</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 30</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 30</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 17</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 26</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 21</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 9</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 38</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 35</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 21</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 28</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 35</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 23</td>
	</tr>
	<tr>
		<td>Maori</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 83</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 60</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 84</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 63</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 12</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 71</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
	</tr>
	<tr>
		<td>Marathi</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 84</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 38</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 78</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 83</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 73</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 15</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 59</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 65</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 84</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 29</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 80</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 86</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 71</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
	</tr>
	<tr>
		<td>Mongolian</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 78</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 43</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Nynorsk</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 65</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 51</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 54</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 40</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 25</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 18</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 65</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 49</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 50</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 90</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 81</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
	</tr>
	<tr>
		<td>Persian</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 90</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 80</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 72</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 61</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 77</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 61</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 49</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 12</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 80</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 68</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 72</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Polish</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 89</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 81</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 74</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 85</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 76</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 63</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 37</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 81</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 86</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Portuguese</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 81</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 68</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 60</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 53</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 59</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 41</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 42</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 30</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 19</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 85</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 69</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 52</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 47</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
	</tr>
	<tr>
		<td>Punjabi</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
	</tr>
	<tr>
		<td>Romanian</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 86</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 72</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 66</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 53</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 68</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 49</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 45</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 11</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 91</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 73</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 62</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 53</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 90</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
	</tr>
	<tr>
		<td>Russian</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 89</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 78</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 67</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 60</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 76</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 59</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 51</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 26</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 83</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 66</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 67</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 84</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 86</td>
	</tr>
	<tr>
		<td>Serbian</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 77</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 66</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 68</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 73</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 61</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 46</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 29</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 90</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 79</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 63</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 77</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 91</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 89</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Shona</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 91</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 80</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 76</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 64</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 77</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 56</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 55</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 23</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 86</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 76</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 71</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Slovak</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 84</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 75</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 64</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 71</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 64</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 49</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 40</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 37</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 90</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 78</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 60</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 75</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Slovene</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 82</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 66</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 52</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 47</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 61</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 39</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 29</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 8</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 86</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 67</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 43</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 42</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 83</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
	</tr>
	<tr>
		<td>Somali</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 84</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 69</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 81</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 64</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 26</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 90</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 83</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Sotho</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 85</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 71</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 53</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 66</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 42</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 12</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 90</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 74</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 54</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
	</tr>
	<tr>
		<td>Spanish</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 69</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 56</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 62</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 47</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 43</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 43</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 25</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 35</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 18</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 12</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 68</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 48</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 54</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 32</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 33</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 91</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 84</td>
	</tr>
	<tr>
		<td>Swahili</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 80</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 69</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 57</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 60</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 43</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 15</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 84</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 68</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 58</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
	</tr>
	<tr>
		<td>Swedish</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 83</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 72</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 71</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 61</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 53</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 64</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 45</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 47</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 34</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 14</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 88</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 75</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 57</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 52</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
	</tr>
	<tr>
		<td>Tagalog</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 77</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 66</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 50</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 52</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 35</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 9</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 83</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 66</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 44</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
	</tr>
	<tr>
		<td>Tamil</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
	</tr>
	<tr>
		<td>Telugu</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
	</tr>
	<tr>
		<td>Thai</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Tsonga</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 84</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 72</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 61</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 66</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 46</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 19</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 89</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 73</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 67</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
	</tr>
	<tr>
		<td>Tswana</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 84</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 71</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 56</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 65</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 44</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 17</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 88</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 73</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 57</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
	</tr>
	<tr>
		<td>Turkish</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 63</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 66</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 83</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 83</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 39</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 30</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 91</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 59</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 71</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 91</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
	</tr>
	<tr>
		<td>Ukrainian</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 86</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 85</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 77</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 84</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 74</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 69</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 45</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 91</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 88</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
	</tr>
	<tr>
		<td>Urdu</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 90</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 78</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 68</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 61</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 80</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 64</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 45</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 8</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 78</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 63</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 75</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Vietnamese</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 90</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 85</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 91</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 63</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 78</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 75</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 64</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 75</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 91</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 89</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Welsh</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 91</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 82</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 72</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 78</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 60</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 34</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 85</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
	</tr>
	<tr>
		<td>Xhosa</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 82</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 68</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 71</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 63</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 44</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 45</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 85</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 67</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
	</tr>
	<tr>
		<td>Yoruba</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 74</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 62</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 36</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 50</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 33</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 1</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 76</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 60</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 22</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
	</tr>
	<tr>
		<td>Zulu</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 80</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 76</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 54</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 62</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 45</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 53</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 18</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 83</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 71</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 77</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 51</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
	</tr>
	<tr>
		<td colspan="20"></td>
	</tr>
	<tr>
		<td><strong>Mean</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> <strong>86</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> <strong>77</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> <strong>84</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> <strong>74</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> <strong>64</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> <strong>74</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> <strong>60</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> <strong>69</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> <strong>57</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> <strong>34</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> <strong>89</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> <strong>78</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> <strong>84</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> <strong>71</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> <strong>68</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> <strong>96</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> <strong>93</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> <strong>99</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> <strong>94</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> <strong>94</strong></td>
	</tr>
	<tr>
		<td colspan="20"></td>
	</tr>
	<tr>
		<td>Median</td>
		<td>89.0</td>
		<td>78.0</td>
		<td>85.0</td>
		<td>69.0</td>
		<td>63.0</td>
		<td>74.0</td>
		<td>56.0</td>
		<td>66.0</td>
		<td>49.5</td>
		<td>26.0</td>
		<td>93.0</td>
		<td>80.0</td>
		<td>89.0</td>
		<td>67.5</td>
		<td>71.0</td>
		<td>99.0</td>
		<td>96.0</td>
		<td>99.0</td>
		<td>97.0</td>
		<td>98.0</td>
	</tr>
	<tr>
		<td>Standard Deviation</td>
		<td>13.17</td>
		<td>17.39</td>
		<td>14.21</td>
		<td>17.27</td>
		<td>18.54</td>
		<td>18.54</td>
		<td>25.09</td>
		<td>26.6</td>
		<td>27.23</td>
		<td>28.81</td>
		<td>13.25</td>
		<td>19.12</td>
		<td>15.66</td>
		<td>21.07</td>
		<td>22.74</td>
		<td>11.06</td>
		<td>11.84</td>
		<td>0.85</td>
		<td>6.04</td>
		<td>12.16</td>
	</tr>
  </table>
</details>

#### 4.5.2 Common languages

<details>
  <summary>Open table</summary>
  <table>
    <tr>
        <th>Language</th>
        <th colspan="5">Average</th>
        <th colspan="5">Single Words</th>
        <th colspan="5">Word Pairs</th>
        <th colspan="5">Sentences</th>
    </tr>
    <tr>
        <th></th>
        <th>Lingua<br>(high accuracy mode)</th>
        <th>Lingua<br>(low accuracy mode)</th>
        <th>Whichlang</th>
        <th>Whatlang</th>
        <th>&nbsp;&nbsp;CLD2&nbsp;&nbsp;</th>
        <th>Lingua<br>(high accuracy mode)</th>
        <th>Lingua<br>(low accuracy mode)</th>
        <th>Whichlang</th>
        <th>Whatlang</th>
        <th>&nbsp;&nbsp;CLD2&nbsp;&nbsp;</th>
        <th>Lingua<br>(high accuracy mode)</th>
        <th>Lingua<br>(low accuracy mode)</th>
        <th>Whichlang</th>
        <th>Whatlang</th>
        <th>&nbsp;&nbsp;CLD2&nbsp;&nbsp;</th>
        <th>Lingua<br>(high accuracy mode)</th>
        <th>Lingua<br>(low accuracy mode)</th>
        <th>Whichlang</th>
        <th>Whatlang</th>
        <th>&nbsp;&nbsp;CLD2&nbsp;&nbsp;</th>
    </tr>
    	<tr>
		<td>Arabic</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 66</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 19</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 81</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Chinese</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 33</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 2</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
	</tr>
	<tr>
		<td>Dutch</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 80</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 73</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 63</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 47</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 73</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 60</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 53</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 39</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 10</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 89</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 81</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 54</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 41</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 90</td>
	</tr>
	<tr>
		<td>English</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 76</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 66</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 57</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 55</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 52</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 34</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 29</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 11</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 77</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 65</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 47</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 55</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>French</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 84</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 68</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 74</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 51</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 82</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 65</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 36</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 50</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 12</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 88</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 73</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 47</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
	</tr>
	<tr>
		<td>German</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 86</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 79</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 76</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 63</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 82</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 55</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 54</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 26</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 89</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 82</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 75</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 66</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
	</tr>
	<tr>
		<td>Hindi</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 76</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 55</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 75</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Italian</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 82</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 85</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 68</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 44</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 82</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 63</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 68</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 45</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 6</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 84</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 87</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 63</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 32</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
	</tr>
	<tr>
		<td>Japanese</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 33</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
	</tr>
	<tr>
		<td>Korean</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td>Portuguese</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 85</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 76</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 67</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 53</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 69</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 55</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 42</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 42</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 19</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 88</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 77</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 62</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 47</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
	</tr>
	<tr>
		<td>Russian</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 60</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 26</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 67</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 86</td>
	</tr>
	<tr>
		<td>Spanish</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 78</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 69</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 62</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 54</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 43</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 59</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 46</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 35</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 29</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 12</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 78</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 65</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 54</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 41</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 33</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 84</td>
	</tr>
	<tr>
		<td>Swedish</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 86</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 71</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 74</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 53</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 83</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 69</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 47</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 51</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/red.png"> 14</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 90</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 70</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 74</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> 52</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 93</td>
	</tr>
	<tr>
		<td>Turkish</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 95</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 81</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 66</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 89</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 83</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 66</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> 30</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 78</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 71</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 100</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 97</td>
	</tr>
	<tr>
		<td>Vietnamese</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 94</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 91</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 85</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 63</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 86</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 83</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 64</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> 79</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/grey.png"> -</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 96</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 92</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 91</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 98</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 89</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> 99</td>
	</tr>
	<tr>
		<td colspan="20"></td>
	</tr>
	<tr>
		<td><strong>Mean</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> <strong>93</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> <strong>89</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> <strong>84</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> <strong>81</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> <strong>57</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> <strong>86</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> <strong>78</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> <strong>69</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> <strong>68</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/orange.png"> <strong>26</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> <strong>95</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> <strong>90</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> <strong>84</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/lightgreen.png"> <strong>79</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/yellow.png"> <strong>57</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> <strong>99</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> <strong>99</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> <strong>99</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> <strong>98</strong></td>
		<td><img src="https://raw.githubusercontent.com/pemistahl/lingua-rs/main/images/green.png"> <strong>95</strong></td>
	</tr>
	<tr>
		<td colspan="20"></td>
	</tr>
	<tr>
		<td>Median</td>
		<td>93.5</td>
		<td>88.5</td>
		<td>85.0</td>
		<td>78.5</td>
		<td>54.0</td>
		<td>84.5</td>
		<td>76.5</td>
		<td>66.0</td>
		<td>60.0</td>
		<td>19.0</td>
		<td>96.0</td>
		<td>91.0</td>
		<td>89.0</td>
		<td>76.5</td>
		<td>55.0</td>
		<td>99.0</td>
		<td>99.0</td>
		<td>99.0</td>
		<td>98.0</td>
		<td>97.5</td>
	</tr>
	<tr>
		<td>Standard Deviation</td>
		<td>6.56</td>
		<td>10.44</td>
		<td>14.21</td>
		<td>16.91</td>
		<td>16.44</td>
		<td>13.51</td>
		<td>20.23</td>
		<td>26.6</td>
		<td>28.33</td>
		<td>25.57</td>
		<td>6.03</td>
		<td>10.76</td>
		<td>15.66</td>
		<td>21.17</td>
		<td>25.16</td>
		<td>0.58</td>
		<td>1.31</td>
		<td>0.85</td>
		<td>1.98</td>
		<td>4.95</td>
	</tr>
  </table>
</details>

## 5. How fast is it?

Various benchmarks for all classifiers can be run via:

    cargo bench --features benchmark

The benchmarks measure the processing time for classifying 2,000 sentences available 
in the common 16 languages that are supported by each language detector. The results 
below have been produced on an iMac 3.6 Ghz 8-Core Intel Core i9 with 40 GB RAM.
Whichlang has the shortest processing time, Lingua the longest.

| Detector                                      | Single Thread | Multiple Threads |
|-----------------------------------------------|--------------:|-----------------:|
| Whichlang                                     |      2.046 ms |        351.03 µs |
| CLD 2                                         |      8.923 ms |          2.05 ms |
| Whatlang (common languages)                   |      47.74 ms |          5.61 ms |
| Whatlang (all languages)                      |     113.08 ms |         12.99 ms |
| Lingua (low accuracy mode, common languages)  |     180.54 ms |         24.55 ms |
| Lingua (high accuracy mode, common languages) |     333.31 ms |         37.35 ms |
| Lingua (low accuracy mode, all languages)     |     373.15 ms |         48.18 ms |
| Lingua (high accuracy mode, all languages)    |     622.00 ms |         96.65 ms |

The accuracy reporter script measures the time each language detector needs
to classify 3000 input texts for each of the supported 75 languages.

| Detector                                     |             Time |
|----------------------------------------------|-----------------:|
| Whichlang                                    |         0.13 sec |
| CLD 2                                        |         1.67 sec |
| Lingua (low accuracy mode, multi-threaded)   |         3.11 sec |
| Whatlang                                     |         9.82 sec |
| Lingua (high accuracy mode, multi-threaded)  |        10.36 sec |
| Lingua (low accuracy mode, single-threaded)  |        21.06 sec |
| Lingua (high accuracy mode, single-threaded) |        57.15 sec |

## 6. Why is it better than other libraries?

Every language detector uses a probabilistic [n-gram](https://en.wikipedia.org/wiki/N-gram) model trained on the
character distribution in some training corpus. Most libraries only use n-grams of size 3 (trigrams) which is
satisfactory for detecting the language of longer text fragments consisting of multiple sentences. For short
phrases or single words, however, trigrams are not enough. The shorter the input text is, the less n-grams are
available. The probabilities estimated from such few n-grams are not reliable. This is why *Lingua* makes use
of n-grams of sizes 1 up to 5 which results in much more accurate prediction of the correct language.

A second important difference is that *Lingua* does not only use such a statistical model, but also a rule-based
engine. This engine first determines the alphabet of the input text and searches for characters which are unique
in one or more languages. If exactly one language can be reliably chosen this way, the statistical model is not
necessary anymore. In any case, the rule-based engine filters out languages that do not satisfy the conditions
of the input text. Only then, in a second step, the probabilistic n-gram model is taken into consideration.
This makes sense because loading less language models means less memory consumption and better runtime performance.

In general, it is always a good idea to restrict the set of languages to be considered in the classification process
using the respective api methods. If you know beforehand that certain languages are
never to occur in an input text, do not let those take part in the classifcation process. The filtering mechanism
of the rule-based engine is quite good, however, filtering based on your own knowledge of the input text is always preferable.

## 7. How to reproduce the accuracy results?

If you want to reproduce the accuracy results above, you can generate the test reports yourself for all classifiers 
and all languages by doing:

    cargo run --release --bin accuracy_reports --features accuracy-reports
    
It is important to use the `--release` flag here because loading the language models in debug mode takes too much time. 
For each detector and language, a test report file is then written into 
[`/accuracy-reports`](https://github.com/pemistahl/lingua-rs/tree/main/accuracy-reports), 
to be found next to the `src` directory. As an example, here is the current output of the *Lingua* German report:

```
##### German #####

>>> Accuracy on average: 89.23%

>> Detection of 1000 single words (average length: 9 chars)
Accuracy: 73.9%
Erroneously classified as Dutch: 2.3%, Danish: 2.1%, English: 2%, Latin: 1.9%, Bokmal: 1.6%, Basque: 1.2%, Esperanto: 1.2%, French: 1.2%, Italian: 1.2%, Swedish: 1%, Afrikaans: 0.8%, Tsonga: 0.7%, Nynorsk: 0.6%, Spanish: 0.6%, Yoruba: 0.6%, Finnish: 0.5%, Sotho: 0.5%, Welsh: 0.5%, Estonian: 0.4%, Irish: 0.4%, Polish: 0.4%, Swahili: 0.4%, Tagalog: 0.4%, Tswana: 0.4%, Bosnian: 0.3%, Icelandic: 0.3%, Romanian: 0.3%, Albanian: 0.2%, Catalan: 0.2%, Croatian: 0.2%, Indonesian: 0.2%, Lithuanian: 0.2%, Maori: 0.2%, Turkish: 0.2%, Xhosa: 0.2%, Zulu: 0.2%, Latvian: 0.1%, Malay: 0.1%, Slovak: 0.1%, Slovene: 0.1%, Somali: 0.1%

>> Detection of 1000 word pairs (average length: 18 chars)
Accuracy: 94.1%
Erroneously classified as Dutch: 0.9%, Latin: 0.8%, English: 0.7%, Swedish: 0.6%, Danish: 0.5%, French: 0.4%, Bokmal: 0.3%, Irish: 0.2%, Tagalog: 0.2%, Afrikaans: 0.1%, Esperanto: 0.1%, Estonian: 0.1%, Finnish: 0.1%, Italian: 0.1%, Maori: 0.1%, Nynorsk: 0.1%, Somali: 0.1%, Swahili: 0.1%, Tsonga: 0.1%, Turkish: 0.1%, Welsh: 0.1%, Zulu: 0.1%

>> Detection of 1000 sentences (average length: 111 chars)
Accuracy: 99.7%
Erroneously classified as Dutch: 0.2%, Latin: 0.1%
```

## 8. How to add it to your project?

Add *Lingua* to your `Cargo.toml` file like so:

```toml
[dependencies]
lingua = "1.6.2"
```

By default, this will download the language model dependencies for all 75 supported languages, 
a total of approximately 90 MB. If your bandwidth or hard drive space is limited, or you simply 
do not need all languages, you can specify a subset of the language models to be downloaded as 
separate features in your `Cargo.toml`:

```toml
[dependencies]
lingua = { version = "1.6.2", default-features = false, features = ["french", "italian", "spanish"] }
```

## 9. How to build?

In order to build the source code yourself, you need the 
[stable Rust toolchain](https://www.rust-lang.org/tools/install) installed on your machine 
so that [*cargo*](https://doc.rust-lang.org/cargo/), the Rust package manager is available.

```
git clone https://github.com/pemistahl/lingua-rs.git
cd lingua-rs
cargo build
```

The source code is accompanied by an extensive unit test suite. To run them, simply say:

    cargo test

With the help of [PyO3](https://github.com/PyO3/pyo3) and
[Maturin](https://github.com/PyO3/maturin), the library has been compiled to a
Python extension module so that it can be used within any Python software as well.
It is available in the [Python Package Index](https://pypi.org/project/lingua-language-detector) 
and can be installed with:

```shell
pip install lingua-language-detector
```

To build the Python extension module yourself, create a virtual environment and install
[Maturin](https://github.com/PyO3/maturin).

```shell
python -m venv .venv
source .venv/bin/activate
pip install maturin
maturin build
```

## 10. How to use?

### 10.1 Basic usage

```rust
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use lingua::Language::{English, French, German, Spanish};

let languages = vec![English, French, German, Spanish];
let detector: LanguageDetector = LanguageDetectorBuilder::from_languages(&languages).build();
let detected_language: Option<Language> = detector.detect_language_of("languages are awesome");

assert_eq!(detected_language, Some(English));
```

### 10.2 Minimum relative distance

By default, *Lingua* returns the most likely language for a given input text. However, there are
certain words that are spelled the same in more than one language. The word *prologue*, for
instance, is both a valid English and French word. *Lingua* would output either English or
French which might be wrong in the given context. For cases like that, it is possible to
specify a minimum relative distance that the logarithmized and summed up probabilities for
each possible language have to satisfy. It can be stated in the following way:

```rust
use lingua::LanguageDetectorBuilder;
use lingua::Language::{English, French, German, Spanish};

let detector = LanguageDetectorBuilder::from_languages(&[English, French, German, Spanish])
    .with_minimum_relative_distance(0.9)
    .build();
let detected_language = detector.detect_language_of("languages are awesome");

assert_eq!(detected_language, None);
```

Be aware that the distance between the language probabilities is dependent on the length of the
input text. The longer the input text, the larger the distance between the languages. So if you
want to classify very short text phrases, do not set the minimum relative distance too high.
Otherwise [`None`](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None) will be
returned most of the time as in the example above. This is the return value for cases where
language detection is not reliably possible.

### 10.3 Confidence values

Knowing about the most likely language is nice but how reliable is the computed likelihood?
And how less likely are the other examined languages in comparison to the most likely one?
These questions can be answered as well:

```rust
use lingua::Language::{English, French, German, Spanish};
use lingua::LanguageDetectorBuilder;

fn main() { 
    let languages = vec![English, French, German, Spanish];
    let detector = LanguageDetectorBuilder::from_languages(&languages).build();
    let confidence_values = detector
        .compute_language_confidence_values("languages are awesome")
        .into_iter()
        .map(|(language, confidence)| (language, (confidence * 100.0).round() / 100.0))
        .collect::<Vec<_>>();

    assert_eq!(
        confidence_values,
        vec![
            (English, 0.93),
            (French, 0.04),
            (German, 0.02),
            (Spanish, 0.01)
        ]
    );
}
```

In the example above, a vector of two-element tuples is returned containing all possible languages 
sorted by their confidence value in descending order. Each value is a probability between 0.0 and 1.0. The probabilities of 
all languages will sum to 1.0. If the language is unambiguously identified by the rule engine, the 
value 1.0 will always be returned for this language. The other languages will receive a value of 0.0.

There is also a method for returning the confidence value for one specific language only:

```rust
use lingua::Language::{English, French, German, Spanish};
use lingua::LanguageDetectorBuilder;

fn main() {
    let languages = vec![English, French, German, Spanish];
    let detector = LanguageDetectorBuilder::from_languages(&languages).build();
    let confidence = detector.compute_language_confidence("languages are awesome", French);
    let rounded_confidence = (confidence * 100.0).round() / 100.0;

    assert_eq!(rounded_confidence, 0.04);
}
```

The value that this method computes is a number between 0.0 and 1.0.
If the language is unambiguously identified by the rule engine, the value
1.0 will always be returned. If the given language is not supported by
this detector instance, the value 0.0 will always be returned.

### 10.4 Eager loading versus lazy loading

By default, *Lingua* uses lazy-loading to load only those language models on demand which are
considered relevant by the rule-based filter engine. For web services, for instance, it is
rather beneficial to preload all language models into memory to avoid unexpected latency while
waiting for the service response. If you want to enable the eager-loading mode, you can do it
like this:

```rust
LanguageDetectorBuilder::from_all_languages().with_preloaded_language_models().build();
```

Multiple instances of `LanguageDetector` share the same language models in memory which are
accessed asynchronously by the instances.

### 10.5 Low accuracy mode versus high accuracy mode

*Lingua's* high detection accuracy comes at the cost of being noticeably slower
than other language detectors. The large language models also consume significant
amounts of memory. These requirements might not be feasible for systems running low
on resources. If you want to classify mostly long texts or need to save resources,
you can enable a *low accuracy mode* that loads only a small subset of the language
models into memory:

```rust
LanguageDetectorBuilder::from_all_languages().with_low_accuracy_mode().build();
```

The downside of this approach is that detection accuracy for short texts consisting
of less than 120 characters will drop significantly. However, detection accuracy for
texts which are longer than 120 characters will remain mostly unaffected.

In high accuracy mode (the default), the language detector consumes approximately
970 MB of memory if all language models are loaded. In low accuracy mode, memory
consumption is reduced to approximately 72 MB. The goal is to further reduce memory
consumption in later releases.

An alternative for a smaller memory footprint and faster performance is to reduce the set
of languages when building the language detector. In most cases, it is not advisable to
build the detector from all supported languages. When you have knowledge about
the texts you want to classify you can almost always rule out certain languages as impossible
or unlikely to occur.

### 10.6 Detection of multiple languages in mixed-language texts

In contrast to most other language detectors, *Lingua* is able to detect multiple languages
in mixed-language texts. This feature can yield quite reasonable results, but it is still
in an experimental state and therefore the detection result is highly dependent on the input
text. It works best in high-accuracy mode with multiple long words for each language.
The shorter the phrases and their words are, the less accurate are the results. Reducing the
set of languages when building the language detector can also improve accuracy for this task
if the languages occurring in the text are equal to the languages supported by the respective
language detector instance.

```rust
use lingua::DetectionResult;
use lingua::Language::{English, French, German};
use lingua::LanguageDetectorBuilder;

fn main() {
    let languages = vec![English, French, German];
    let detector = LanguageDetectorBuilder::from_languages(&languages).build();
    let sentence = "Parlez-vous français? \
        Ich spreche Französisch nur ein bisschen. \
        A little bit is better than nothing.";

    let results: Vec<DetectionResult> = detector.detect_multiple_languages_of(sentence);

    if let [first, second, third] = &results[..] {
        assert_eq!(first.language(), French);
        assert_eq!(
            &sentence[first.start_index()..first.end_index()],
            "Parlez-vous français? "
        );

        assert_eq!(second.language(), German);
        assert_eq!(
            &sentence[second.start_index()..second.end_index()],
            "Ich spreche Französisch nur ein bisschen. "
        );

        assert_eq!(third.language(), English);
        assert_eq!(
            &sentence[third.start_index()..third.end_index()],
            "A little bit is better than nothing."
        );
    }
}
```

In the example above, a vector of [`DetectionResult`](https://github.com/pemistahl/lingua-rs/blob/main/src/result.rs#L21)
is returned. Each entry in the vector describes a contiguous single-language text section,
providing start and end indices of the respective substring.

### 10.7 Single-threaded versus multi-threaded language detection

The `LanguageDetector` methods explained above all operate in a single thread.
If you want to classify a very large set of texts, you will probably want to
use all available CPU cores efficiently in multiple threads for maximum performance.

Every single-threaded method has a multi-threaded equivalent that accepts a list of texts
and returns a list of results.

| Single-threaded                      | Multi-threaded                                   |
|--------------------------------------|--------------------------------------------------|
| `detect_language_of`                 | `detect_languages_in_parallel_of`                |
| `detect_multiple_languages_of`       | `detect_multiple_languages_in_parallel_of`       |
| `compute_language_confidence_values` | `compute_language_confidence_values_in_parallel` |
| `compute_language_confidence`        | `compute_language_confidence_in_parallel`        |

### 10.8 Methods to build the LanguageDetector

There might be classification tasks where you know beforehand that your language data is
definitely not written in Latin, for instance (what a surprise :-). The detection accuracy can
become better in such cases if you exclude certain languages from the decision process or just
explicitly include relevant languages:

```rust
use lingua::{LanguageDetectorBuilder, Language, IsoCode639_1, IsoCode639_3};

// Include all languages available in the library.
LanguageDetectorBuilder::from_all_languages();

// Include only languages that are not yet extinct (= currently excludes Latin).
LanguageDetectorBuilder::from_all_spoken_languages();

// Include only languages written with Cyrillic script.
LanguageDetectorBuilder::from_all_languages_with_cyrillic_script();

// Exclude only the Spanish language from the decision algorithm.
LanguageDetectorBuilder::from_all_languages_without(&[Language::Spanish]);

// Only decide between English and German.
LanguageDetectorBuilder::from_languages(&[Language::English, Language::German]);

// Select languages by ISO 639-1 code.
LanguageDetectorBuilder::from_iso_codes_639_1(&[IsoCode639_1::EN, IsoCode639_1::DE]);

// Select languages by ISO 639-3 code.
LanguageDetectorBuilder::from_iso_codes_639_3(&[IsoCode639_3::ENG, IsoCode639_3::DEU]);
```

## 11. WebAssembly support

This library can be compiled to [WebAssembly (WASM)](https://webassembly.org) which allows to use *Lingua*
in any JavaScript-based project, be it in the browser or in the back end running on [Node.js](https://nodejs.org).

The easiest way to compile is to use [`wasm-pack`](https://rustwasm.github.io/wasm-pack). After the installation,
you can, for instance, build the library with the web target so that it can be directly used in the browser:

    wasm-pack build --target web

By default, all 75 supported languages are included in the compiled wasm file which has a size of 74 MB, approximately. 
If you only need a subset of certain languages, you can tell `wasm-pack` which ones to include:

    wasm-pack build --target web -- --no-default-features --features "french,italian,spanish"

This creates a directory named `pkg` on the top-level of this repository, containing the compiled wasm files
and JavaScript and TypeScript bindings. In an HTML file, you can then call *Lingua* like the following, for instance:

```html
<script type="module">
    import init, { LanguageDetectorBuilder } from './pkg/lingua.js';

    init().then(_ => {
        const detector = LanguageDetectorBuilder.fromAllLanguages().build();
        console.log(detector.computeLanguageConfidenceValues("languages are awesome"));
    });
</script>
```

There are also some integration tests available for Node.js and all major browsers.
To run them, simply say:

    wasm-pack test --node --headless --chrome --firefox --safari

The output of `wasm-pack` will be hosted in a [separate repository](https://github.com/pemistahl/lingua-js) which
allows to add further JavaScript-related configuration, tests and documentation. *Lingua* will then be added to the 
[npm registry](https://www.npmjs.com) as well, allowing for an easy download and installation within every JavaScript 
or TypeScript project.

## 12. What's next for version 1.7.0?

Take a look at the [planned issues](https://github.com/pemistahl/lingua-rs/milestone/9).

## 13. Contributions

- [Josh Rotenberg](https://github.com/joshrotenberg) has written a [wrapper](https://github.com/joshrotenberg/lingua_ex)
for using *Lingua* with the [Elixir programming language](https://elixir-lang.org/).
  
- [Simon Liang](https://github.com/lhr0909) has written a [wrapper](https://github.com/xanthous-tech/lingua-node)
for using *Lingua* with [NodeJS](https://nodejs.org/).

Any contributions to *Lingua* are very much appreciated. Please read the instructions
in [`CONTRIBUTING.md`](https://github.com/pemistahl/lingua-rs/blob/main/CONTRIBUTING.md)
for how to add new languages to the library.
