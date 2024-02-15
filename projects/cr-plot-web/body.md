
here's the original cosmic rays spectrum plot by [Carmelo Evoli](https://carmeloevoli.github.io/):

![original CR plot by Carmelo Evoli](https://raw.githubusercontent.com/carmeloevoli/The_CR_Spectrum/master/plots/The_CR_Spectrum_2023.png)

it is very nice, but a little bit crowded (and it misses some data still). there is
a natural upper limit to how much information can fit into a static PDF. i had long been
thinking about browser-based visualizations for scientific data presentation, and decided
to give it a go. basically, i just compiled all the plot's data to a JSON document and wrote some
Javascript to render it using [Plotly](https://plotly.com/javascript/).

[here](/cosmic-rays-spectrum)'s the plot. the nice thing is that it is completely static and
self-contained -- libraries are loaded from CDN.

the only true innovation with respect to the original static plot is the ability to
change the power of E the intensity is multiplied by. this allows one to dynamically make the plot
informative in different energy ranges, e.g. to set the power to 3 and zoom to UHECR region, than
change it to 2.8 and see near-knee features as clear as possible.
