.. image:: https://travis-ci.org/buster/rrun.svg?branch=master
    :target: https://travis-ci.org/buster/rrun
.. image:: https://img.shields.io/crates/v/rrun.svg
    :target: https://crates.io/crates/rrun
.. image:: https://cdn.rawgit.com/syl20bnr/spacemacs/442d025779da2f62fc86c2082703697714db6514/assets/spacemacs-badge.svg
    :target: http://github.com/syl20bnr/spacemacs

rrun
====

rrun is a minimalistic command launcher in rust similar to gmrun.
It started as a playground to learn Rust, but since i use it all day for months now, it's probably useful for others as well.
It replaced gmrun and gnome-do on my laptop.
rrun has few features, it can do bash completion and run commands and that's it.
It will also append the commands being run to your bash history.

.. image:: rrun.gif

Dependencies
""""""""""""

GTK3.10+

Installation
""""""""""""

You have several options:

#. download a Debian package from https://github.com/buster/rrun/releases
#. install from crates.io with "cargo install rrun"
#. compile yourself with "cargo build"

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

How to build the package
""""""""""""""""""""""""

Creation of a cowbuilder image
''''''''''''''''''''''''''''''

The build process needs pbuilder/cowbuilder installed in debian (apt-get install cowbuilder pbuilder).
A Debian testing buid image can be created with::

  sudo cowbuilder --create --distribution testing

Eatmydata Installation
''''''''''''''''''''''

Install eatmydata (on build machine and in the image) to speeding up dpkg (from https://wiki.debian.org/cowbuilder ):

On the build machine::

  apt-get install eatmydata

In the build image::

  sudo cowbuilder --login --save
  apt-get install eatmydata

For eatmydata (>=82-2), add this /etc/pbuilderrc (on the build machine)::

  if [ -z "$LD_PRELOAD" ]; then
    LD_PRELOAD=libeatmydata.so
  else
    LD_PRELOAD="$LD_PRELOAD":libeatmydata.so
  fi

  export LD_PRELOAD

Package Build Process
'''''''''''''''''''''

The debian package can be built with the following commands:

- `make deb` just creates the .deb file without touching the changelog
- `make snapshot` creates a snapshot .deb without incrementing the version number (but updating the changelog)
- `make release` creates a new release and bumps the minor version number


Contributors
""""""""""""

@nightscape
@tshepang
