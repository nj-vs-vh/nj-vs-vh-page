`pyindigo` is a python interface for [`indigo`](https://github.com/indigo-astronomy/indigo)
astronomical software framework. `indigo` provides a common entrypoint and a set of
abstractions to control various devices like cameras, filter wheels, guides etc --- all in 
the form of C library. `pyindigo` is a python package that provides bindings to this library,
as well as some higher-level abstraction for common tasks.

**this is prototype-grade software and is not suitable for general use**

i developed the library primarily for [SIT telemetry monitoring tool](/projects/tunka-sit).
it is split into:
- "core" level --- a set of functions with direct bindings to `indigo`'s message bus via python
  C API
- "models" --- several classes modelling "entities" in the system (driver, device) that
  attempt to streamline some common tasks

unfortunately, with no access to a wide variety of astronomical hardware, i was unable to
properly test my library and build anything more sophisticated than a single camera going
**snap**. i would be happy to return to this someday, but for now this remains an extremely
limited and untested prototype.
