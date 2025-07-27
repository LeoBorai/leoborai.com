---
title: "Configure your SvelteKit Web Application to be a PWA"
description: "Guide on how to setup a PWA in your SvelteKit web application."
categories: [svelte, pwa, sveltekit, web-development, javascript]
icon: svelte
date: 2024-06-07
preview_image_url: "/images/preview/sveltekit-pwa.png"
published: true
---

If you have an existent SvelteKit Web Application, you can easily convert it into a Progressive Web Application (PWA).

SvelteKit supports part of whats required to turn the project into PWA out of the box. In my own site I have already done it,
by following this SvelteKit documentation.

https://kit.svelte.dev/docs/service-workers

The Service Worker allows me to cache static assets so they are available even offline and the second time
you visit my site you will notice it loads faster!

But theres more to it to have your SvelteKit Web Application to be a full PWA. Theres lots of benefits
to have PWAs such as adding your site to the home screen, push notifications, offline support and more.

## Enter the `manifest.json`

The `manifest.json` file is the configuration file that allows us to introduce our Web Application to the
browser as a PWA. Here configurations such as the name of the app, the icon, the theme color, the
background color and the start URL are defined.

You can read more about the `manifest.json` file here:

https://developer.mozilla.org/en-US/docs/Web/Manifest

Using the same base file as from MDN, you can create your own `manifest.json`, for it to work with
SvelteKit you must create it under `static` directory from your SvelteKit project.

```json name="/static/manifest.json"
{
  "name": "My SvelteKit PWA",
  "short_name": "My PWA",
  "start_url": "/",
  "display": "standalone",
  "background_color": "#fff",
  "theme_color": "#000",
  "icons": [
    {
      "src": "/icon-192x192.png",
      "sizes": "192x192",
      "type": "image/png"
    },
    {
      "src": "/icon-512x512.png",
      "sizes": "512x512",
      "type": "image/png"
    }
  ]
}
```

You will also need to add the icons to the `static` directory, in this case we introduce two icons
one for `192x192` and another for `512x512`, these should match names and sizes from the `manifest.json`.

Both icons should be added to the `static` directory as well.

In order for SvelteKit to load this file on page load, you must add a `link` tag to the `head` of your
`src/routes/+layout.svelte` file.

```html
<link rel="manifest" crossorigin="use-credentials" href="manifest.json" />
```

## Creating The Service Worker

As mentioned before, SvelteKit has a pre-defined Service Worker location on `/src/service-worker.ts`.
SvelteKit will automatically register a Service Worker for you if this file is present.

You can use my service worker definition as a base for your own, it will cache the static assets on load.

https://github.com/LeoBorai/leoborai.com/commit/f77c29d329b355205da2c26b322be5f1317ce940#diff-0aba05b7e68a2bb0e8cebbd6297fafc218eaeff08345a12bd379181c4a88e454

Remember to change the [`CACHE_KEY`][1] to something unique to your project.

## Build and Test

If you have followed all the steps correctly, you should be able to build your SvelteKit project and
test it locally.

Use the following command to build your project:

```bash
npm run build
```

Service Workers nor PWA's are present on development environments, so you must test the build version of your
project. To serve this version we just builded you must use `preview` script from SvelteKit.

```bash
npm run preview
```

You should be able to see an icon in the browser's top bar that indicates your site is a PWA and that you can
install it.

## Conclusion

PWAs are powerful solutions to help users visit your site often, with improved load times, a quick access icon
which results familiar to mobile users and even updates/refresh procedures which allows you to deliver a better UX.

[1]: https://github.com/LeoBorai/leoborai.com/commit/f77c29d329b355205da2c26b322be5f1317ce940#diff-0aba05b7e68a2bb0e8cebbd6297fafc218eaeff08345a12bd379181c4a88e454R9
