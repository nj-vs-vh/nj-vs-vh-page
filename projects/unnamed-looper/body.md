at the time i was using MATLAB for science stuff, but also discovered that
it [can do](https://www.mathworks.com/help/audio/audio-plugin-creation-and-hosting.html)
VST plugins. at this point i decided to build an experimental VST looper plugin,
which in the end was pretty exhastive as far as my studio/live performance needs went:

- recording and overdubbing to an in-memory buffer, without clicks and digital artifacts
- playing back the loop with pitch/speed adjustment and with reverse mode
- glitcher, randomly changing loop's pitch, direction or playback point with some periodicity
- "condensing/anticondensing" mode, in which only the parts louder/quieter than a given
  threshold were played

i don't have access to MATLAB now, so i can't continue adding features, but maybe one day i
will rewrite it in another framework/language...
