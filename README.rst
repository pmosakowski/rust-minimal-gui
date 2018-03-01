rust-minimal-gui
================

A GUI example in Rust with conrod

conrod basics
-------------

'conrod' is a UI library which facilitates creation and lifetime of widgets and translates them into primitives which can be rendered by various backends.

glium
-----

'glium' is the rendering backend which I have found the easiest to use on my first attempt. It uses OpenGl to render the UI primitives.

How did I come up with this code?
---------------------------------

This example is based mainly on the 'old_demo.rs' example from the conrod repository.