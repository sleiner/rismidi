use rismidi::{MidiChannel, NUM_MIDI_CHANNELS, NUM_MIDI_NOTES};

pub struct ChannelTracker {
    cache: [[MidiChannel; NUM_MIDI_NOTES as usize]; NUM_MIDI_CHANNELS as usize],
}

impl ChannelTracker {
    pub fn new() -> ChannelTracker {
        let mut tracker = ChannelTracker {
            cache: [[MidiChannel::Channel1; NUM_MIDI_NOTES as usize]; NUM_MIDI_CHANNELS as usize],
        };

        for channel_idx in 0..(NUM_MIDI_CHANNELS as usize) {
            // We know that channel_idx is a valid (0-based) MIDI channel,so we can safely call
            // unwrap() here.
            let channel = MidiChannel::try_from_0_based(channel_idx).unwrap();

            tracker.cache[channel_idx] = [channel; NUM_MIDI_NOTES as usize];
        }

        tracker
    }

    pub fn set(&mut self, note: u8, in_channel: MidiChannel, out_channel: MidiChannel) {
        self.cache[in_channel.to_0_based() as usize][note as usize] = out_channel;
    }

    pub fn get(&self, note: u8, in_channel: MidiChannel) -> MidiChannel {
        self.cache[in_channel.to_0_based() as usize][note as usize]
    }
}
