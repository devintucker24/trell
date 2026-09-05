# MarkItDown local conversion for RepoBrain issues #15 and #16

**Date:** 5 September 2026  
**Accessed:** 2026-09-05  
**Status:** Research note, not an implementation spec  
**Scope:** The narrow local MarkItDown tracer for RepoBrain issue
[#15](https://github.com/devintucker24/trell/issues/15), with dependency and
policy awareness for issue
[#16](https://github.com/devintucker24/trell/issues/16).

Only first-party sources were used: Microsoft's MarkItDown repository and
tagged source, its GitHub releases, PyPI package metadata, and the repository
license. Dependency observations use the dependencies' own PyPI metadata.

## Recommendation

Use a tiny UTF-8 **CSV** fixture and pin:

```text
markitdown==0.1.7
```

There is no `csv` extra. CSV support is in the base package, so
`markitdown[csv]` is invalid and `[all]` is unnecessarily broad. CSV is the
narrowest useful tracer because it:

- visibly transforms structure into a Markdown table, rather than merely
  passing plain text through;
- is local and text-based;
- does not invoke archive handling, a document parser, OCR, a model, a plugin,
  a cloud service, or a media tool;
- needs no format-specific optional dependency or external executable; and
- can be generated deterministically in a Rust/Python test.

This does **not** mean the base installation is free of native artifacts.
MarkItDown 0.1.7 unconditionally depends on `magika~=0.6.1`. Magika's package
metadata declares `numpy` and `onnxruntime`, and its platform wheels include a
precompiled Rust CLI. A clean Python 3.12 install performed for this research
selected `magika==0.6.3`, `numpy==2.5.2`, and `onnxruntime==1.29.0` platform
wheels. CSV avoids *additional format-specific* binary dependencies, but a
supported `markitdown==0.1.7` install cannot honestly be described as
introducing no binary dependencies.

## Version, Python, and license

As of 2026-09-05:

- **Latest non-prerelease package/release:** `0.1.7`. GitHub marks `v0.1.7`
  non-prerelease and published it on 2026-07-29. PyPI's 0.1.7 distribution is
  not yanked.
- **Newer prerelease:** PyPI published `0.1.8b1` on 2026-09-04; it is a beta
  prerelease and is not the stable pin for issue #15.
- **Declared Python requirement:** `>=3.10`, with classifiers for Python
  3.10, 3.11, 3.12, and 3.13 and for CPython and PyPy. The metadata declares no
  upper bound.
- **Project maturity classifier:** `Development Status :: 4 - Beta`, even
  though 0.1.7 is the latest non-prerelease version.
- **License:** MIT. The tagged `LICENSE` grants use, modification,
  distribution, sublicensing, and sale subject to retaining the copyright and
  permission notice, and disclaims warranty.

Sources:

- [GitHub release v0.1.7](https://github.com/microsoft/markitdown/releases/tag/v0.1.7)
- [PyPI metadata for 0.1.7](https://pypi.org/pypi/markitdown/0.1.7/json)
- [PyPI page for 0.1.8b1](https://pypi.org/project/markitdown/0.1.8b1/)
- [Tagged package metadata (`pyproject.toml`)](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/pyproject.toml)
- [Tagged MIT license](https://github.com/microsoft/markitdown/blob/v0.1.7/LICENSE)

All sources in this note were accessed 2026-09-05.

## Exact install surface

For the CSV tracer:

```bash
python -m pip install 'markitdown==0.1.7'
```

The base package dependencies in the 0.1.7 metadata are:
`beautifulsoup4`, `charset-normalizer`, `defusedxml`, `magika~=0.6.1`,
`markdownify`, and `requests`.

The exact optional extras exposed by 0.1.7 are:

| Extra | Additional requirements in 0.1.7 | Relevant capability |
| --- | --- | --- |
| `pptx` | `python-pptx` | PowerPoint |
| `docx` | `mammoth~=1.11.0`, `lxml` | Word |
| `xlsx` | `pandas`, `openpyxl` | Excel `.xlsx` |
| `xls` | `pandas`, `xlrd` | Legacy Excel `.xls` |
| `pdf` | `pdfminer-six>=20251230`, `pdfplumber>=0.11.9` | PDF |
| `outlook` | `olefile` | Outlook messages |
| `audio-transcription` | `pydub`, `SpeechRecognition` | Audio transcription |
| `youtube-transcription` | `youtube-transcript-api` | YouTube transcripts |
| `az-doc-intel` | `azure-ai-documentintelligence`, `azure-identity` | Azure Document Intelligence |
| `az-content-understanding` | `azure-ai-contentunderstanding>=1.2.0b1`, `azure-identity` | Azure Content Understanding |
| `all` | The union listed in `pyproject.toml` | Every optional built-in capability |

There are **no** `csv`, `html`, `epub`, `zip`, `json`, `xml`, or `text`
extras. Those converters use the base dependency set. For issue #16, install
only the enabled format extras; do not use `[all]`.

Sources:

- [PyPI 0.1.7 `requires_dist` and `provides_extra`](https://pypi.org/pypi/markitdown/0.1.7/json)
- [Tagged optional-dependency declarations](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/pyproject.toml)
- [Magika 0.6.1 PyPI metadata](https://pypi.org/pypi/magika/0.6.1/json)

## Local-file API and result

Use the narrow local API explicitly:

```python
from pathlib import Path

from markitdown import MarkItDown

source = Path("/validated/repository/root/fixtures/people.csv")
converter = MarkItDown(enable_plugins=False)
result = converter.convert_local(source)

derived_markdown: str = result.markdown
source_title: str | None = result.title
```

`convert_local()` accepts either `str` or `pathlib.Path`. It opens the source
as `"rb"` and returns `DocumentConverterResult`:

- `result.markdown: str` is the converted Markdown;
- `result.title: Optional[str]` is optional and is `None` for CSV;
- `result.text_content` is a soft-deprecated read/write alias for
  `result.markdown`; new code should use `markdown`; and
- `str(result)` returns `result.markdown`.

For a seekable or non-seekable binary file-like object:

```python
from io import BytesIO

from markitdown import MarkItDown, StreamInfo

result = MarkItDown(enable_plugins=False).convert_stream(
    BytesIO(b"name,role\nAda,engineer\n"),
    stream_info=StreamInfo(extension=".csv", charset="utf-8"),
)
assert result.markdown == (
    "| name | role |\n"
    "| --- | --- |\n"
    "| Ada | engineer |"
)
```

`convert_stream()` requires a **binary** file-like object. Converter streams
must support `read()`, `seek()`, and `tell()`; the public method copies a
non-seekable stream into an in-memory `BytesIO`. Passing an `io.TextIOBase` to
the generic `convert()` is rejected with `TypeError`. Supplying `StreamInfo`
is advisable for deterministic dispatch and decoding.

The generic `convert()` also accepts a local `str`, a `Path`, a
`requests.Response`, or a binary stream, but it is intentionally permissive:
strings beginning with `http:`, `https:`, `file:`, or `data:` are treated as
URIs. RepoBrain should not use it for this local-only seam.

Sources:

- [Tagged `MarkItDown` implementation](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/src/markitdown/_markitdown.py)
- [Tagged result and converter interfaces](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/src/markitdown/_base_converter.py)
- [Tagged public exports](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/src/markitdown/__init__.py)
- [Tagged CSV converter](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/src/markitdown/converters/_csv_converter.py)

## Security and network behavior

MarkItDown is a converter, not a sandbox.

- The official security guidance says it performs I/O with the current
  process's privileges. `convert_local()` calls Python `open()` on the supplied
  path. It does not enforce repository roots, block symlinks, apply file-size
  limits, or protect the source from a caller that later writes to it.
- Validate and canonicalize the path against repository-managed input roots
  before conversion. Keep source and derived roots distinct. Open/convert only
  after the root check, and account for symlink/race behavior in the RepoBrain
  boundary.
- `convert_local()` opens the source read-only and the library returns an
  in-memory result; it does not itself write derived Markdown or overwrite the
  source.
- `convert_uri()` performs `requests.Session.get(..., stream=True)` for HTTP(S)
  and follows the process/session's network capabilities. The official docs
  explicitly tell server-side callers to restrict schemes and network
  destinations, including private, loopback, link-local, and metadata-service
  addresses.
- `convert_local()` on CSV has no network call. File identification by Magika
  is local model inference.
- Plugins are disabled by default, but installed entry-point plugins execute
  Python registration/conversion code when enabled. Keep
  `enable_plugins=False`; do not make plugin enablement implicit from package
  installation.
- CSV conversion reads the entire input and materializes all rows. Enforce an
  input byte limit before calling it.

Use an allowlist such as `.csv`, an explicit UTF-8 `StreamInfo`, a byte-size
limit, plugins off, no LLM/client endpoints, and `convert_local()` only. Do not
accept a converter method or URI from user configuration for issue #15.

Sources:

- [Official README security considerations at v0.1.7](https://github.com/microsoft/markitdown/blob/v0.1.7/README.md#security-considerations)
- [Tagged URI/local/stream implementation](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/src/markitdown/_markitdown.py)
- [Tagged plugin documentation](https://github.com/microsoft/markitdown/blob/v0.1.7/README.md#plugins)
- [Tagged CSV implementation](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/src/markitdown/converters/_csv_converter.py)

## Failure behavior and malformed-input caveats

The public failure surface is not a single exception:

| Situation | Observed/source-defined behavior |
| --- | --- |
| Missing, denied, or invalid local path | `open()` errors such as `FileNotFoundError`, `PermissionError`, or `IsADirectoryError` propagate directly from `convert_local()` because opening occurs outside the converter-attempt wrapper. |
| Recognized type, converter throws | MarkItDown records each failed converter attempt and, if no converter succeeds, raises `FileConversionException`. Its `attempts` contains converters and captured exception information. |
| Optional format dependency absent | The format converter raises `MissingDependencyException`; the dispatcher records the attempt and may ultimately expose it through `FileConversionException` if no converter succeeds. |
| No converter accepts the input | `UnsupportedFormatException`. |
| Invalid source object passed to `convert()` | `TypeError`. |
| Non-seekable binary stream | Buffered fully into memory before conversion. |

CSV specifically is permissive:

- Python's default `csv.reader` is used without `strict=True`. Broken quoting
  and ragged rows are not a reliable malformed-fixture failure.
- Rows shorter than the header are padded; rows longer than the header are
  silently truncated.
- Cells are joined into Markdown without escaping `|` or Markdown content.
  Keep the tracer fixture deliberately simple.
- Without an explicit charset, `charset-normalizer` guesses the encoding.
  Invalid bytes may therefore decode rather than fail.

For a deterministic malformed-input integration test, pass a declared UTF-8
`StreamInfo` and invalid UTF-8 bytes:

```python
from io import BytesIO

from markitdown import MarkItDown, StreamInfo
from markitdown._exceptions import FileConversionException

try:
    MarkItDown(enable_plugins=False).convert_stream(
        BytesIO(b"name\n\xff\n"),
        stream_info=StreamInfo(extension=".csv", charset="utf-8"),
    )
except FileConversionException as error:
    # Record source, converter/version, safe reason, and retryability.
    assert error.attempts
else:
    raise AssertionError("invalid UTF-8 unexpectedly converted")
```

Do not persist raw tracebacks or arbitrary exception text without
normalization; parser errors and paths can contain sensitive data. RepoBrain's
manifest should classify the failure, preserve a safe reason, and mark it
retryable without blocking non-strict setup, as issue #15 requires.

Sources:

- [Tagged exception definitions](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/src/markitdown/_exceptions.py)
- [Tagged dispatch and error aggregation](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/src/markitdown/_markitdown.py)
- [Tagged CSV converter](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/src/markitdown/converters/_csv_converter.py)

## Features to keep outside the #15 tracer

| Feature/path | Requirement and behavior | Policy for #15 / awareness for #16 |
| --- | --- | --- |
| PDF | `[pdf]`; local `pdfminer-six` and `pdfplumber` extraction | Explicit format enablement in #16; scanned/image-only content needs OCR/cloud for useful extraction. |
| DOCX | `[docx]`; `mammoth` and `lxml` | Explicit format enablement in #16. |
| PPTX | `[pptx]`; `python-pptx`; embedded image descriptions additionally need an `llm_client` and `llm_model` | Local text/table extraction may be enabled in #16; model captioning remains off. |
| XLSX / XLS | `[xlsx]` (`pandas`, `openpyxl`) or `[xls]` (`pandas`, `xlrd`) | Explicit format enablement in #16. |
| HTML | Base package; BeautifulSoup/markdownify; scripts and styles are removed | Safe-local candidate for #16, but malformed HTML is usually repaired/fallback-extracted rather than rejected. |
| EPUB | Base package; ZIP and XML/HTML parsing | #16 format; apply size/archive limits. Malformed EPUBs generally become `FileConversionException`. |
| Generic ZIP | Base package; recursively reads members and converts supported ones | Keep disabled unless explicitly required. Archive expansion and nested content need dedicated limits/policy. |
| Image description | JPEG/PNG converter; useful metadata requires a configured ExifTool executable, and descriptions require a multimodal `llm_client`/`llm_model` | Off by default. ExifTool is an external binary; model behavior may be networked/billable depending on the client. |
| OCR | Separate `markitdown-ocr` plugin plus an OpenAI-compatible vision client/model; Azure services can also OCR | Plugins, model calls, and cloud OCR off by default. Core image/PDF conversion is not general local OCR. |
| Audio transcription | `[audio-transcription]`; `pydub` and `SpeechRecognition`; source calls `recognize_google()` | Media and network transcription off by default. |
| Video | Azure Content Understanding is the documented video path | Cloud, networked, and billable; off by default. |
| YouTube URL/transcript | HTTP(S) page fetch plus `[youtube-transcription]`; transcript API makes network requests | Remote/network path; off by default. |
| Azure Document Intelligence | `[az-doc-intel]`, endpoint and credentials | Cloud/network/billable; explicit host configuration only. |
| Azure Content Understanding | `[az-content-understanding]`, endpoint and credentials; each routed conversion is documented as billable | Cloud/network/billable; explicit host configuration and file-type allowlist only. |
| Third-party converters | Separately installed plugin and `enable_plugins=True` | Arbitrary extension code; off by default. |

Sources:

- [Official v0.1.7 README: formats, extras, plugins, model and Azure paths](https://github.com/microsoft/markitdown/blob/v0.1.7/README.md)
- [Tagged image converter](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/src/markitdown/converters/_image_converter.py)
- [Tagged audio converter](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/src/markitdown/converters/_audio_converter.py)
- [Tagged audio transcription implementation](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/src/markitdown/converters/_transcribe_audio.py)
- [Tagged ExifTool wrapper](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/src/markitdown/converters/_exiftool.py)
- [Tagged YouTube converter](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/src/markitdown/converters/_youtube_converter.py)
- [Tagged PDF converter](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/src/markitdown/converters/_pdf_converter.py)
- [Tagged DOCX converter](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/src/markitdown/converters/_docx_converter.py)
- [Tagged PPTX converter](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/src/markitdown/converters/_pptx_converter.py)
- [Tagged spreadsheet converter](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/src/markitdown/converters/_xlsx_converter.py)
- [Tagged EPUB converter](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/src/markitdown/converters/_epub_converter.py)
- [Tagged ZIP converter](https://github.com/microsoft/markitdown/blob/v0.1.7/packages/markitdown/src/markitdown/converters/_zip_converter.py)

## Local validation performed

An isolated `--target` install on Python 3.12.3 confirmed:

```text
version: 0.1.7
result_type: DocumentConverterResult
markdown: '| name | role |\n| --- | --- |\n| Ada | engineer |'
title: None
alias_equal: True
stream_equal: True
malformed_exception: FileConversionException
failed_attempts: 2
```

The test replaced the instance's HTTP session `get` method with an assertion
before exercising `convert_local(Path(...))` and `convert_stream(...)`; no
network path was invoked by either CSV conversion. It also confirms that a
declared UTF-8 stream containing `0xff` gives a retryable conversion failure
surface rather than converted output.

