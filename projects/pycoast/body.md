here's how it goes:

- [CORSIKA](https://www.iap.kit.edu/corsika/) --- COsmic Ray SImulations for KAscade
  --- is a ubiquitous Monte-Carlo tool for simulating Extensive Air Showers from
  cosmic rays in the atmosphere.
- COAST --- COrsika dAta accesS Tools --- is a C++ program/library to read CORSIKA's
  binary data format. the data can be many gigabytes in size, but conceptually it's
  a simple packet series. so the reader is essentially an iterator over simulated
  particles.
- finally, pyCOAST is a python interface for COAST, for those who wants to read out
  the particles without writing a dedicated C++ program.

at the core, this is just a simple [SWIG](https://swig.org/)-generated interface with some
quality-of-life wrapper classes on the python side. (after using SWIG once i don't want to
ever touch python C API with my hands, at least for trivial interfacing tasks!)

the usage examples for this library are in
[`carpet-scripts`](https://github.com/nj-vs-vh/pyCOAST/tree/main/carpet-scripts)
dir, which, as the name suggests, were used for Monte-Carlo data analysis in
Carpet-2 experiment (see e.g. [this article](https://arxiv.org/abs/2011.02452)).
  