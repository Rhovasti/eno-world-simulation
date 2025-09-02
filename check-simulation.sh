#!/bin/bash

echo "=== World Simulation Status ==="
echo ""
echo "Current Hour:"
docker-compose exec -T spacetimedb spacetime call worldsim get_current_hour 2>/dev/null

echo ""
echo "City Summary (ID: 1):"
docker-compose exec -T spacetimedb spacetime call worldsim get_city_summary "1" 2>/dev/null

echo ""
echo "Recent Events:"
docker-compose exec -T spacetimedb spacetime sql worldsim "SELECT * FROM NarrativeEvent ORDER BY timestamp DESC LIMIT 5" 2>/dev/null

echo ""
echo "Individual Status (sample):"
docker-compose exec -T spacetimedb spacetime sql worldsim "SELECT id, name, age, location_x, location_y FROM Individual LIMIT 5" 2>/dev/null

echo ""
echo "Building Status (sample):"
docker-compose exec -T spacetimedb spacetime sql worldsim "SELECT id, building_type, occupancy, max_occupancy FROM Building LIMIT 5" 2>/dev/null