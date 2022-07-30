# Build Instruments


	**TODO** Integrate `Mistress`, `ModhostSimulators`, `driver`, `Instruments/*/Initialise.sh`, `lpx_control`,

	**TODO**: A pattern of writing scripts that:

	* Set up and tear down JACK connections between MIDI sinks, LV2 plugins, audio input (2-channels), ???, and the output two channels.  Not necessarily stereo.  Just two. **DONE** mostly.  Need to use it for a while to plan developments.



* Make copies of the binaries for the LV2 plugins locally

**TODO** Add code to `ExtractModep` to copy the `LV2` programmes and configuration data into 120Proof tree.

**TODO** 
	** TODO** Make initialisation scripts work from `systemd` using `systemctl [start|enable]` **DONE** `Mistress` is called by `/etc/systemd/system/120Proof`

	* Clean the MIDI set up.  (The `midir` library is setting more connections than I ask for when using `lpx_manager`.  **DONE** `InitialiseMidi` uses a cnfiguration file that specifies exactly what MIDI connections must be present and deletes the rest.

	**TODO** Fix `InitialiseMidi` so it will not disconnect `lpx_controll`.  And make the midi ports `lpx_controll` uses configurable.  Currenyly need to edit the congiguration files `midi.cfg` that `InitialiseMIDI` reads to ensure the connects arte not deleted. **DONE** Must put:

```
#lpx_control 
Launchpad X:Launchpad X MIDI 2	120-Proof-CTL:120-Proof-CTL-in
120-Proof-CTL:120-Proof-CTL-out	Launchpad X:Launchpad X MIDI 1
```
	...in the configuration files for `InitialiseMidi`

## Inputs

* Keyboard

* LPX

* Pedal

A three pedal USB keyboard.  Use to configure JACK connections quickly.  Generally takes about 80ms.

**TODO**: Get pedal driver from `ModHostPedal` to work here.

**TODO**: Rename the pedal driver.  `driver` is a bad name.

**TODO**: Add ability to pedal to send control signals to LV2 plugins

**TODO**: Calculate how much overhead there is in USB pedal, hence how important it is to move to a better/quicker system

* Audio

The audio inout/output to Pisound is stereo so effectively there are two independent channels

  * Left channel audio

  * Right channel

**TODO** A case and prlugs that make left and right channels available separately. (Asked on the Blokaslabs forum if channels can be treated independently)

## Processing

## InitialisePd

**Done** Make `InitialisePd` take a patch argument with or without a ".pd" suffix.  Adjust documentation in `README.md`

## InitialiseMidi

**TODO**: Generate the initialisation file from settings.  Define: Input devices, synthesis systems (including LV2 simulators), output devices (? May use USB sound - Berringer XR18?).  From that generate the MIDI, Jack, and LV2 setup scripts including the pedal control files.  The set of Midi, Jack, and Pedal control files are what is used live.


### MIDI Sources

**Done**: Write a programme that deletes all the MIDI connections that are not desired

* LPX
  
	A grid of 64 pads with 16 control pads: eight in the top row and eight in the right column.  
  
  * `novation_lpx` is a Rust crate that generates a series of utilities for the LPX

    * lpx_control
	
	  Run scripts based on pressing control pads on the LPX.  This is intended to change configurations when playing live.
	
    **DONE**: Ensure that the controls cannot be sent accidentally by not accepting a control withing five (?) seconds of sending a MIDI signal.  Change the colours of the control pads when they are active/inactive.
	
    * lpx_manager
	
	  Set up LPX buttons for melodic use.Colouring them with three colours: One for root notes, one for notes on scale, and one for all other notes.
	
	  Assign the pads to MIDI notes so that they are aligned in five columns.  This leads to duplication where pads in three leftmost and rightmost columns play the same notes.
	
	  When a pad is pressed change its colour (to a fourth colour).  Also change the colour of the other pad that can play this note.
	
      **TODO**: Provide a method of reassigning colours without recompiling

      **Done**: Provide a method of defining the names of the Launchpad on one side (MIDI source) and the synthesiser on the other (MIDI sink) without recompiling.
	
    * lpx_colour
	
	The first argument is a pad identifier (11-98 inclusive, no pad identifiers divisible by 10, eighty in total, sixty four MIDI notes and sixteen control pads) the next three arguments are red, green, and blue in range 0..127 inclusive
	
	  * **TODO**: The RGB nature of this does not work as documented.  Need to work out how to accurately set the colour
	  
	  This utility is mostly useful for fun.  Painting patterns on the surface of the LPX
	  
    * lpx_mode
	
	Set the mode of the LPX.  `lpx_mode 127` is most useful as it sets the LPX to "programmer mode".

* Keyboard

Currently using a WORLDE keyboard.

**TODO**: Find how to get the knobs and sliders working

### MIDI Sinks

MIDI sinks take MIDI note definitions and output audio to JACK devices.

* Pure Data

	Define synthesisers and audio processing.  `InitialisePD` to set it up
  
* Yoshimi

	Define synthesisers.  `InitialiseYos` to set it up

  * `./InitialiseYos
  Takes two parameters:
  
  1. A name used to define MIDI connections and Jack pipe names.
  
  2. A path to an "Instrument" file.  These have a suffix ".xiz" and are supplied with Yoshimi.  Yoshimi can edit them to change their characteristics
  
  * LV2 Plugins
  
  Several LV2 plugins are synthesisers.  
	


**TODO**: Define more MIDI Sinks

# Case

**TODO**: Build a case!-)

**TODO**: Separate stereo input and stereo output into two mono channels
