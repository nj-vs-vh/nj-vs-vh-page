[Alvin Lucier](https://en.wikipedia.org/wiki/Alvin_Lucier) is one of my favorite composers ever, and
his seminal piece [I Am Sitting in a Room](https://www.youtube.com/watch?v=fAxHlLK3Oyk) lends itself
nicely for performance/recreation. so, over the years i've made not one but two different automation
tools for that:

## Matlab
[IAmSittingInARoom.m](media/IAmSittingInARoom.m)

back in the day, when i was using Matlab for research work, i discovered that it actually has an audio
processing toolkit with lots of nice features. one of them is a simple framework for VST plugin
development. so, i implemented a simple record-playback loop in the form of a plugin and ran it in
Matlab's standalone host, thus generating a progression of the piece.

here's a video i made with it, in which it's applied to a part of ["steamed hams"](https://knowyourmeme.com/memes/steamed-hams):

<iframe width="560" height="315" src="https://www.youtube.com/embed/SFVlHOtJHs8?si=hNQo8owvGNAE2Cdo" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

## Reaper Script

[i-am-sitting-in-a-room-automation.lua](media/i-am-sitting-in-a-room-automation.lua)

when i recently wanted to perform "I Am Sitting in a Room", i stopped using Matlab a long time ago.
so i decided to use Reaper, my DAW of choice, and use the opportunity a learn a bit about its
[scripting abilities](https://www.reaper.fm/sdk/reascript/reascript.php). turns out, they're great! not
much more to say there. the script is very simple, but still a bit more intelligent than Matlab one; it
- uses RMS compression to ensure a constant loudness regardless of recording input gain
- slowly increases recording length to contain reverb tails
- sets fade-in and fade-out to ensure clickless playback

also, not in the script, but my Reaper setup uses filtering to avoid amplifying a room's resonant
frequency that is too high or too low.

