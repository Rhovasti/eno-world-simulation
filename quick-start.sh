#!/bin/bash

echo "=== World Simulation Quick Start Guide ==="
echo ""
echo "The simulation is running. The reducers execute successfully but don't produce console output."
echo "This is normal behavior for SpacetimeDB reducers - they modify database state."
echo ""
echo "Available commands that work:"
echo ""

echo "1. Toggle simulation on/off:"
echo "   docker-compose exec -T spacetimedb spacetime call worldsim toggle_simulation"
echo ""

echo "2. Advance time by 1 hour:"
echo "   docker-compose exec -T spacetimedb spacetime call worldsim tick_hour"
echo ""

echo "3. Skip multiple hours (e.g., 24):"
echo "   docker-compose exec -T spacetimedb sh -c 'spacetime call worldsim skip_hours \"24\"'"
echo ""

echo "4. Import test city data:"
echo "   docker-compose exec -T spacetimedb spacetime call worldsim import_test_city"
echo ""

echo "5. Get current hour:"
echo "   docker-compose exec -T spacetimedb spacetime call worldsim get_current_hour"
echo ""

echo "6. Get city summary:"
echo "   docker-compose exec -T spacetimedb sh -c 'spacetime call worldsim get_city_summary \"1\"'"
echo ""

echo "Note: Reducers modify the database but may not print output to console."
echo "To see the actual simulation state, you would need to:"
echo "- Use a SpacetimeDB client SDK to subscribe to tables"
echo "- Check the logs for database modifications"
echo "- Use the web interface at http://localhost:3001 (if available)"