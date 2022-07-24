# Electron Messenger
Facebook Messenger as a desktop app using Electron. This project converts https://www.messenger.com to a desktop app using [Electron](http://electron.atom.io/) to render the page in a native app. Basically the same thing that [Messenger for Desktop](http://messengerfordesktop.com/) does, but using Electron instead of NW.js.

### Install
Install Electron:

    npm install electron-prebuilt -g

### Run
Run the following command:

    electron electron-messenger

where `electron-messenger` is the name of the folder.

### Dev Notes
For now, this is just a spike. The messenger page is rendered directly into the Electron app using `BrowserWindow.loadUrl()`. Electron is more permissive than a browser, so we can do stuff like execute JS and add CSS on a page that's in another domain. We leverage this in order to hide elements and add click handlers to mimic an actual desktop app with a buddy list and multiple chat windows. Double-clicking on a buddy list will open a chat with that buddy in a new window.

Note that the buddy list and chat window are not just a buddy list and just a chat window; it's the entire messenger.com page with certain elements hidden through CSS. This means that Electron will spawn a new sandbox thread for each new window and load messenger.com into each window, hiding the appropriate elements depending on the window type. Each new window will take around 100MB of RAM, so this method is not performant nor scalable. You can use just one window (basically messenger.com wrapped in a desktop app), but I wanted to test how multi-windows would work with Electron.

Messenger.com is written in ReactJS and uses what looks like machine-generated class names. The class names seem to be the same for every instance, but there's no guarantee that it is. Regardless, this is the only way we can target the elements for showing/hiding, and Messenger for Desktop uses the same thing for its theming, so we'll do the same.
