who doesn't love a good ol' [n-body system](https://en.wikipedia.org/wiki/N-body_problem)
simulation? i was mainly inspired by [this](https://t.me/random_three_body_problem)
([github](https://github.com/robolamp/3_body_problem_bot)) neat bot that generates
random **3**-body simulations. however cool, this seemed a little dry to me, not dramatic
enough, you know?

so my spin on the idea:
- start with the solar system with realistic orbital parameters (phases are random)
- generate disruptions:
  - some planets blow up, launching their pieces all over the place
  - heavy "visitor" bodies come and g r a v i t a t e
- let it run for a while while making some effort to focus the camera on interesting parts
  (the last part is the least developed, i want to make the camera much more cinematic in the future)
- make it more ~~~epic~~~ by adding sounds, specifically algorithmic sci-fi-ish music produced from 
  [Wreckage System](https://65daysofstatic.com/WreckageSystems_FAQ) project by 65daysofstatic (tl;dr:
  it's a [live broadcast](https://www.youtube.com/watch?v=z2Ox0Up7IUc) of generatively composed music).

here's an example of the simulation:

<script>
  // this is a script that generates telegram embedding script https://core.telegram.org/widgets/post
  const currentScript = document.currentScript;
  const tgScript = document.createElement("script");
  tgScript.setAttribute("async", "");
  tgScript.setAttribute("src","https://telegram.org/js/telegram-widget.js?22");
  tgScript.setAttribute("data-telegram-post", `solar_system_disruption/${1 + Math.floor(Math.random() * 1465)}`);
  tgScript.setAttribute("data-width", "100%");
  tgScript.setAttribute("data-color", "343638");
  tgScript.setAttribute("data-dask-color", "FFFFFF");
  currentScript.insertAdjacentElement("afterend", tgScript);
</script>

## technical details

- simulation uses `scipy`'s solver for matrix-formulated Newton's law
- animation is done in `matplotlib` because why not?
- sounds are added using `ffmpeg`
