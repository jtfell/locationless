//
// Script to insert Geonames DB into PG
//

const fs = require('fs');
const cityNames = require('./city-subset.json');
const countryContinentMapping = fs.readFileSync(`${__dirname}/country_continent.csv`, 'utf8')
  .split('\n')
  .map(row => row.split(','))
  .map(row => ({ country: row[0], continent: row[1] }));

const getContinent = (country, rest) => {
  const row = countryContinentMapping.find(row => row.country === country);
  return row.continent;
};


const cities = fs.readFileSync(`${__dirname}/cities15000.txt`, 'utf8')
  .split('\n')
  .map(city => city.split('\t'))
  .filter(city => city[0])
  .map(cityFields => ({
    id: cityFields[0],
    name: (cityFields[1] || '').replace("'", "''"),
    lat: parseFloat(cityFields[4]),
    long: parseFloat(cityFields[5]),
    countryCode: cityFields[8],
    continentCode: getContinent(cityFields[8], cityFields),
    population: parseInt(cityFields[14], 10)
  }))
  .filter(c => cityNames.indexOf(c.name) > -1);

const citiesSql = cities
  .filter(c => !cities.find(o =>
    o.name === c.name &&
    o.population > c.population
  ))
  .map(c => `(${c.id}, '${c.name}', ${c.lat}, ${c.long}, '${c.countryCode}', '${c.continentCode}', ${c.population})`)
  .join(',\n  ');

console.log(`
  BEGIN;

  ALTER TABLE ONLY trips
    DROP CONSTRAINT trips_city_fkey;

  DELETE FROM cities WHERE 1 = 1;

  INSERT INTO cities VALUES
  ${citiesSql};

  ALTER TABLE ONLY trips
    ADD CONSTRAINT trips_city_fkey FOREIGN KEY (city) REFERENCES cities(id);

  COMMIT;
`);
