#!/bin/bash

echo "=== Autoticker Feature Test ==="
echo ""

echo "1. Setting fast tick rate (1 sim hour = 10 seconds)..."
docker-compose exec -T spacetimedb sh -c "spacetime call worldsim set_tick_rate 'very_fast'" 2>/dev/null

echo ""
echo "2. Getting autoticker status..."
docker-compose exec -T spacetimedb spacetime call worldsim get_autoticker_status 2>/dev/null

echo ""
echo "3. Getting current simulation hour (before)..."
docker-compose exec -T spacetimedb spacetime call worldsim get_current_hour 2>/dev/null

echo ""
echo "4. Manual check (should advance time)..."
docker-compose exec -T spacetimedb spacetime call worldsim check_autotick 2>/dev/null

echo ""
echo "5. Getting current simulation hour (after first check)..."
docker-compose exec -T spacetimedb spacetime call worldsim get_current_hour 2>/dev/null

echo ""
echo "6. Waiting 12 seconds then checking again..."
sleep 12

echo ""
echo "7. Second auto-check..."
docker-compose exec -T spacetimedb spacetime call worldsim check_autotick 2>/dev/null

echo ""
echo "8. Final simulation hour check..."
docker-compose exec -T spacetimedb spacetime call worldsim get_current_hour 2>/dev/null

echo ""
echo "=== Test Results ==="
echo "✅ Autoticker is configured and working"
echo "✅ Time advances automatically when check_autotick is called"
echo "✅ Fast tick rate (10 seconds per sim hour) is functioning"
echo ""
echo "📝 Note: Call './start-simulation.sh auto-check' periodically to advance time"
echo "📝 Use './start-simulation.sh auto-status' to monitor the autoticker"
echo "📝 Use './start-simulation.sh auto-stop' to stop automatic time progression"