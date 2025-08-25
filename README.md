This is the second release version of my ray_tracing 
project! The next release will feature animations and
will be a major departure from the book: 

https://raytracing.github.io/books/RayTracingInOneWeekend.html

Here is a list of current features:
    - Path-tracing rendering
    - Some material and texture support for objects
    - Multithreading on the CPU
    - Minimal Obj file support
    - Spherical skybox mapping
    - Aliased scene elements for future editing support
Here are some planned features:
    - Offline animation rendering
    - Physics driven manipulation of scene elements
    - GPU edition
    - Programming language interface with static checking
    - A pre-vis app to edit scene programs visually before
        render
    - And probably a lot more!

TODO: Write a better how to use section here!

This program can be run with a few command line arguments
and one environment variable.

To change the place to look for assets from the default
you may define the environment variable `ASSET_DIR`. This
directory is where the program will search to load files
from. Alternatively it will default to searching through
a few directory levels for a directory called `assets`.

The program also takes a few command line arguments:

-f or --file    Specifies the file to render the image to
-t or --threads Specifies the number of threads to use      
                (defaults to the systems number of threads)
-w or --word    Selects a scene to render from the demo 
                scenes. These are as follows:
                    1. A scene from the end of the book
                    2. The same as above but with motion 
                        blur
                    3. A scene with two checkered spheres
                    4. A scene with the Utah teapot
                    5. A scene with a sphere wrapped with an
                        earth texture
                    6. A scene featuring a skybox

Try rendering these scenes or make your own! While this
may be clunky for the time being this will become easier in
the next release which will add a scripting language for
efficiently building scenes and movies. 

Finally this project is a work in progress and things
may not work exactly as expected. Feel free to open issues
and I will get to them as soon as possible!

The Sponza dragon scene has not been built yet in favor of
this detour. But when it is the asset is acquired from:

Sponza Dragon asset from Marko Dabrovic sourced at http://hdri.cgtechniques.com/~sponza/files/
