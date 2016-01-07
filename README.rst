.. image:: https://travis-ci.org/buster/rrun.svg?branch=master
    :target: https://travis-ci.org/buster/rrun

rrun
====

rrun is a minimalistic command launcher in rust similar to gmrun.
It started as a playground to learn Rust, but since i use it all day for months now, it's probably useful for others as well.
It replaced gmrun and gnome-do on my laptop.
rrun has few features, it can do bash completion and run commands and that's it.
It will also append the commands being run to your bash history.

.. image:: rrun.gif

Installation
""""""""""""

Download a Debian package from https://github.com/buster/rrun/releases or compile yourself with "cargo build".

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
  
Contributors
""""""""""""

@nightscape
@tshepang
