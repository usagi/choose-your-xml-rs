**DEPRECATED** in favor to the [roxmltree](https://github.com/RazrFalcon/roxmltree#alternatives).


# Choose your XML

|         Feature/Crate         |       [xml-rs]     |     [quick-xml]    |       [xml5ever]    |    [sxd-document]  |
| ----------------------------- | ------------------ | ------------------ | ------------------- | ------------------ |
| [Document type definition]    | No                 | No                 | Partial<sup>1</sup> | No                 |
| Namespaces                    | Yes                | Yes                | Yes                 | Yes                |
| [Numeric character reference] | Yes                | Yes                | Yes                 | Yes                |
| [XML entity reference]        | Yes                | Yes                | Yes                 | Yes                |
| [HTML entity reference]       | No                 | No                 | Yes                 | No                 |
| [DTD entity reference]        | No                 | No                 | No                  | No                 |
| `xml:space`                   | No                 | No                 | No                  | No                 |
| Non UTF-8 input               | No                 | Yes                | No                  | No                 |
| [XPath]                       | No                 | No                 | No                  | 1.0<sup>2</sup>    |
| [XQuery]                      | No                 | No                 | No                  | No                 |
| Streaming parser              | Yes                | Yes                | Yes                 | No                 |
| Pull parser                   | Yes                | Yes                | No                  | -                  |
| Push parser                   | No                 | No                 | Yes                 | -                  |
| DOM                           | Extern<sup>3</sup> | Extern<sup>4</sup> | Yes<sup>5</sup>     | Yes                |
| Error recovery                | No                 | No                 | Partial<sup>6</sup> | No                 |
| Writing                       | Yes                | Yes                | Yes                 | Yes                |
| Tested version                | 0.7.0              | 0.10.0             | 0.11.1              | 0.2.4              |
| Latest version                | ![][xml-rs-ver]    | ![][quick-xml-ver] | ![][xml5ever-ver]   | ![][sxd-doc-ver]   |
| License                       | MIT                | MIT                | MIT / Apache-2.0    | MIT                |

1. Only DTD without children. Also as stated in the [readme](https://github.com/servo/html5ever/blob/master/xml5ever/README.md#when-you-shouldnt-use-it).
2. Via [`sxd-xpath`](https://crates.io/crates/sxd-xpath) crate.
3. Via [`treexml`](https://crates.io/crates/treexml) or [`elementtree`](https://crates.io/crates/elementtree) crates.
4. Via [`minidom`](https://crates.io/crates/minidom) crate.
5. Using [`rcdom`](https://github.com/servo/html5ever/blob/master/markup5ever/rcdom.rs) module from the crate.
6. As stated in the [readme](https://github.com/servo/html5ever/blob/master/xml5ever/README.md#xml5ever).

[xml-rs]: https://crates.io/crates/xml-rs
[quick-xml]: https://crates.io/crates/quick-xml
[xml5ever]: https://crates.io/crates/xml5ever
[sxd-document]: https://crates.io/crates/sxd-document

[Document type definition]: https://en.wikipedia.org/wiki/Document_type_definition
[Numeric character reference]: https://en.wikipedia.org/wiki/Numeric_character_reference
[XPath]: https://en.wikipedia.org/wiki/XPath
[XQuery]: https://en.wikipedia.org/wiki/XQuery
[XML entity reference]: https://en.wikipedia.org/wiki/List_of_XML_and_HTML_character_entity_references#Predefined_entities_in_XML
[HTML entity reference]: https://en.wikipedia.org/wiki/List_of_XML_and_HTML_character_entity_references#Character_entity_references_in_HTML
[DTD entity reference]: https://en.wikipedia.org/wiki/List_of_XML_and_HTML_character_entity_references#Character_reference_overview

[xml-rs-ver]: https://img.shields.io/crates/v/xml-rs.svg
[quick-xml-ver]: https://img.shields.io/crates/v/quick-xml.svg
[xml5ever-ver]: https://img.shields.io/crates/v/xml5ever.svg
[sxd-doc-ver]: https://img.shields.io/crates/v/sxd-document.svg

You can test features by yourself using corresponding example:

```bash
cargo run --example crate_name file_path

# example
cargo run --example quick_xml data/small.xml
```

## Performance

```
test quick_xml_large     ... bench:   1,864,496 ns/iter (+/- 1,292)
test quick_xml_medium    ... bench:     465,637 ns/iter (+/- 539)
test quick_xml_small     ... bench:       6,339 ns/iter (+/- 24)

test sxd_document_large  ... bench:      failed
test sxd_document_medium ... bench:   2,639,623 ns/iter (+/- 11,376)
test sxd_document_small  ... bench:      42,252 ns/iter (+/- 149)

test xml5ever_large      ... bench:   8,621,706 ns/iter (+/- 9,631)
test xml5ever_medium     ... bench:   7,181,995 ns/iter (+/- 8,180)
test xml5ever_small      ... bench:      45,492 ns/iter (+/- 49)

test xmlrs_large         ... bench:  25,715,749 ns/iter (+/- 27,183)
test xmlrs_medium        ... bench:  12,798,271 ns/iter (+/- 13,670)
test xmlrs_small         ... bench:      90,392 ns/iter (+/- 132)
```

![](chart.svg)

You can run benchmarks by yourself using `cargo bench`.

\* Note that `sxd-document` generates a whole DOM.

## Contributing

Any suggestions and patches are welcome. Especially from crate authors.

## License

This repo is licensed under MIT license.
