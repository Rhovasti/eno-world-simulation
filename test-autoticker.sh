#!/bin/bash

# Test script for autoticker functionality
DB_NAME="worldsim-autoticker"

echo "=== Testing Autoticker Functionality ==="
echo ""

echo "1. Initialize simulation..."
docker-compose exec -T spacetimedb spacetime call $DB_NAME init_simulation

echo ""
echo "2. Import test data..."
docker-compose exec -T spacetimedb spacetime call $DB_NAME import_test_city

echo ""
echo "3. Start simulation..."
docker-compose exec -T spacetimedb spacetime call $DB_NAME toggle_simulation

echo ""
echo "4. Check initial hour..."
docker-compose exec -T spacetimedb spacetime call $DB_NAME get_current_hour

echo ""
echo "5. Set tick rate to very fast (1 hour = 10 seconds)..."
docker-compose exec -T spacetimedb spacetime call $DB_NAME set_tick_rate '"very_fast"'

echo ""
echo "6. Start autoticker..."
docker-compose exec -T spacetimedb spacetime call $DB_NAME start_autoticker

echo ""
echo "7. Check autoticker status..."
docker-compose exec -T spacetimedb spacetime call $DB_NAME get_autoticker_status

echo ""
echo "8. Trigger first autotick check..."
docker-compose exec -T spacetimedb spacetime call $DB_NAME check_autotick

echo ""
echo "9. Check hour after first tick..."
docker-compose exec -T spacetimedb spacetime call $DB_NAME get_current_hour

echo ""
echo "10. Wait 11 seconds and check again..."
sleep 11

echo ""
echo "11. Trigger second autotick check..."
docker-compose exec -T spacetimedb spacetime call $DB_NAME check_autotick

echo ""
echo "12. Check hour after second tick..."
docker-compose exec -T spacetimedb spacetime call $DB_NAME get_current_hour

echo ""
echo "13. Final autoticker status..."
docker-compose exec -T spacetimedb spacetime call $DB_NAME get_autoticker_status

echo ""
echo "=== Autoticker Test Complete ==="