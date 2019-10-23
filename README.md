# Mortgage Munger

> When your bank gives you only garbage reports online and you need the data in
> a spreadsheet

|         |                                      |
| ------: | ------------------------------------ |
|      CI | [![CI Status][badge-ci-overall]][ci] |
| License | [![License][badge-license]][license] |

**Table of Contents**

<!-- toc -->

- [About](#about)
- [Usage](#usage)
- [Issues](#issues)
- [Contributing](#contributing)
- [Authors](#authors)
- [License](#license)

<!-- tocstop -->

## About

This program takes multiple pastes to `stdin` of terrible Online Banking output
report pages that look like this:

```text
Payment Date: 	Jul 20, 2018 	Jul 13, 2018 	Jul 6, 2018 	Jun 29, 2018 	Jun 22, 2018
Principal Paid: 	$123.45		...
Interest Paid: 	$67.89		...
Taxes Paid: 	$12.34		...
Total Paid: 	$567.89		...
```

and output a CSV file that looks like this:

```text
Payment Date,Principal Paid,Interest Paid,Taxes Paid,Total Paid
2018-07-20,123.45,67.89,12.34,567.89
...
```

## Usage

You run the program:

```console
$ cargo run
```

and paste each report page in, followed with a newline and `Ctrl+D` to send an
`eof` to the `stdin` stream. Repeat this process for each page and when
finished, end with a single newline and `Ctrl+D`. The CSV report will be output
to `stdout`.

## Issues

If you have any problems with or questions about this project, then I really do
feel for you as you might have the same problems as this author. The author
apologies.

## Contributing

I advise that you do not contribute to this project. It should **not** need to
exist and comes up every year this author does his year end taxes. Consider
contributing to other projects maintained by the author or donate your time to a
worthy cause.

## Authors

Angrily created and reluctantly maintained by [Fletcher Nichol][fnichol]
(<fnichol@nichol.ca>).

## License

Licensed under the Mozilla Public License Version 2.0 ([LICENSE.txt][license]).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the MPL-2.0 license, shall be
licensed as above, without any additional terms or conditions.

[badge-ci-overall]:
  https://img.shields.io/cirrus/github/fnichol/mortgage-munger.svg?style=flat-square
[badge-license]:
  https://img.shields.io/badge/License-MPL%202.0-blue.svg?style=flat-square
[ci]: https://cirrus-ci.com/github/fnichol/mortgage-munger
[fnichol]: https://github.com/fnichol
[license]: https://github.com/fnichol/mortgage-munger/blob/master/LICENSE.txt
