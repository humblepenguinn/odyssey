## Contributing

Thanks for your interest in contributing to odyssey!

Any form of contribution is accepted, be it bug fixes, implementing a new feature or even just fixing a small typo. The goal is to get the community involved as much as possible

pull requests for bug fixes and features will only be accepted if the approach has been discussed in an issue and a community memeber has been given the go-ahead to work on it

Please keep the following in mind at all times:

* Check existing issues to verify that the [`bug`](https://github.com/humblepenguinn/odyssey/labels/bug) or [`feature request`](https://github.com/humblepenguinn/odyssey/labels/feature%20request) has not already been submitted.
* Open an issue if things aren't working as expected.
* Open an issue to propose a significant change.
* Open a pull request to fix a bug.

* Open a pull request for any issue labelled [`help wanted`](https://github.com/humblepenguinn/odyssey/labels/help%20wanted), [`good first issue`](https://github.com/humblepenguinn/odyssey/labels/good%20first%20issue) or [`community`](https://github.com/humblepenguinn/odyssey/labels/community).

Please avoid:

* Opening pull requests for issues marked `needs-triage`, `needs-investigation`, or `blocked`.
* Opening pull requests for any issue marked `maintainers`. These issues require additional context from
  the maintainers/code owners and any external pull requests will not be accepted.

## Building the Blockchain
To build and run the Odyssey blockchain on your local machine, follow these simple steps:

### Clone the Project:

Start by cloning the `odyssey` repository to your computer. You can do this using `Git` by running the following command in your terminal:

```bash
git clone https://github.com/humblepenguinn/odyssey.git
```

### Install Rust:
Ensure that you have `Rust` installed on your system. If you haven't installed it yet, you can download and install it from the official website: https://www.rust-lang.org/


### Navigate to the Project Directory:
Change your current directory to the cloned project folder:

```bash
cd odyssey
```

### Build and Run:
Now, you can use Cargo, the Rust package manager, to build and run `odyssey`:

```bash
cargo run
```

Please note that since the project is still a work in progress and not production-ready, be cautious while using it and avoid deploying it in critical environments. Feel free to explore the code and contribute to the project to make it more robust and reliable.

## Tests
Tests have not yet been written for `odyssey`, so maybe thats something you could create a pull request for?

## Submitting a pull request

1. Create a new branch: `git checkout -b my-branch-name`
2. Make your change
3. Run `cargo fmt --all --check`
4. Run `cargo clippy --fix --all-features`
5. Submit a pull request

Contributions to this project are released to the public under the project's open source licenses,
the [MIT License](LICENSE-MIT) and the [Apache License](LICENSE-APACHE)

Please note that this project adheres to a [Contributor Code of Conduct][code-of-conduct]. By participating in this project you agree to abide by its terms.

## Design guidelines
Let your imagination run wild and suggest amazing ideas

There isn't any strict design guidelines yet. I am still working on that, so for now the project is open to any kind of change

## Resources

- [How to Contribute to Open Source][]
- [Using Pull Requests][]
- [GitHub Help][]



[code-of-conduct]: ./CODE_OF_CONDUCT.md
[How to Contribute to Open Source]: https://opensource.guide/how-to-contribute/
[Using Pull Requests]: https://docs.github.com/en/free-pro-team@latest/github/collaborating-with-issues-and-pull-requests/about-pull-requests
[GitHub Help]: https://docs.github.com/

