#!/bin/bash

# Helper script for world-simulation with SpacetimeDB 1.2.0

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}World Simulation SpacetimeDB 1.2.0 Helper${NC}"
echo "=========================================="

case "$1" in
  start)
    echo -e "${YELLOW}Starting SpacetimeDB 1.2.0 and world-simulation...${NC}"
    docker-compose up -d
    echo -e "${GREEN}Waiting for SpacetimeDB to start...${NC}"
    sleep 15
    echo -e "${GREEN}Initializing simulation...${NC}"
    docker-compose exec -T spacetimedb spacetime call worldsim init_simulation 2>/dev/null || true
    echo -e "${GREEN}SpacetimeDB is now running on http://localhost:3001${NC}"
    echo -e "${GREEN}Run './start-simulation.sh demo' to start a demo simulation${NC}"
    ;;
    
  stop)
    echo -e "${YELLOW}Stopping world-simulation...${NC}"
    docker-compose down
    ;;
    
  logs)
    docker-compose logs -f
    ;;
    
  exec)
    shift
    docker-compose exec spacetimedb spacetime "$@"
    ;;
    
  demo)
    echo -e "${YELLOW}Running demo simulation with SpacetimeDB 1.2.0...${NC}"
    # Check if already initialized and skip if so
    docker-compose exec -T spacetimedb sh -c "
      echo 'Checking simulation state...' && \
      (spacetime call worldsim init_simulation 2>/dev/null || echo 'Simulation already initialized') && \
      echo '' && \
      echo 'Importing test city...' && \
      (spacetime call worldsim import_test_city 2>/dev/null || echo 'Test city already imported') && \
      echo '' && \
      echo 'Toggling simulation on...' && \
      spacetime call worldsim toggle_simulation && \
      echo '' && \
      echo 'Advancing time by 5 hours...' && \
      spacetime call worldsim skip_hours '5' && \
      echo '' && \
      echo 'Current simulation hour:' && \
      spacetime call worldsim get_current_hour && \
      echo '' && \
      echo 'City 1 Summary:' && \
      spacetime call worldsim get_city_summary '1' && \
      echo '' && \
      echo 'Sample individual story (ID 1, last 24 hours):' && \
      spacetime call worldsim get_individual_story '1' '24' 2>/dev/null || echo 'No individual with ID 1'
    "
    ;;
    
  shell)
    docker-compose exec spacetimedb /bin/bash
    ;;
    
  rebuild)
    echo -e "${YELLOW}Rebuilding and republishing module...${NC}"
    docker-compose exec spacetimedb sh -c "cd /app/world-simulation && \
      cargo build --target wasm32-unknown-unknown && \
      spacetime publish worldsim"
    ;;

  init)
    echo -e "${YELLOW}Initializing simulation...${NC}"
    docker-compose exec spacetimedb spacetime call worldsim init_simulation
    ;;

  import)
    echo -e "${YELLOW}Importing test data...${NC}"
    docker-compose exec spacetimedb spacetime call worldsim import_test_city
    echo -e "${GREEN}Test city with 1000 population imported${NC}"
    ;;

  tick)
    echo -e "${YELLOW}Advancing time by 1 hour...${NC}"
    docker-compose exec spacetimedb spacetime call worldsim tick_hour
    ;;

  status)
    echo -e "${YELLOW}Getting simulation status...${NC}"
    echo -e "\n${GREEN}Current Hour:${NC}"
    docker-compose exec -T spacetimedb spacetime call worldsim get_current_hour 2>/dev/null || echo "Not available"
    echo -e "\n${GREEN}City Summary (City 1):${NC}"
    docker-compose exec -T spacetimedb sh -c 'spacetime call worldsim get_city_summary "1"' 2>/dev/null || echo "No city data available"
    ;;
    
  hour)
    echo -e "${YELLOW}Current simulation hour:${NC}"
    docker-compose exec spacetimedb spacetime call worldsim get_current_hour
    ;;
    
  skip)
    if [ -z "$2" ]; then
      echo -e "${RED}Please specify number of hours to skip${NC}"
      echo "Usage: $0 skip <hours>"
      exit 1
    fi
    echo -e "${YELLOW}Advancing time by $2 hours...${NC}"
    docker-compose exec spacetimedb sh -c "spacetime call worldsim skip_hours '$2'"
    ;;
    
  story)
    if [ -z "$2" ] || [ -z "$3" ]; then
      echo -e "${RED}Please specify type and ID${NC}"
      echo "Usage: $0 story <individual|building> <id>"
      exit 1
    fi
    case "$2" in
      individual)
        HOURS="${4:-24}"
        echo -e "${YELLOW}Getting story for individual $3 (last $HOURS hours)...${NC}"
        docker-compose exec spacetimedb sh -c "spacetime call worldsim get_individual_story '$3' '$HOURS'"
        ;;
      building)
        echo -e "${YELLOW}Getting story for building $3...${NC}"
        docker-compose exec spacetimedb sh -c "spacetime call worldsim get_building_story '$3'"
        ;;
      *)
        echo -e "${RED}Invalid type. Use 'individual' or 'building'${NC}"
        exit 1
        ;;
    esac
    ;;
    
  city)
    CITY_ID="${2:-1}"
    echo -e "${YELLOW}Getting summary for city $CITY_ID...${NC}"
    docker-compose exec spacetimedb sh -c "spacetime call worldsim get_city_summary '$CITY_ID'"
    ;;

  # Autoticker commands
  auto-start)
    echo -e "${YELLOW}Starting autoticker...${NC}"
    docker-compose exec spacetimedb spacetime call worldsim start_autoticker
    ;;

  auto-stop)
    echo -e "${YELLOW}Stopping autoticker...${NC}"
    docker-compose exec spacetimedb spacetime call worldsim stop_autoticker
    ;;

  auto-status)
    echo -e "${YELLOW}Getting autoticker status...${NC}"
    docker-compose exec spacetimedb spacetime call worldsim get_autoticker_status
    ;;

  auto-check)
    echo -e "${YELLOW}Checking for auto-tick...${NC}"
    docker-compose exec spacetimedb spacetime call worldsim check_autotick
    ;;

  auto-rate)
    if [ -z "$2" ]; then
      echo -e "${RED}Please specify tick rate${NC}"
      echo "Usage: $0 auto-rate <rate>"
      echo "Available rates: realtime, fast, very_fast, test, slow"
      echo "  realtime: 1 hour = 1 hour real time"
      echo "  fast: 1 hour = 1 minute real time" 
      echo "  very_fast: 1 hour = 10 seconds real time"
      echo "  test: 1 hour = 1 second real time"
      echo "  slow: 1 hour = 5 minutes real time"
      exit 1
    fi
    echo -e "${YELLOW}Setting tick rate to $2...${NC}"
    docker-compose exec spacetimedb sh -c "spacetime call worldsim set_tick_rate '$2'"
    ;;

  auto-interval)
    if [ -z "$2" ]; then
      echo -e "${RED}Please specify interval in milliseconds${NC}"
      echo "Usage: $0 auto-interval <milliseconds>"
      echo "Example: $0 auto-interval 5000  # 5 seconds"
      exit 1
    fi
    echo -e "${YELLOW}Setting custom tick interval to $2ms...${NC}"
    docker-compose exec spacetimedb sh -c "spacetime call worldsim set_tick_interval '$2'"
    ;;

  auto-demo)
    echo -e "${YELLOW}Running autoticker demo...${NC}"
    docker-compose exec -T spacetimedb sh -c "
      echo 'Initializing simulation...' && \
      (spacetime call worldsim init_simulation 2>/dev/null || echo 'Simulation already initialized') && \
      echo '' && \
      echo 'Importing test city...' && \
      (spacetime call worldsim import_test_city 2>/dev/null || echo 'Test city already imported') && \
      echo '' && \
      echo 'Starting simulation...' && \
      spacetime call worldsim toggle_simulation && \
      echo '' && \
      echo 'Setting to fast tick rate (1 hour = 10 seconds)...' && \
      spacetime call worldsim set_tick_rate 'very_fast' && \
      echo '' && \
      echo 'Starting autoticker...' && \
      spacetime call worldsim start_autoticker && \
      echo '' && \
      echo 'Checking auto-tick (triggering first tick)...' && \
      spacetime call worldsim check_autotick && \
      echo '' && \
      echo 'Current simulation hour:' && \
      spacetime call worldsim get_current_hour && \
      echo '' && \
      echo 'Waiting 10 seconds then checking again...' && \
      sleep 11 && \
      spacetime call worldsim check_autotick && \
      echo '' && \
      echo 'Current simulation hour after auto-tick:' && \
      spacetime call worldsim get_current_hour && \
      echo '' && \
      echo 'Autoticker demo complete!' && \
      echo 'The autoticker is now configured. Call \"check_autotick\" periodically to advance time.' && \
      echo 'Use \"$0 auto-stop\" to stop the autoticker.' && \
      echo 'Use \"$0 auto-status\" to check the current status.'
    "
    ;;
    
  *)
    echo "Usage: $0 {start|stop|logs|exec|demo|shell|rebuild|init|import|tick|status|hour|skip|story|city|auto-*}"
    echo ""
    echo "Basic Commands:"
    echo "  start   - Start SpacetimeDB 1.2.0 server in Docker"
    echo "  stop    - Stop the Docker container"
    echo "  logs    - Show container logs"
    echo "  exec    - Execute spacetime commands (e.g., $0 exec call worldsim <reducer> <args>)"
    echo "  demo    - Run a quick demo simulation"
    echo "  shell   - Open a shell in the container"
    echo "  rebuild - Rebuild and republish the module"
    echo ""
    echo "Simulation Commands:"
    echo "  init    - Initialize the simulation"
    echo "  import  - Import test city data"
    echo "  tick    - Advance time by 1 hour"
    echo "  status  - Show simulation status"
    echo "  hour    - Show current simulation hour"
    echo "  skip    - Skip N hours (e.g., $0 skip 24)"
    echo "  story   - Get story (e.g., $0 story individual 1)"
    echo "  city    - Get city summary (defaults to city 1)"
    echo ""
    echo "Autoticker Commands:"
    echo "  auto-start    - Start automatic time progression"
    echo "  auto-stop     - Stop automatic time progression"
    echo "  auto-check    - Check for and execute auto-tick if due"
    echo "  auto-status   - Show autoticker status"
    echo "  auto-rate     - Set predefined tick rate (realtime|fast|very_fast|test|slow)"
    echo "  auto-interval - Set custom tick interval in milliseconds"
    echo "  auto-demo     - Quick demo with autoticker enabled"
    exit 1
    ;;
esac