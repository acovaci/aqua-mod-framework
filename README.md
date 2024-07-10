# Aqua Mod Framework for Factorio

Factorio mods with static type checking! Aqua is a mod framework for Factorio that aims to make
creating mods easier and more enjoyable. It provides a CLI tool to help you create and manage your
mods, so that you can focus on the fun part.

Aqua also includes a batteries-included development setup that uses [Teal](https://www.github.com/teal-language/tl)
to provide static type checking for your mods. This helps you catch bugs early and write more
reliable code.

Finally, Aqua introduces a new paradigm for modpacks, allowing you to dynamically define modpacks
based on the mods in the current project. This makes it easier to manage dependencies and
compatibility between mods.

## Installation

This project is still in development, so it is not yet ready for use. However, you can install it
from the GitHub repository if you want to try it out. I will document the installation process once
someone asks for it :)

## Goals

1. **Easy to use**: The framework is easy to use, with a minimum of boilerplate code and 
   configuration.
2. **Static Type Checking**: The development setup this framework provides uses
   [Teal](https://github.com/teal-language/tl) to provide static type checking.
3. **Modpack Support**: Aqua introduces a new paradigm for modpacks, allowing you to dynamically
   define modpacks based on the mods in the current project.
5. **No Watermarking**: I am not adding any needless notices to your files. The only thing that will
   be added to the files is a comment containing the framework version. This is to help with
   keeping track of what version of the framework was used to create the mod. If you are not
   comfortable with this, you can remove it without any issues. No questions asked.
6. **Try not to break the VSCode Mod Development Extension** (but I reserve the right to change my
   mind on this one): I am trying to make sure that the VSCode Mod Development Extension still works
    with this framework, but I am not going to guarantee that it will always work.

## Features

This is a list of the planned features. Check the GitHub Project for the current status of each
feature.

* **Command Line Interface**: Aqua provides a CLI tool to help you create and manage your mods.
* **Diverse Modding Setup**: Aqua supports three different kinds of development setup:
    * **Single Mod**: Your project contains a single, standalone mod.
    * **Multiple Mods**: Your project contains multiple mods that are loosely related.
    * **Modpack**: Your project contains multiple mods that are tightly related. The framework will
      automatically generate an extra mod for you, that will strongly depend on the other mods,
      keeping the versions locked. This should ensure that your users will always be using a
      single set of versions for each of your modpack versions.
* **Static Type Checking**: Aqua uses Teal to provide static type checking for your mods. Aqua also
  provides a type definition library for Factorio, to help you write more reliable code.
* **YAML Configuration**: Aqua uses YAML configuration files to define your mods, modpacks, and
  other settings. This way, it can auto-populate some of the fields for you, leaving you with only
  the important stuff to fill in.
* **Strongly Opinionated**: Aqua is strongly opinionated, meaning that it will make a lot of
  decisions for you. This is to help you get started quickly, and to make sure that your mods are
  following best practices.
* **Versioning Tools**: Aqua provides tools to help you manage your mod versions, easily being able
  to bump the version number, and to generate a changelog.

## Can I use this?

Absolutely! The framework is free to use and modify. However, I will bring something to your
attention: From my experience, Factorio mods tend to not be open source. A lot of them are source
available, but with a license that prohibits modification and redistribution. If you are planning on
using this framework to create a mod, due to the Affero GNU General Public License v3.0, you will
have to make your mod open source and licensed under the same license. If you are not comfortable
with this, I would recommend not using this framework.

If you are planning on using this framework to create a mod, I would love to hear about it! Feel
free to share your project with me, post it in the *Discussions* tab, or send me a message. If you
want to share your project with the world, I would be happy to include it in a list of projects that
use this framework. But I will not do so without your permission and request.

## Contributing

I am more than happy to accept contributions to this project, in any form.

You can help out this project in one of the following ways:
* Reporting bugs via the *Issues* tab.
* Sharing ideas, suggestions, or feedback via the *Discussions* tab.
* Contributing code, documentation, or other resources by forking the repository and creating a pull
  request.
* Supporting the project by starring it or sharing it with others.
* If you can afford it, I am accepting donations via GitHub Sponsors.

## License

This project is licensed under the Affero GNU General Public License v3.0. You can find the full
license text in the `LICENSE` file.
