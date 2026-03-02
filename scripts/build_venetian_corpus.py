#!/usr/bin/env python3
"""Download Venetian Wikipedia, Wiktionary, and Tatoeba sentences,
then extract clean Venetian text for lingua-rs corpus."""

import bz2
import csv
import re
import ssl
import subprocess
import urllib.request
import xml.etree.ElementTree as ET
from pathlib import Path

WIKI_DUMP_URL = "https://dumps.wikimedia.org/vecwiki/latest/vecwiki-latest-pages-articles.xml.bz2"
WIKT_DUMP_URL = "https://dumps.wikimedia.org/vecwiktionary/latest/vecwiktionary-latest-pages-articles.xml.bz2"
TATOEBA_URL = "https://downloads.tatoeba.org/exports/per_language/vec/vec_sentences.tsv.bz2"

WIKI_CACHE = Path("/tmp/vecwiki-latest-pages-articles.xml.bz2")
WIKT_CACHE = Path("/tmp/vecwiktionary-latest-pages-articles.xml.bz2")
TATOEBA_CACHE = Path("/tmp/tatoeba_vec.tsv")

OUTPUT_PATH = Path("/tmp/venetian_clean.txt")

# Minimum character length for a sentence to be included
MIN_SENTENCE_LEN = 40

# Year-stub and formulaic patterns to reject
REJECT_PATTERNS = [
    re.compile(r"\b(numari|numeri)\s+romani\b", re.IGNORECASE),
    re.compile(r"^\(.*\)\s*(el|ła|la)\s+(xe|ze)\s+(on|un)\s+an\b"),
    re.compile(r"^(El|La|Le|Ła)\s+\d+\s+\("),
    re.compile(r"^thumb\b", re.IGNORECASE),
    re.compile(r"^\[\["),
    re.compile(r"^Categoria:", re.IGNORECASE),
    re.compile(r"^(right|left|center)\|", re.IGNORECASE),
]

# Formulaic geographic stub patterns
GEO_STUB_PATTERNS = [
    re.compile(
        r"(el xe|la xe|el ze|la ze|ła ze|ła xe)\s+(on|un)\s+"
        r"(comun|paeze|çità|frasion|frazsion|viłajo|borgo|distreto|canton)",
        re.IGNORECASE,
    ),
    re.compile(
        r"(el xe|el ze)\s+(on|un)\s+(stado|stato|paeze)\s+de",
        re.IGNORECASE,
    ),
]

# Lines should have a minimum ratio of letter characters
MIN_LETTER_RATIO = 0.6


def download_file(url: str, cache: Path) -> bytes:
    if cache.exists():
        print(f"  Using cached {cache.name}")
        return cache.read_bytes()

    try:
        print(f"  Downloading {url} ...")
        req = urllib.request.Request(url, headers={"User-Agent": "lingua-rs-corpus-builder/1.0"})
        with urllib.request.urlopen(req) as resp:
            data = resp.read()
        cache.write_bytes(data)
    except (ssl.SSLError, urllib.error.URLError):
        print("  urllib SSL failed, falling back to curl ...")
        subprocess.run(["curl", "-fSL", "-o", str(cache), url], check=True)
        data = cache.read_bytes()

    print(f"  Downloaded {len(data) / 1024 / 1024:.1f} MB")
    return data


def strip_wiki_markup(text: str) -> str:
    # Remove templates {{...}} (nested)
    depth = 0
    result = []
    i = 0
    while i < len(text):
        if text[i : i + 2] == "{{":
            depth += 1
            i += 2
        elif text[i : i + 2] == "}}" and depth > 0:
            depth -= 1
            i += 2
        elif depth == 0:
            result.append(text[i])
            i += 1
        else:
            i += 1
    text = "".join(result)

    # Remove HTML comments
    text = re.sub(r"<!--.*?-->", "", text, flags=re.DOTALL)
    # Remove <ref>...</ref> and self-closing <ref ... />
    text = re.sub(r"<ref[^>]*>.*?</ref>", "", text, flags=re.DOTALL)
    text = re.sub(r"<ref[^/]*/\s*>", "", text)
    # Remove remaining HTML tags
    text = re.sub(r"<[^>]+>", "", text)
    # Remove files/images [[File:...]] [[Image:...]]
    text = re.sub(r"\[\[(File|Image|Immagine|Imagine):[^\]]*\]\]", "", text, flags=re.IGNORECASE)
    # Convert wikilinks [[target|display]] -> display, [[target]] -> target
    text = re.sub(r"\[\[[^|\]]*\|([^\]]+)\]\]", r"\1", text)
    text = re.sub(r"\[\[([^\]]+)\]\]", r"\1", text)
    # Remove external links [url text] -> text
    text = re.sub(r"\[https?://[^\s\]]+ ([^\]]+)\]", r"\1", text)
    text = re.sub(r"\[https?://[^\]]+\]", "", text)
    # Remove bold/italic markup
    text = re.sub(r"'{2,5}", "", text)
    # Remove headings
    text = re.sub(r"^=+.*?=+\s*$", "", text, flags=re.MULTILINE)
    # Remove table markup
    text = re.sub(r"^\s*\{\|.*$", "", text, flags=re.MULTILINE)
    text = re.sub(r"^\s*\|\}.*$", "", text, flags=re.MULTILINE)
    text = re.sub(r"^\s*[|!].*$", "", text, flags=re.MULTILINE)
    # Remove category links
    text = re.sub(r"\[\[(Category|Categoria):[^\]]*\]\]", "", text, flags=re.IGNORECASE)
    # Remove bullet/numbered list markers
    text = re.sub(r"^[*#:;]+\s*", "", text, flags=re.MULTILINE)
    # Collapse whitespace
    text = re.sub(r"[ \t]+", " ", text)
    return text


def is_good_sentence(sent: str) -> bool:
    """Return True if the sentence is likely genuine Venetian prose."""
    if len(sent) < MIN_SENTENCE_LEN:
        return False

    # Must contain Venetian letter characters
    letters = sum(1 for c in sent if c.isalpha())
    if letters / max(len(sent), 1) < MIN_LETTER_RATIO:
        return False

    # Reject formulaic patterns
    for pat in REJECT_PATTERNS:
        if pat.search(sent):
            return False
    for pat in GEO_STUB_PATTERNS:
        if pat.search(sent):
            return False

    # Reject lines that are just metadata/markup remnants
    if re.match(r"^[\d\s|{}\[\]<>*#:;=]+$", sent):
        return False

    return True


def extract_sentences(text: str) -> list[str]:
    sentences = []
    for line in text.split("\n"):
        line = line.strip()
        if len(line) < MIN_SENTENCE_LEN:
            continue
        # Split on sentence boundaries
        for sent in re.split(r"(?<=[.!?])\s+", line):
            sent = sent.strip()
            if is_good_sentence(sent):
                sentences.append(sent)
    return sentences


def extract_wiki_articles(xml_data: bytes) -> list[str]:
    """Extract sentences from a MediaWiki XML dump."""
    root = ET.fromstring(xml_data)

    ns = ""
    tag = root.tag
    if tag.startswith("{"):
        ns = tag[: tag.index("}") + 1]

    all_sentences: list[str] = []
    article_count = 0

    for page in root.iter(f"{ns}page"):
        ns_elem = page.find(f"{ns}ns")
        if ns_elem is None or ns_elem.text != "0":
            continue

        redirect = page.find(f"{ns}redirect")
        if redirect is not None:
            continue

        revision = page.find(f"{ns}revision")
        if revision is None:
            continue
        text_elem = revision.find(f"{ns}text")
        if text_elem is None or not text_elem.text:
            continue

        raw = text_elem.text

        # Skip very short articles (likely stubs)
        if len(raw) < 200:
            continue

        cleaned = strip_wiki_markup(raw)
        sentences = extract_sentences(cleaned)
        if sentences:
            all_sentences.extend(sentences)
            article_count += 1

    return all_sentences, article_count


def extract_wiktionary(xml_data: bytes) -> list[str]:
    """Extract example sentences and definitions from Wiktionary."""
    root = ET.fromstring(xml_data)

    ns = ""
    tag = root.tag
    if tag.startswith("{"):
        ns = tag[: tag.index("}") + 1]

    sentences: list[str] = []

    for page in root.iter(f"{ns}page"):
        ns_elem = page.find(f"{ns}ns")
        if ns_elem is None or ns_elem.text != "0":
            continue

        redirect = page.find(f"{ns}redirect")
        if redirect is not None:
            continue

        revision = page.find(f"{ns}revision")
        if revision is None:
            continue
        text_elem = revision.find(f"{ns}text")
        if text_elem is None or not text_elem.text:
            continue

        raw = text_elem.text
        cleaned = strip_wiki_markup(raw)

        # Extract any sentence-like lines from definitions/examples
        for line in cleaned.split("\n"):
            line = line.strip()
            if is_good_sentence(line):
                sentences.append(line)

    return sentences


def load_tatoeba(tsv_path: Path) -> list[str]:
    """Load Venetian sentences from Tatoeba TSV."""
    sentences = []
    if not tsv_path.exists():
        return sentences
    with open(tsv_path, encoding="utf-8") as f:
        reader = csv.reader(f, delimiter="\t")
        for row in reader:
            if len(row) >= 3:
                sent = row[2].strip()
                if len(sent) >= 10:  # Tatoeba sentences can be shorter
                    sentences.append(sent)
    return sentences


def main():
    all_sentences: list[str] = []

    # 1. Wikipedia (primary source)
    print("=== Wikipedia ===")
    data = download_file(WIKI_DUMP_URL, WIKI_CACHE)
    print("  Decompressing bz2 ...")
    xml_data = bz2.decompress(data)
    print(f"  Decompressed to {len(xml_data) / 1024 / 1024:.1f} MB XML")
    wiki_sents, article_count = extract_wiki_articles(xml_data)
    print(f"  Extracted {len(wiki_sents)} sentences from {article_count} articles")
    all_sentences.extend(wiki_sents)

    # 2. Wiktionary (supplementary)
    print("\n=== Wiktionary ===")
    data = download_file(WIKT_DUMP_URL, WIKT_CACHE)
    print("  Decompressing bz2 ...")
    xml_data = bz2.decompress(data)
    wikt_sents = extract_wiktionary(xml_data)
    print(f"  Extracted {len(wikt_sents)} sentences")
    all_sentences.extend(wikt_sents)

    # 3. Tatoeba (supplementary)
    print("\n=== Tatoeba ===")
    download_file(TATOEBA_URL, Path("/tmp/tatoeba_vec.tsv.bz2"))
    if not TATOEBA_CACHE.exists():
        subprocess.run(["bunzip2", "-fk", "/tmp/tatoeba_vec.tsv.bz2"], check=True)
    tatoeba_sents = load_tatoeba(TATOEBA_CACHE)
    print(f"  Loaded {len(tatoeba_sents)} sentences")
    all_sentences.extend(tatoeba_sents)

    # Deduplicate while preserving order
    seen = set()
    unique = []
    for s in all_sentences:
        normalized = s.lower().strip()
        if normalized not in seen:
            seen.add(normalized)
            unique.append(s)

    print(f"\n=== Total: {len(unique)} unique sentences ===")

    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    with open(OUTPUT_PATH, "w", encoding="utf-8") as f:
        for s in unique:
            f.write(s + "\n")

    print(f"Written to {OUTPUT_PATH}")


if __name__ == "__main__":
    main()
