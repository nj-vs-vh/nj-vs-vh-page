i was working on a large scientific codebase that required some environment variables
to be set and setup scripts to be run before doing anything. i wanted to automate this
environment activation, just like we do in `.bashrc`-like scripts, but inside visual studio code's
integrated terminal. after some time spent experimenting with hacks and workaround, i decided
to make this into a tiny VS Code extension.

it is not really usable by a large audience as it:
- probably has a lot of untested edge-cases
- likely constitutes a security vulnerability: if you get someone to checkout your repo
  with `activation.json` and they have the extension, you can make their vscode run
  arbitrary commands and scripts on terminal startup!
- is only limited to unix-like systems

anyway, it does the trick for me, solving a specific problem, and that's everything i wanted!
