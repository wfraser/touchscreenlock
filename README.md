# TouchscrenLock

This is a very simple win32 program which prevents touchscreen inputs from
having any effect while the program is in the foreground.

I use this on my Surface when showing videos to my toddler son so that he can
put his hands all over the screen without interrupting the video.

Use Alt-Tab to switch to another program or to quit it. This is fine for a
Surface where you can simply detach the keyboard, but probably not good enough
for a laptop, where keyboard inputs work just fine.

It also blocks mouse click inputs, which is fine for my use case. To make mouse
work but not touchscreen, I'd need to inject pointer events back into the OS or
something, but this isn't important for my use-case.
