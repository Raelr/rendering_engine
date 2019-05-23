# rendering_engine

This is a small rendering engine which i've decided to build out of pure curiosity. This is part of a process which I hope will lead to me learning to create a game engine from scratch. 

The project is following the great OpenGL tutorial by <b><i>Joey DeVries</i></b> (https://learnopengl.com/Getting-started/Hello-Triangle), and attempts to follow his tutorials (add extend them) to best suit a game engine. This is largely experimental, and is a rugh interpretation of how I would think rendering would be tied into an engine. 

<h2>Components</h2>

So far, the engine is comprised of a number of basic scripts and classes. These are broken down as follows:

* <b>SDL2_rust:</b> I use SDL2 to handle input and manage windows. 

* <b>gl_rs:</b> A wrapper which allows the OpenGL API to be accessed through rust code.

<h2>Features</h2>

So far, the engine is incredibly rough. Its most significant classes are listed as follows:

* <b>application: </b>handles the main event loop and calls the renderer's main functions.

* <b>render_application: </b>handles basic GPU indexing, buffering and vertex array management.

* <b>renderer_tests: </b>contains methods for rendering certain objects and shapes. These shapes 

* <b>shader_program: </b>combines multiple shaders into a single object which can be used to determine how an object is rendered.

* <b>shader: </b>takes in a vertex or fragment shader and creates a new shader object containing that shader.
