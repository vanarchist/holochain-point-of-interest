# Holochain Point of Interest Demo

![Screenshot](https://i.imgur.com/roNcIml.jpg)

This is a demo holochain app that enables adding and displaying points of interest on a map. Leaflet and OpenStreetMap is used for the map display and the point of interest data is stored in a holochain DHT.

## Building
### Front End
The front end javascript code is managed in a node.js project. Browserify is used to generate a bundle.js file that is rerferenced in index.html. To generate the bundle.js file run the following command.
```
npm run build
```

### Back End
The backend rust code can be built using the holochain hc command as shown.
```
hc package
````
Due to some dependency problems with certain rust releases, the following version of rust was used.
```
rustc 1.34.0-nightly (f29b4fbd7 2019-01-31)
```

## Running
Startup the holochain node using the holochain hc command as shown.
```
hc run
```
Open index.html in your browser.

