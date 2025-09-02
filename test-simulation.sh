#!/bin/bash

echo "=== Testing World Simulation ==="
echo ""

# Test basic queries
echo "1. Testing get_current_hour:"
docker-compose exec -T spacetimedb spacetime call worldsim get_current_hour 2>&1 | grep -v "WARNING"

echo ""
echo "2. Testing city summary for city 1:"
docker-compose exec -T spacetimedb sh -c 'spacetime call worldsim get_city_summary "1"' 2>&1 | grep -v "WARNING"

echo ""
echo "3. Toggling simulation:"
docker-compose exec -T spacetimedb spacetime call worldsim toggle_simulation 2>&1 | grep -v "WARNING"

echo ""
echo "4. Advancing 1 hour:"
docker-compose exec -T spacetimedb spacetime call worldsim tick_hour 2>&1 | grep -v "WARNING"

echo ""
echo "5. Getting current hour again:"
docker-compose exec -T spacetimedb spacetime call worldsim get_current_hour 2>&1 | grep -v "WARNING"

echo ""
echo "6. Getting individual 1 story (last 24 hours):"
docker-compose exec -T spacetimedb sh -c 'spacetime call worldsim get_individual_story "1" "24"' 2>&1 | grep -v "WARNING"

echo ""
echo "7. Creating a building:"
docker-compose exec -T spacetimedb sh -c 'spacetime call worldsim create_building "Home" "1"' 2>&1 | grep -v "WARNING"

echo ""
echo "8. Getting building 1 story:"
docker-compose exec -T spacetimedb sh -c 'spacetime call worldsim get_building_story "1"' 2>&1 | grep -v "WARNING"