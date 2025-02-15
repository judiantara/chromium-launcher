# Chromium Launcher

This is a simple helper to launch a new instance of Chromium, with a separate profile, for a web application or website of your liking.

There's a myriad of programs on Linux and similar OSes, that can help you "install" a website to your desktop, as if it were a native app. There's no magic in this installation process, it's just a desktop file that specifies an icon, the name, the description, and of course, how to run the program. 

Such installers usually comes with a simplified browser to render the website. Sometimes, they come bundled with an outdated runtime, and would fail to render some websites.

So, instead of writing a browser, I just run Chromium with a separate profile for each application, with a special window class.

## Usage

`chromium-launcher <NAME> <URL>`

For example, to open whatsapp web in dedicated window :

```sh
chromium-launcher Whatsapp https://web.whatsapp.com
```
