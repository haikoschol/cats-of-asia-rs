function init_map(accessToken) {
    if (!document.map) {
        document.map = L.map('cattos');
        L.tileLayer(`https://api.mapbox.com/styles/v1/{id}/tiles/{z}/{x}/{y}?access_token=${accessToken}`, {
            maxZoom: 22,
            id: 'mapbox/streets-v11',
            tileSize: 512,
            zoomOffset: -1
        }).addTo(document.map);
    }
}

function remove_map() {
    if (document.map) {
        document.map.remove();
        delete document.map;
    }
}

function set_view(latitude, longitude, zoomLevel) {
    if (!document.map) {
        console.error('document.map not initialized');
        return;
    }

    document.map.setView([latitude, longitude], zoomLevel);
}

function add_marker(latitude, longitude, radius, randomized) {
    if (!document.map) {
        console.error("can't add marker without a map");
        return;
    }

    const color = randomized ? 'blue' : 'red';
    const circle = L.circle([latitude, longitude], {color: color, radius: radius});
    circle.addTo(document.map);
}