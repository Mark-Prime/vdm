# VDM

A rust library for parsing and modifying [.vdm](https://developer.valvesoftware.com/wiki/Demo_Recording_Tools#Demo_editor) files used in source games.

Included features:

- Easily parse .vdm files.
- Create a new .vdm from scratch.
- Modify, delete and add actions.
- Export to a file.

## Usage

Add this to your `Cargo.toml`

```
[dependencies]
vdm = "1"
```

### Examples

#### Creating a new .vdm

The quickest way to get stated is by making a brand new .vdm file and exporting it.

```rust
let vdm = VDM::new();

// Include the file path you wish to export to
vdm.export("example.vdm"));
```

However, this would result in a blank .vdm file. Instead, let's add a new action to it.

```rust
let mut vdm = VDM::new();

// This will create a new action and automatically append it to the vdm.
let mut action = vdm.create_action(ActionType::SkipAhead);

// Gather the props from the action to edit them.
let mut props = action.props();

// Name the action for easy organization when exported
props.name = "Skip 5 seconds in".to_string();

// Skip 5 seconds into the demo
props.skip_to_time = Some(5.0);

// Save the new props to the action
action.set_props(props);

// Save the action to the vdm
vdm.set_last(action);

vdm.export("example.vdm");
```

This is rather verbose, but we can shorten it quite a bit by knowing what we want to do.

```rust
let mut vdm = VDM::new();

// create_action() always create the action at the end, we don't need to save it because it's easy to access later.
let mut props = vdm.create_action(ActionType::SkipAhead).props_mut();

// Since we used .props_mut() we can directly edit the Action without needing to set anything after.
// Set is available if you want to completely replace an Action or its properties with .set_nth_props()
props.name = "Skip 5 seconds in".to_string();
props.skip_to_time = Some(5.0);

vdm.export("example.vdm");
```

example.vdm

```vdm
demoactions
{
  "1"
  {
    factory "SkipAhead"
    name "Unnamed"
    skiptotime "5.000"
  }
}
```

Because no start time is listed, it will happen as soon as possible after the demo starts.

#### Editing an existing .vdm file

You can also parse/edit existing .vdm files in a very similar way.

In this example, we're going to change when the skipping starts in the previous example.

```rust
let mut vdm = VDM::open("example.vdm").unwrap();

// Grab the first actions properties.
let mut props = vdm.first().props();

// This sets the starting point 100 game ticks into the demo
// 66 ticks = 1 second
props.start_tick = Some(100);

// You could also use vdm.set_nth_props(0, props);
vdm.set_first_props(props);

vdm.export("example.vdm");
```

Alternatively, we could also borrow it as mutable using .first_mut() and .props_mut() which cuts it down even further.

```rust
let mut vdm = VDM::open("example.vdm").unwrap();
// Grab the first actions properties as mutable.
let mut props = vdm.first_mut().props_mut();

// This sets the starting point 100 game ticks into the demo
// 66 ticks = 1 second
props.start_tick = Some(500);

// export without needing to set anything.
vdm.export("example.vdm");
```

example.vdm

```vdm
demoactions
{
  "1"
  {
    factory "SkipAhead"
    name "Unnamed"
    skiptotime "5.000"
    starttick "100"
  }
}
```

The .vdm file will only ever show actions that work with that specific action. SkipAhead will never show RGB values for example.

#### Deleting an Action

Let's say we need to delete the action we just made. It's simple!

```rust
let mut vdm = VDM::open("example.vdm").unwrap();

// Remove whatever the last action is.
vdm.remove_last();

// Alternatively you can remove the first or nth element
// vdm.remove(n);
// vdm.remove_first();

vdm.export("example.vdm");
```

example.vdm

```vdm
demoactions
{
}
```

### Action Types

- SkipAhead
- StopPlayback
- PlayCommands
- ScreenFadeStart
- TextMessageStart
- PlayCDTrackStart
- PlaySoundStart
- Pause
- ChangePlaybackRate
- ZoomFov

Dev command: cargo watch -q -c -x "run -q"

License: MIT OR Apache-2.0