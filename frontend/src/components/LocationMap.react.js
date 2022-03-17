import React from 'react';
import { Map, Marker, Popup, TileLayer, Polyline } from 'react-leaflet';
import { divIcon } from 'leaflet';

// Custom Icon for trip destinations
const locationIcon = divIcon({
  html: '<span class="icono-caretDown"/>',
  iconSize: [25, 25],
  iconAnchor: [17, 24],
  popupAnchor: [-3, -26]
});

const LocationMap = ({ onSelect, locations }) => {
  onSelect = onSelect || (() => ({}));

  // Show an empty map
  if (locations.length === 0) {
    return (
      <Map zoom={1.5} center={[30, 40]} style={{ height: 600 }}>
        <TileLayer
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
          attribution='&copy; <a href="http://osm.org/copyright">OpenStreetMap</a>'
        />
      </Map>
    );
  }

  const bounds = locations.map(getLatLong);
  return (
    <Map bounds={findBoundingBox(bounds)} style={{ height: 600 }}>
      <TileLayer
        url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
        attribution='&copy; <a href="http://osm.org/copyright">OpenStreetMap</a>'
      />
      <Polyline positions={locations.map(getLatLong)} color="#bf4e4e" />
      {locations.map(({ id, city, start_date, end_date }) => (
        <Marker
          key={id}
          position={[city.latitude, city.longitude]}
          icon={locationIcon}
        >
          <Popup
            onOpen={() => onSelect(id)}
            onClose={() => onSelect(null)}
            className="uk-hidden"
          />
        </Marker>
      ))}
    </Map>
  );
};

const getLatLong = ({ city }) => [city.latitude, city.longitude];

const findBoundingBox = bounds => [
  [
    Math.min.apply(null, bounds.map(b => b[0])) - 2,
    Math.min.apply(null, bounds.map(b => b[1])) - 2
  ],
  [
    Math.max.apply(null, bounds.map(b => b[0])) + 2,
    Math.max.apply(null, bounds.map(b => b[1])) + 2
  ]
];

export default LocationMap;
