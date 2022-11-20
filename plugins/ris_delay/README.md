# ris_delay

Delays MIDI notes or control changes, including feedback.

## Parameters

- **Tempo Sync**:
  Of this is active, the delay time will be measured in musical note values, following the host's
  tempo.
- **Delay Time**:
  Either tempo-synced values (if **Tempo Sync** is on) or some constant time (in milliseconds).
- **Feedback**:
  The amount of delayed signal "fed back" into the algorithm to be delayed again. At 0%, the input
  will only be delayed once, at 100% the input signal will be infinitely repeated.
- **Feedback Limit**:
  Optional Limit for the number of times that an event will be sent through the feedback loop.
  If set to "Off", no limit will be applied.
- **Wet**:
  Scaling factor for the level (e.g. the velocity of a "Note On" message) of delayed events.
- **Dry**:
  Scaling factor for the level (e.g. the velocity of a "Note On" message) of events passed through
  without delay.
- **Mode**:
  Either "Notes only" or "Notes and CCs". Be careful when delaying CC events with feedback.
- **Channel**:
  Optionally, select only a single channel for which MIDI events should be delayed.
