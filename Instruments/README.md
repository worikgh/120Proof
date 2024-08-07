# 120Proof/Instruments

Each instrument has an `Initialise` programme that sets it up

Some depend on the LPX, and the rest can use any MIDI controller

Each instrument has a configuration file for the LPX MIDI names:

* **`midi_source_lpx`** The MIDI connetion the LPX uses to send note and control MIDI 
* **`midi_sink_lpx`** The MIDI connection the LPX receives control MIDI on. (This is used to change the colours of the pads)
* **`midi_sink_synth`** The MIDI connection the MIDI Notes and controls are sent to the synthesiser on.

## Directory Contents

### Collections

Synthesisers using yoshimi

### DrumPad01

Not sure what this is

### lpx_ctl

A collection of synthesises using lpx_ctl

### midi_sample 

Use the LPX to play samples.  Turns the LPX into a drum machine.  Utilises `midi_sample`

### pd

Synthesisers using [pd](http://puredata.info/)

### yoshimi

Instrument definition (*.xiz) files for [Yoshimi](https://sourceforge.net/p/yoshimi/code/ci/master/tree/)


