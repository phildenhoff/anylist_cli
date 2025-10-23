# anylist_cli

An unofficial command-line utility for manage your
[AnyList](https://www.anylist.com/) shopping lists, items, meal plans, stores, and categories.

## Installation

### Prerequisites

- Rust toolchain (1.70+)
- Protocol Buffers compiler (`protoc`)

### Building

```bash
cargo build --release
```

The binary will be available at `target/release/anylist`.

## Usage

### Authentication

First, log in with your AnyList credentials:

```bash
anylist login
```

This will save your credentials locally for future use.

### Shopping Lists

```bash
# List all shopping lists
anylist list

# View a specific list with all items
anylist list get "Groceries"

# Create a new list
anylist list create "Weekly Shopping"

# Rename a list
anylist list rename "Old Name" "New Name"

# Delete a list
anylist list delete "List Name"
```

### List Items

```bash
# Add an item to a list
anylist item add "Groceries" "Milk"

# Add an item with quantity, details, and category
anylist item add "Groceries" "Apples" --quantity "2 lbs" --details "Organic" --category "Produce"

# Update an item
anylist item update "Groceries" ITEM_ID "Green Apples" --quantity "3 lbs"

# Check off an item
anylist item check "Groceries" ITEM_ID

# Uncheck an item
anylist item uncheck "Groceries" ITEM_ID

# Delete an item
anylist item delete "Groceries" ITEM_ID
```

### Stores

```bash
# Add a store to a list
anylist store add "Groceries" "Whole Foods"

# Update a store name
anylist store update "Groceries" STORE_ID "Trader Joe's"

# Delete a store
anylist store delete "Groceries" STORE_ID
```

### Categories

```bash
# Add a category to a list
anylist category add "Groceries" CATEGORY_GROUP_ID "Bakery"

# Rename a category
anylist category rename "Groceries" CATEGORY_GROUP_ID CATEGORY_ID "Fresh Bakery"

# Delete a category
anylist category delete "Groceries" CATEGORY_ID
```

### Meal Plans

```bash
# List meal plan events for a date range
anylist meal-plan list 2024-01-01 2024-01-31

# Add a meal plan event
anylist meal-plan add CALENDAR_ID 2024-01-15 --title "Dinner with friends"

# Add a meal plan event with a recipe
anylist meal-plan add CALENDAR_ID 2024-01-16 --recipe-id RECIPE_ID --label-id LABEL_ID

# Update a meal plan event
anylist meal-plan update CALENDAR_ID EVENT_ID 2024-01-17 --title "Updated dinner"

# Delete a meal plan event
anylist meal-plan delete CALENDAR_ID EVENT_ID
```

## Getting Help

Use `--help` with any command to see available options:

```bash
anylist --help
anylist list --help
anylist item --help
anylist item add --help
```

## Configuration

Credentials are stored in `~/.config/anylist_rs/config.json` after logging in on
Linux systems, and at
`/Users/<you>/Library/Application Support/anylist_rs/config.json` on macOS.

## Development

This CLI uses the [anylist_rs](https://github.com/phildenhoff/anylist_rs) library for API access.

### Project Structure

```
src/
├── main.rs              # Entry point and command routing
├── auth.rs              # Authentication and credential management
└── commands/
    ├── mod.rs           # Module declarations
    ├── login.rs         # Login command
    ├── list.rs          # List management commands
    ├── items.rs         # Item management commands
    ├── stores.rs        # Store management commands
    ├── categories.rs    # Category management commands
    └── meal_plans.rs    # Meal plan commands
```

## Disclaimer

This is unofficial: please do not bother the AnyList folks with this tool.
