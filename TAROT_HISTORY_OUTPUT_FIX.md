# Tarot History Output Fix

## Problem

The `:tarothistory` commands were working correctly but the output was appearing as error messages (likely in red text) instead of as normal messages in the room.

## Root Cause

The history display functions were using `Err(CommandError::Error(output))` to display results. While this is the standard way for iamb to show command-line output in the status bar, it appears as an error message rather than a proper room message.

## Solution

Added a new `SendAction::SendText(String)` variant that sends formatted text as an actual message to the current room, similar to how tarot readings are sent.

### Changes Made:

1. **`src/base.rs`** - Added new `SendAction` variant:
   ```rust
   pub enum SendAction {
       // ... existing variants ...
       
       /// Send formatted text as a message
       SendText(String),
   }
   ```

2. **`src/windows/room/chat.rs`** - Added handler for `SendText`:
   ```rust
   SendAction::SendText(text) => {
       // Send formatted text as a message
       let text_msg = text_to_message(text);
       let resp = room.send(text_msg.clone()).await.map_err(IambError::from)?;
       
       self.reset();
       
       (resp.event_id, text_msg)
   },
   ```

3. **`src/commands.rs`** - Updated all history display functions:
   - `show_history_list()` - Now sends history as room message
   - `show_reading_details()` - Now sends reading details as room message
   - `show_attribute_graph()` - Now sends bar graphs as room message
   - `show_analytics_summary()` - Now sends summary as room message

### Before:
```rust
let msg = CommandError::Error(output);
return Err(msg);
```

### After:
```rust
let sact = SendAction::SendText(output);
let iact = IambAction::from(sact);
let step = CommandStep::Continue(iact.into(), _ctx.context.clone());
return Ok(step);
```

## Result

Now when you run:
- `:tarothistory` - Sends reading list as a message to the room
- `:tarothistory 1` - Sends reading details as a message to the room
- `:tarothistory suits` - Sends bar graph as a message to the room
- `:tarothistory summary` - Sends analytics summary as a message to the room

All output appears as normal formatted messages in the Matrix room, just like the tarot card readings themselves!

## Testing

```bash
# Rebuild the application
cargo build --release

# Run iamb
./target/release/iamb

# In a room, test the commands:
:tarot 3
:tarothistory
:tarothistory 1
:tarothistory suits
:tarothistory summary
```

The output should now appear as properly formatted messages in the room instead of error messages in the status bar.

## Technical Notes

- The `SendText` action follows the same pattern as `UploadWithText` and other send actions
- It properly resets the message bar after sending
- It creates a local echo so the message appears immediately
- The formatted text supports Markdown formatting (bold, etc.)
- Error cases (like API failures) still use `CommandError::Error` appropriately
