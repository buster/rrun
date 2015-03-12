rrun
====
.. image:: https://travis-ci.org/buster/rrun.svg?branch=master
    :target: https://travis-ci.org/buster/rrun

minimalistic command launcher in rust similar to gmrun

.. image:: rrun.gif

Press Ctrl + Return to display output in window, else rrun will close after execution.

Set up rrun as command helper on Capslock
"""""""""""""""""""""""""""""""""""""""""

I have mapped the unused, needless CapsLock key to some other key and set up Gnome or whatever (i3wm in my case) to launch rrun on keypress.


My ~/.Xmodmap::

  remove Lock = Caps_Lock
  keysym Caps_Lock = XF86HomePage

Don't forget to run "xmodmap ~/.Xmodmap" after login.

The relevant parts of ~/.i3/config::

  bindsym XF86HomePage exec rrun
  for_window [title="rrun"] floating enable
  exec --no-startup-id xmodmap ~/.Xmodmap
