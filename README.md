<div align="center">
  <h1>Flagger 🚩</h1>
  <p>
    <b>
      Experimental feature flag platform.
    </b>
  </p>
  <sub>
    Built with Rust and TypeScript.
  </sub>
</div>

## Abstract

Many **production-ready** tools for feature flags are out there; **this is not
one of them**.

Behind this project, there is a pure personal exploration. First, I want to
practice my Rust skills, making the codebase as modular as possible. The main
focus of this repository is to deliver a fast feature flags tool for the web,
but it is also possible to embed it in desktop or mobile applications.
The admin UI is built in TypeScript with Next.js, Tailwind CSS and Preline; it
aims to be simple but flexible.
The platform is distributed via Docker, exposing two CLI tools that run on
dependency-free Alpine Linux.

As for [fast-cli](https://github.com/fdionisi/fast-cli), this it's also an
opportunity to exercise my Rust and test [Zed](https://zed.dev/) editor.

## License

_Flagger_ is distributed under the terms of the MIT license.

See [LICENSE](LICENSE) for details.