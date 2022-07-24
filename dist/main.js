var app = require('app');  // Module to control application life.
var BrowserWindow = require('browser-window');  // Module to create native browser window.
var fs = require('fs');

// Keep a global reference of the window object, if you don't, the window will
// be closed automatically when the javascript object is GCed.
var mainWindow = null;

// Quit when all windows are closed.
app.on('window-all-closed', function() {
  if (process.platform != 'darwin')
    app.quit();
});

// This method will be called when Electron has done everything
// initialization and ready for creating browser windows.
app.on('ready', function() {
  // Create the browser window.
  mainWindow = new BrowserWindow({
    width: 420,
    height: 600,
    'use-content-size': true,
    'node-integration': false,
    title: 'Messenger',
    icon: __dirname + '/icon.png'
});

  // and load the index.html of the app.
  //mainWindow.loadUrl('file://' + __dirname + '/index.html');
  mainWindow.loadUrl('https://www.messenger.com');
  mainWindow.webContents.on('dom-ready', function() {
    fs.readFile(__dirname + '/style.css', 'utf8', function(error, data) {
      mainWindow.webContents.insertCSS(data);
    });

    fs.readFile(__dirname + '/webview.js', 'utf8', function(error, data) {
      mainWindow.webContents.executeJavaScript(data);
    });
  });

  mainWindow.webContents.on('new-window', function(event, url, frameName) {
    console.log('new window', frameName);
    event.preventDefault();

    var win = new BrowserWindow({
      width: 500,
      height: 500,
      'node-integration': false
    });

    win.loadUrl('https://www.messenger.com');

    win.webContents.on('dom-ready', function() {
      console.log('dom ready');

      fs.readFile(__dirname + '/chat.css', 'utf8', function(error, data) {
        win.webContents.insertCSS(data);
      });

      win.webContents.executeJavaScript('document.querySelector(\'a[data-reactid*="' + frameName + '"]\').click();');
    });
  });

  // Emitted when the window is closed.
  mainWindow.on('closed', function() {
    // Dereference the window object, usually you would store windows
    // in an array if your app supports multi windows, this is the time
    // when you should delete the corresponding element.
    mainWindow = null;
  });
});
