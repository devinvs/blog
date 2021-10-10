How I Built This|how-i-built-this|2021-10-09

# How I Built This
*Published October 9, 2021*

---

Welcome to the first "real" post on this blog. I call it the first real post, but in another way, it could also be
classified as part two to the introduction. Either way, the topic I want to write about today is the methods and tools I
used to create this blog site. I considered taking you through the step-by-step process, highlighting pitfalls you can
avoid and best practices, but I found the entire article to be rather tedious after the first couple of drafts. This
doesn't mean I won't go into some details, but just note you are only seeing the final result and not any of the
revisions that came before it.

## The Plan and The Process

From the very start, I knew that I wanted this blog to be as minimalistic as possible. There is a certain aesthetic
quality that derives from minimalistic design that is pleasing to me. And I don't just mean this in a visual design
sense, but also the writing process and technology stack as a whole. From the inception of an article to serving it to
the reader, I wanted the process to be as simple as possible. Thus every aspect of this site is touched by the
principles of minimalism.

### Writing Articles

I don't use any fancy tools for my writing process. I prefer to write distraction-free, so I can focus on the creative
aspect of writing, separating the editing and error checking for later. This allows me to focus on getting my thoughts
down on paper before worrying about correctness. My software of choice, then, for writing is also minimal; I use a great
piece of open-source software called [ghostwriter](https://wereturtle.github.io/ghostwriter/). It specializes in
removing distractions, dimming other pieces of text that you aren't currently editing, and taking up the entire screen;
it gets out of the way and just lets you write.
Additionally, ghostwriter supports several markdown flavors, a text formatting language, which is especially useful for
this blog. The other piece of software that I use and currently am using is [vim](https://www.vim.org/). Vim doesn't
have all the fancy distraction removing technologies that ghostwriter has, but it is also dead simple to use and doesn't
do much by default. When I am editing code, it can provide linting and autocomplete, but inside a text file, it leaves
me to write without distractions.

This does mean that I don't have any syntax or grammatical error checking, and I definitely make a decent number of
typos and mistakes on my first drafts. However, once I feel that I have fully fleshed out the creative aspect of the
article, I then copy the contents to a more advanced text editing tool that does include spell check and other nifty
error-catching features. As of late, I've been using Grammarly and have been pretty satisfied with the results.

As hinted at above, I am writing my articles in markdown. If you've never heard of markdown before, that's okay. You've
probably used it in other software such as Slack or Discord to bold text in your messages or add a little bit of text
formatting. For instance, the string `**bolded text**` would be rendered as **bolded text**, or `*italicized text*` would show
as *italicized text*. I'm not going to go into depth on how markdown works, but if you are curious, you can check out [this
resource here.](https://guides.github.com/features/mastering-markdown/)

The last problem I had to solve as far as article content goes was article metadata, such as the title of the article to
display on the home page, the published date, or the slug to put in the link (look at the URL for this article, the slug
is `how-i-built-this`). I have to admit, my solution for this is a little bit of a hack, but it's a hack that works well
enough for me at the moment. All metadata is stored in the first line of the markdown file. In this file the metadata
line is:

```md
How I Built This|how-i-built-this|2021-10-09
```

It is a simple but effective solution that I think should last me quite a while.

### Storing Articles

All of the writing I do is inside markdown files, which must be stored and tracked somewhere. So I opted for a solution
that I have always been curious to try; I track all of my writing inside a git repository. [Git](https://git-scm.com/) provides many features for managing different versions of code and syncing between local and remote versions. This was perfect for my use case, as I could use commits to track different edits of my articles and then push the final versions to the remote to be published. Again a straightforward solution, but pleasingly so.

### Converting Articles

But the article you are reading right now isn't displayed in markdown. Instead, the markdown needs to be converted into
a language that the browser can understand, namely HTML. This is where the bulk of my efforts went, creating a
dead-simple static site generator to convert my markdown files into HTML files. My language of choice for this endeavor
was [Rust](https://www.rust-lang.org/), a low-level language that prioritizes memory safety and performance. I've done
quite a few projects with Rust, and this probably won't be the last time you hear about it. Anyways, the static site
generator has to do the following tasks:

1. Read and parse all the markdown files.
2. Convert the parsed markdown into HTML.
3. Insert the generated HTML into the blog template, writing the new file to disk.
4. Generate particular files such as the latest blog entry, the archive page, and the home page based on the metadata of the markdown blogs.

I'm going to omit the source code for the static site generator, but you can check it out [on my GitHub if you are interested.](https://github.com/DevinVS/blog)

The HTML templates and files are also very minimal. Every page shares the same fonts, the same stylesheet, and
effectively the same layout. I used semantic HTML elements to convey meaning without unnecessary nesting and focused on
making sure that my site was readable in all conditions. This meant testing on many screen sizes and resolutions, making
sure that the nav wasn't overlapping or the text wasn't growing too large. Eventually, I created a set of rules to make
the content easily usable, from ultrawide monitors to screen readers.

Regrettably, I did have to include some javascript on my site. My first revision of this template system did not include
this, but there were 2 problems that only javascript could solve on my site:

1. Automatically setting the copyright text at the bottom of the page to the current year
2. Code Syntax Highlighting

I found that these two issues, especially the code highlighting, ended up being deal breakers for me, so I did have to
include some javascript. However, even though javascript is a part of this site, all content and interactions are
entirely functional and accessible in an environment with javascript disabled, the code just might be less readable than
you would prefer.

### Publishing Articles

Once I have the articles ready for the browser, I need a way to get them on the server. Luckily, Github provides a neat
service they call Actions which allows you to execute jobs every time you sync your local copy with the copy on their
servers. What this means is that every time I push my local files to the server, Github automatically does 3 things:

1. Compiles my static site generator
2. Runs the static site generator on the markdown blog posts
3. Copies the final public HTML files to the server via ssh

This makes the workflow from my perspective dead simple: Once I'm ready to publish, I push, and after a few minutes, it
is done. Admittedly, this limits the functionality; I can't schedule posts, and it is harder to delete a post once it is
published. Luckily for my purposes, these are not necessary, so my simple solution will do just fine.

### Serving Articles

Finally, I needed a way to deliver these freshly generated HTML files to you, the reader. I rent out a server instance from [Linode](https://www.linode.com/) running [Alpine Linux](https://www.alpinelinux.org/), an extremely minimalistic Linux distribution that suits my purposes wonderfully. After
quickly installing and configuring [Nginx](https://nginx.org/en/) as a webserver and setting up [LetsEncrypt](https://letsencrypt.org/) to support https, I was ready to serve my site.

### Conclusion

This concludes my rushed explanation of how I built this blog. Honestly, there isn't much to talk about because, by
design, there isn't much there. I used simple tools to create a simple process. There is something beautiful about not
using anything more complicated than you need, just the bare necessities of technology to fulfill your purpose. I don't
have any fancy javascript frameworks or a powerful content management system, but what I built is good enough for me,
and I am satisfied.
