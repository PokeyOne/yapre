# Yet Another Pokey Rendering Engine (in Rust This Time)

![gpl 3 badge](https://img.shields.io/badge/license-GPL%203.0-blue)
![repo size](https://img.shields.io/github/repo-size/PokeyOne/yapre)
![lines of code](https://img.shields.io/badge/lines%20of%20rust-2286-informational)

I think this is the 5th time (?) that I have started working on a ray-tracing
rendering engine. Hopefully this time with more follow through as Rust makes
things easier to get setup and running with fewer errors along the way.

This project is currently in its early stages of development. Currently, it
can only ray cast with no shadows, reflections, or refraction. Here is an
example of it rendering a single 3D diamond shape from an orthographic view.

![test diamond](readme_resources/test_diamond.png)

And now it can render animations:
![first animation](readme_resources/first_animation.mov)

