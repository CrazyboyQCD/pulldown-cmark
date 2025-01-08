// This file is auto-generated by the build script
// Please, do not modify it manually

use super::test_markdown_html;

#[test]
fn wikilinks_test_1() {
    let original = r##"This is a [[WikiLink]].
"##;
    let expected = r##"<p>This is a <a href="WikiLink">WikiLink</a>.</p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_2() {
    let original = r##"This is a [[Main/WikiLink]].
"##;
    let expected = r##"<p>This is a <a href="Main/WikiLink">Main/WikiLink</a>.</p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_3() {
    let original = r##"This is [[Ambiguous]].

[Ambiguous]: https://example.com/
"##;
    let expected = r##"<p>This is <a href="Ambiguous">Ambiguous</a>.</p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_4() {
    let original = r##"[[squid] calamari is considered a delicacy](https://en.wikipedia.org/wiki/Squid)

[calamari [squid]](https://en.wikipedia.org/wiki/Squid)
"##;
    let expected = r##"<p>
<a href="https://en.wikipedia.org/wiki/Squid">[squid] calamari is considered a delicacy</a>
</p>
<p>
<a href="https://en.wikipedia.org/wiki/Squid">calamari [squid]</a>
</p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_5() {
    let original = r##"This is [also [[Ambiguous]]](https://example.com/).
"##;
    let expected = r##"<p>This is [also <a href="Ambiguous">Ambiguous</a>](https://example.com/).</p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_6() {
    let original = r##"<https://example.org/>

[[https://example.org/]]
"##;
    let expected = r##"<p><a href="https://example.org/">https://example.org/</a></p>
<p><a href="https://example.org/">https://example.org/</a></p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_7() {
    let original = r##"This is [[WikiLink|a pothole]].
"##;
    let expected = r##"<p>This is <a href="WikiLink">a pothole</a>.</p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_8() {
    let original = r##"This is a [[WikiLink/In/A/Directory|WikiLink]].
"##;
    let expected = r##"<p>This is a <a href="WikiLink/In/A/Directory">WikiLink</a>.</p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_9() {
    let original = r##"This is [[WikiLink|a **strong** pothole]].
"##;
    let expected = r##"<p>This is <a href="WikiLink">a <strong>strong</strong> pothole</a>.</p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_10() {
    let original = r##"This is a cute dog, linked to the page "WikiLink"

[[WikiLink|![dog](dog.png)]]
"##;
    let expected = r##"<p>This is a cute dog, linked to the page "WikiLink"</p>
<p>
<a href="WikiLink"><img src="dog.png" alt="dog" /></a>
</p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_11() {
    let original = r##"[[WikiLink|[[Fish]]]]
"##;
    let expected = r##"<p>[[WikiLink|<a href="Fish">Fish</a>]]</p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_12() {
    let original = r##"[[WikiLink|[cat](cat.html)]]
"##;
    let expected = r##"<p>[[WikiLink|<a href="cat.html">cat</a>]]</p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_13() {
    let original = r##"This is a cute dog.

![[dog.png]]
"##;
    let expected = r##"<p>This is a cute dog.</p>
<p>
<img src="dog.png" alt="dog.png" />
</p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_14() {
    let original = r##"![[dog.png|a cute dog]]
"##;
    let expected = r##"<p><img src="dog.png" alt="a cute dog" /></p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_15() {
    let original = r##"]] [[]] [[|]] [[|Symbol]] [[
"##;
    let expected = r##"<p>]] [[]] [[|]] [[|Symbol]] [[</p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_16() {
    let original = r##"[inline link]([[url]])
"##;
    let expected = r##"<p><a href="%5B%5Burl%5D%5D">inline link</a></p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_17() {
    let original = r##"[inline link]([[url)]]
"##;
    let expected = r##"<p><a href="%5B%5Burl">inline link</a>]]</p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_18() {
    let original = r##"`[[code]]`
"##;
    let expected = r##"<p><code>[[code]]</code></p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}

#[test]
fn wikilinks_test_19() {
    let original = r##"emphasis **cross [[over** here]]
"##;
    let expected = r##"<p>emphasis **cross <a href="over**%20here">over** here</a></p>
"##;

    test_markdown_html(original, expected, false, false, false, false, true);
}
