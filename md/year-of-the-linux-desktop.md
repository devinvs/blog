Year of the Linux Desktop|year-of-the-linux-desktop|2021-11-02

# Year of the Linux Desktop

*Published November 2, 2021*

---

I have used Linux for most of my computing life, but I'm also relatively young, having just turned 20. I absolutely love
it, and it is in large part responsible for honing in what technical skills I have today. I'll admit that by some standards, I
am still a newbie to the Linux world, having only started using it 6 years ago while dual-booting Windows and only fully
deleted Windows 10 a year and a half ago. I have never modified or contribued code to the Linux kernel. I have made
mistakes that lost me entire installs. I am neither the most experienced or the least experienced in any of the things I
will talk about today.

With all that said, right now is likely the most exciting time to be a Linux user. Wayland is finally becoming more
mainstream with chromium (and thus electron) having support for it, Valve's Proton has made gaming on Linux not only
possible but in a lot of cases painless, and with the Steam Deck soon to arrive proper Linux desktop gaming could become
mainstream.

In such times a certain phrase is tossed around, the "Year of the Linux Desktop". A quick search for it on DuckDuckGo
reveals countless articles along the lines of "2021 is the Year of Linux on the Desktop", "Year of the Linux Desktop
2021", and "We could still have a 'Year of the Linux Desktop'". This phrase has become such a cliche over the years that
now it is used to mock the Linux community for misplaced optimism in Linux adoption. Linux failed to catch on during the
last "year of the Linux desktop", so why is this year any different?

I want to take some time and think seriously about Linux, it's strengths, it's flaws, what it aspires to be, what it
fails to be, and how it could someday become mainstream. We'll take this journey in parts:

1. What counts as a Linux Desktop?
2. What is good and bad about Linux Desktops?
4. Who is using it?
5. Why hasn't it become mainstream yet?
6. How could it become mainstream?
7. Should it become mainstream?

## What counts as a "Linux Desktop"?

This question is harder than it first seems. Linux has crept its way into most modern computing devices in one way or
another, so drawing the line can be *tricky*. For instance, Windows now has support for running gui apps via the Windows
Subsytem for Linux, does that count as using the Linux Desktop? Or Google's chromebooks, which run a flavor of gentoo
under the hood, do they count? What about Android smart phones which use a heavily modified Linux kernel?

Beyond even strange integrations of Linux into other products, which Linux desktop are we talking about? Default Gnome 40 or my
specially handcrafted sway build?

For this article we are first going to limit our scope to desktop computers booting a Linux kernel. This means that WSL
and Android are out of the discussion. I want to limit the conversation to what is possible on popular Linux
distributions, such as Ubuntu, Fedora, or Manjaro. This is mainly because this is what the Linux community refers to as
the Linux Desktop, which usually never includes chromebooks (though in their current state their is no particular reason
to exclude them).

So for this article I'm going to focus on 4 different environments:

- My current Arch Linux Setup
- My Friend's Manjaro Setup
- Default Fedora Install
- Default Ubuntu Install

Between these 4 I think we cover a lot of ground across the Linux community that will be useful for our analysis later.

## My Setup (Sway)

![My Sway Setup](/assets/images/sway_desktop_full.png)

I am a minimalist. Hopefully by the design of this site and my views on technology you can see this, and the same is
true for my Linux desktop. Now I am not as minimal as is theoretically possible, I still like my computer to look
colorful and clean, and in fact go to great lengths to do so. 

Every detail has to be correct and every program has to comply. I have a somewhat strict colorscheme that every app
follows and in the cases where I have enough control, such as firefox, I even have a custom layout that I prefer over
teh default. It took work to get here. For reference, here is what a default sway configuration looks like:

![Default Sway](https://i1.wp.com/itsfoss.com/wp-content/uploads/2019/03/sway-wm.jpg?w=800&ssl=1)

### What's Good

I have complete control over every aspect of my environment. Every shortcut is defined in a config file that I can edit.
Every piece is modular and easily swappable with a different program. I can easily open, arrange, and close windows
without ever touching the mouse. It's mine.

### What's Bad

Pretty much everything about this desktop is unsuitable for almost everyone. My opinionated configs around special
colors and layouts make sense to me, but not everyone. Strangely, most people actually want to use their mouse to
interact with their computer, or even more strange, a touchscreen. When I show most people how I use my computer, they
usually aren't wonderstruck by how elegant and simple it is, instead gawking at me as if I've lost my mind. Surely this
can't be the future of the Linux desktop.

