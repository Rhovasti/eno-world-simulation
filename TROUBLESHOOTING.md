# Troubleshooting Guide - World Simulation

## Common Issues and Solutions

### 1. Docker Build Errors

#### Cargo.lock Version Error
```
failed to parse lock file at: /app/world-simulation/Cargo.lock
lock file version `4` was found, but this version of Cargo does not understand this lock file
```

**Solution:** Update Rust version in Dockerfile
```dockerfile
FROM rust:latest  # Instead of rust:1.75-slim
```

#### Port Already in Use
```
Error response from daemon: driver failed programming external connectivity on endpoint
bind: address already in use
```

**Solution:** Change port in `docker-compose.yml`
```yaml
ports:
  - "3002:3001"  # Change external port
```

### 2. SpacetimeDB Module Issues

#### Module Name Invalid
```
Error: Module name 'world-simulation' contains invalid characters
```

**Solution:** Use only alphanumeric characters in `spacetimedb.toml`
```toml
name = "worldsim"  # No hyphens
```

#### Invalid Publish Command
```
Error: The following required arguments were not provided: <MODULE_PATH>
```

**Solution:** Use correct publish syntax for 1.1.2
```bash
spacetime publish worldsim  # Not 'spacetime publish . worldsim'
```

#### Invalid --release Flag
```
error: Found argument '--release' which wasn't expected
```

**Solution:** Remove `--release` flag from start command
```bash
spacetime start --listen-addr 0.0.0.0:3001  # No --release
```

### 3. Compilation Errors

#### Missing WASM Target
```
error[E0463]: can't find crate for `std`
```

**Solution:** Install WASM target
```bash
rustup target add wasm32-unknown-unknown
```

#### println! Not Allowed
```
error: `println!` is not allowed in SpacetimeDB modules
```

**Solution:** Replace with log macros
```rust
// Before
println!("Message");

// After  
log::info!("Message");
```

#### Type Conflicts
```
error[E0428]: the name `HomeData` is defined multiple times
```

**Solution:** Check for duplicate type definitions and rename conflicting types
```rust
// Rename conflicting types
pub struct HomeConfig { ... }  // Instead of HomeData
pub struct WorkplaceConfig { ... }  // Instead of WorkplaceData
```

### 4. API Migration Issues

#### Old Table Access Patterns
```rust
// Old 0.8.2 syntax (won't compile)
let individual = Individual::filter_by_id(&id);

// New 1.1.2 syntax
let individual = ctx.db.individual().id().find(&id);
```

#### Old Reducer Syntax
```rust
// Old 0.8.2 syntax
#[spacetimedb(reducer)]
pub fn my_reducer() { }

// New 1.1.2 syntax
#[spacetimedb::reducer]
pub fn my_reducer(ctx: &ReducerContext) { }
```

#### Old Table Attributes
```rust
// Old 0.8.2 syntax
#[spacetimedb(table)]
#[primarykey]
pub struct MyTable { }

// New 1.1.2 syntax
#[spacetimedb::table(name = my_table)]
pub struct MyTable {
    #[primary_key]
    // ...
}
```

### 5. Runtime Issues

#### Simulation Not Initialized
```
Error: "Simulation not initialized"
```

**Solution:** Run initialization first
```bash
spacetime call worldsim init_simulation
```

#### No Data Found
```
Error: "City not found" or "Individual not found"
```

**Solution:** Import test data
```bash
spacetime call worldsim import_test_city
```

#### Time Not Advancing
```
Individual needs not updating
```

**Solution:** Use time progression commands
```bash
spacetime call worldsim tick_time
spacetime call worldsim update_individuals
```

### 6. Connection Issues

#### Cannot Connect to Server
```
Error: Connection refused (os error 61)
```

**Solutions:**
1. Check if server is running: `docker-compose ps`
2. Wait longer for startup: Server takes 10-15 seconds
3. Check port configuration in docker-compose.yml
4. Verify SpacetimeDB is listening: `docker-compose logs`

#### Wrong Server Address
```
Error: No such host is known
```

**Solution:** Use correct server address
```bash
spacetime call --server http://localhost:3001 worldsim command
```

### 7. Development Issues

#### Build Fails After Changes
```bash
# Clean rebuild
docker-compose down
docker-compose build --no-cache
docker-compose up
```

#### Module Not Updating
```bash
# Rebuild and republish
./start-simulation.sh rebuild
```

#### Table Schema Changes
When changing table structures, you may need to:
1. Stop the simulation
2. Clear the database
3. Restart and republish

## Debug Commands

### Check System Status
```bash
# Container status
docker-compose ps

# View logs
docker-compose logs spacetimedb

# Check published modules
spacetime list --server http://localhost:3001

# Query database directly
spacetime query --server http://localhost:3001 "SELECT * FROM simulation_time"
```

### Manual Testing
```bash
# Test basic functionality
spacetime call worldsim init_simulation
spacetime call worldsim import_test_city
spacetime call worldsim get_simulation_status
spacetime call worldsim tick_time
```

### Reset Everything
```bash
# Complete reset
docker-compose down
docker system prune -f
docker-compose up --build
```

## Getting Help

1. Check SpacetimeDB 1.1.2 documentation
2. Review the migration changes in the code
3. Check `CLAUDE.md` for project-specific guidelines
4. Use `./start-simulation.sh shell` to debug inside container
5. Enable verbose logging in the Rust module