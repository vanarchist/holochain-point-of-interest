const { connect } = require('@holochain/hc-web-client')
const L = require('leaflet');

var map;
var connection;

function onEachFeature(feature, layer) {
    if (feature.properties && feature.properties.name) {
        layer.bindPopup(feature.properties.name);
    }
}

function onMapClick(e) {
    var geojsonFeature = {
        "type": "Feature",
        "properties": {
            "name": "Point @ " + e.latlng.toString()
        },
        "geometry": {
            "type": "Point",
            "coordinates": [e.latlng.lng, e.latlng.lat]
        }};

    L.geoJSON(geojsonFeature, {
        onEachFeature: onEachFeature
    }).addTo(map);
    
    // save to holochain
    connection.then(({ callZome }) => {
    callZome('test-instance', 'point_of_interest', 'create_point_of_interest')({"entry": geojsonFeature}).then((result) => console.log(JSON.parse(result)))
    })
    
}

function addPoints(hcResult){
    console.log(hcResult)
    L.geoJSON(hcResult.Ok, {
        onEachFeature: onEachFeature
    }).addTo(map)
}


window.onload = function() {
    map = L.map('map').setView([34.594, -114.331], 13);
    map.on('click', onMapClick);

    L.tileLayer('https://{s}.tile.osm.org/{z}/{x}/{y}.png', {
        attribution: 'Map data &copy; <a href="https://openstreetmap.org">OpenStreetMap</a> contributors'
    }).addTo(map);

    // load saved points from holochain
    connection = connect("ws://localhost:8888")
    connection.then(({ callZome }) => {
    callZome('test-instance', 'point_of_interest', 'load_points')({"args":{}}).then((result) => addPoints(JSON.parse(result)))
})

}





