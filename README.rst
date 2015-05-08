rrun
====

Requires GTK 3.10+

( Build fails since Travis uses Ubuntu 12.04 with no GTK3.10 available: https://github.com/travis-ci/travis-ci/issues/2046  )#

.. image:: https://travis-ci.org/buster/rrun.svg?branch=master
    :target: https://travis-ci.org/buster/rrun

minimalistic command launcher in rust similar to gmrun

.. image:: rrun.gif

Usage
"""""

- enter a command and press Return to execute it
- press TAB for tab completion of available commands
- Press Ctrl + Return to display the command output in the text field

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
