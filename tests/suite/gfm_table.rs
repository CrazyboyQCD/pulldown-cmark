// This file is auto-generated by the build script
// Please, do not modify it manually

use super::test_markdown_html;

#[test]
fn gfm_table_test_1() {
    let original = r##"| foo | bar |
| --- | --- |
| baz | bim |
"##;
    let expected = r##"<table>
<thead>
<tr>
<th>foo</th>
<th>bar</th>
</tr>
</thead>
<tbody>
<tr>
<td>baz</td>
<td>bim</td>
</tr>
</tbody>
</table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn gfm_table_test_2() {
    let original = r##"| abc | defghi |
:-: | -----------:
bar | baz
"##;
    let expected = r##"<table>
<thead>
<tr>
<th style="text-align: center">abc</th>
<th style="text-align: right">defghi</th>
</tr>
</thead>
<tbody>
<tr>
<td style="text-align: center">bar</td>
<td style="text-align: right">baz</td>
</tr>
</tbody>
</table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn gfm_table_test_3() {
    let original = r##"| f\|oo  |
| ------ |
| b `\|` az |
| b **\|** im |
"##;
    let expected = r##"<table>
<thead>
<tr>
<th>f|oo</th>
</tr>
</thead>
<tbody>
<tr>
<td>b <code>\|</code> az</td>
</tr>
<tr>
<td>b <strong>|</strong> im</td>
</tr>
</tbody>
</table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn gfm_table_test_4() {
    let original = r##"| abc | def |
| --- | --- |
| bar | baz |
> bar
"##;
    let expected = r##"<table>
<thead>
<tr>
<th>abc</th>
<th>def</th>
</tr>
</thead>
<tbody>
<tr>
<td>bar</td>
<td>baz</td>
</tr>
</tbody>
</table>
<blockquote>
<p>bar</p>
</blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn gfm_table_test_5() {
    let original = r##"| abc | def |
| --- | --- |
| bar | baz |
bar

bar
"##;
    let expected = r##"<table>
<thead>
<tr>
<th>abc</th>
<th>def</th>
</tr>
</thead>
<tbody>
<tr>
<td>bar</td>
<td>baz</td>
</tr>
<tr>
<td>bar</td>
<td></td>
</tr>
</tbody>
</table>
<p>bar</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn gfm_table_test_6() {
    let original = r##"| abc | def |
| --- |
| bar |
"##;
    let expected = r##"<p>| abc | def |
| --- |
| bar |</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn gfm_table_test_7() {
    let original = r##"| abc | def |
| --- | --- |
| bar |
| bar | baz | boo |
"##;
    let expected = r##"<table>
<thead>
<tr>
<th>abc</th>
<th>def</th>
</tr>
</thead>
<tbody>
<tr>
<td>bar</td>
<td></td>
</tr>
<tr>
<td>bar</td>
<td>baz</td>
</tr>
</tbody>
</table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn gfm_table_test_8() {
    let original = r##"| abc | def |
| --- | --- |
"##;
    let expected = r##"<table>
<thead>
<tr>
<th>abc</th>
<th>def</th>
</tr>
</thead>
<tbody></tbody>
</table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn gfm_table_test_9() {
    let original = r##"Hello World
| abc | def |
| --- | --- |
| bar | baz |
"##;
    let expected = r##"<p>Hello World</p>
<table>
<thead>
<tr>
<th>abc</th>
<th>def</th>
</tr>
</thead>
<tbody>
<tr>
<td>bar</td>
<td>baz</td>
</tr>
</tbody>
</table>
"##;

    test_markdown_html(original, expected, false, false, false);
}
