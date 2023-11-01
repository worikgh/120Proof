# Drum Pattern on LPX Novation

Light up LEDs on LPX Novation to simulate dums.

Divide the LEDs into sections and make each section a colour and MIDI signal => drum sound

## Sections - Individual Drums


* Defined  by index of top left pad, width, and height.
* Only rectangular sections for now
* All the pads in a section have the same properties.
* No section can intersect with another

### Properties of a Section

* Main Colour: Each section has a unique main colour
* Active Colour: Each section has an "active" colour.  When any pad in
  the section is pressed (has issued an "on" but not an "off" MIDI
  signal) the section  is the active colour.

## Input

The definition of the drum patern is in a file that is the first argument: `lpx drum <Pattern File>`

It is a JSON file.

An array of JSON Objects.  Each object, is a `Section` has the
following properties:

* pad: Number (int).  11 - 88.  Top left of the section
* main_colour: [Number, Number, Number] ([usize;3]) RGB colour.  Each
  in range 0-127, each unique for every section
* active_colour: [Number, Number, Number] ([usize;3]) RGB colour.
  Each in range 0-127
* midi_note: The note to attach note-on and note-off MIDI events to.
  It is unique for every section






