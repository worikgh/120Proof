# 120Proof

Idiosyncratic music making system.

Runs on [Raspberry Pi](https://raspberrypi.org/) and a [Pisound](https://blokas.io/pisound/).  It could probably run on any other similar hardware.

It uses MIDI, USB, and audio inputs. 

* [ALSA MIDI](https://github.com/opensrc/alsa/blob/master/lib/md/AlsaMidi.md) for MIDI routing.
* [JACK Audio Connection Kit](https://github.com/jackaudio) for audio routing
* [Yoshimi](https://yoshimi.sourceforge.io/) and [Pure Data](https://puredata.info/) for synthesis.  Any MIDI controllable software that can output to Jack will work.

## Raspberry Pi


* Soundcard:  [Pisound](https://blokas.io/pisound/)

* Operating System:  [Patchbox OS](https://blokas.io/patchbox-os/) 

* [Raspberry Pi IV](https://www.raspberrypi.com/products/raspberry-pi-4-model-b/)

## MIDI Pedal

### Driver

The pedal this was developed with is: `ID 4353:4b4d Jieli Technology SINCO`

* The driver in in `pedal/midi_pedal`
* Converts the four buttons into 'A',...,'D'
* Looks for a configuration file in: $Home120Proof/pedal/PEDALS/
* Named 'A', 'B', 'C', or 'D'




## Third Party Software

* [Pure Data - `pd`](https://puredata.info/) 

* [`yoshimi`](https://github.com/Yoshimi/yoshimi)

* Delete amidiauto

	It makes MIDI connections without asking.  Very annoying.

## Setup

* Clone this repo 

* Set environment variable `Home120Proof` to point at the root of the repository  and run `bin/Mistress`.  Works first time....
* Make links from $Home120Proof/bin
120Proofpd -> /home/patch/120Proof/pedal/120Proofpd
lpx_blank_screen -> /home/patch/120Proof/novation_lpx/target/release/lpx_blank_screen
lpx_colour -> /home/patch/120Proof/novation_lpx/target/release/lpx_colour
lpx_controll -> /home/patch/120Proof/novation_lpx/target/release/lpx_controll
lpx_manager -> /home/patch/120Proof/novation_lpx/target/release/lpx_manager
lpx_mode -> /home/patch/120Proof/novation_lpx/target/release/lpx_mode
lpx_scale -> /home/patch/120Proof/novation_lpx/target/release/lpx_scale
lpx_drum -> /home/patch/120Proof/novation_lpx/target/release/lpx_drum

## Configuration

### MIDI

Two classes of hardware device

1. Musician Interface 

	The device the musician uses.  It requires three connections:
	
	1. MIDI note out
	2. MIDI control out
	3. MIDI control in
	
	The two outputs can be the same MIDI device.
	
	Examples
	
	* a keyboard 
	* Novation LPX

2. Audience Interface.

	The device that the audience listens to.  It requires two connections:
	
	1. MIDI note in
	2. MIDI control in
	
	The two inputs can be the same MIDI device.



## Configuration Files 

In `./Instruments` are the instruments definitions.

Each instrument has a configuration file for the LPX MIDI names:

* **`midi_source_lpx`** The MIDI connetion the LPX uses to send note and control MIDI 
* **`midi_sink_lpx`** The MIDI connection the LPX receives control MIDI on. (This is used to change the colours of the pads)
* **`midi_sink_synth`** The MIDI connection the MIDI Notes and controls are sent to the synthesiser on.


For each instrument there is a `midi.cfg` file that 

## Instruments

* Launchpad LPX

	Made by Novation.  A nice grid of LED buttons.  120Proof has custom programmes to control this instrument.

* Pedal

	Essentially a three key USB-keyboard configured as a pedal

  * Look into using a [Behringer FCB1010](https://www.behringer.com/product.html?modelCode=P0089) A MIDI foot pedal
  
* Berringer XR18

	Can only be controlled by software

	~/X-Air-LiveToolbox-132-source/X-AIR-Edit_RASPI_1.5/X-AIR-Edit


# Tools

Executable files in the `bin` directory


## InitialiseMidi

Reads a configuration file, the `midi_Connections` section.  The string `/^MIDI_Connections$/` defines the start of the section.

Each non-cmment line is a tab seperated list of two MIDI devices.  

Connection from first to second.  

Use `aconnect` to ensure that these connections are made and all other cnnections broken

### Invocation

`bin/InitialiseMidi <Config file>`

## lpx_drum

* Runs with Hydrogen to make the drum sounds
* A JSON configuration file to configure the LPX pads into regions (sections) as drums.
* Instrument/Drums/midi.cfg has midi configuration (for bin/InitialiseMidi)

## lpx_mode

Sets the mode of the `lpx`.  Most useful mode is `127` "Programmer Mode".

## lpx_control

Runs programmes based on the control pad pressed on the LPX 

Programmes (executable files) are placed in the sub directrory `subs/`.  Each control signal can trigger execution of two programmes in `subs/`.  When a control signal is received (say `29`) programme `subs/ON-CTL.29` is run.  When any other control is received, say `39`, `subs/OFF-CTL.29` is run then `subs/ON-39`.

## lpx_colour

Sets the colour of a pad on the `LPX`.

`bin/lpx_colour <Pad> <red> <green> <blue>` where `Pad` in `11..98` and `red`, `green`, and `blue` are in `1..127`. 

## lpx_manager

    Sets up LPX buttons for melodic use. Colouring them with three colours: One for root notes, one for notes on scale, and one for all other notes.
	
	Assign the pads to MIDI notes so that they are aligned in five columns.  This leads to duplication where pads in three leftmost and rightmost columns, in adjacent rows, play the same notes.
	
	When a pad is pressed change its colour (to a fourth colour).  Also change the colour of the other pad that can play this note.

### Invocation

`bin/lpx_manager <Path to MIDI configuration> <Root note MIDI> <root colour> <scale colour> other colour> <[1-12]>`

	Where `<Root note MIDI>` is the MIDI value for the note the center pad (r4, c5) is assigned to.

	`<root colour>`, `<scale colour>`, and `<other colour>` are the colours (from the LPX palette) that are assigned to root notes, scale notes, and other notes respectively.

	`<[1-12]>` is the scale defined as one to twelve integers in the range 1 - 12 inclusive, and ordered, that define the notes of the scale.  Always starts with `1`

* Example

	`bin/lpx_manager 60 1 4 6 8 11` 


# Workflow

## General Setup for All Instrumets/Configurations

The LV2 pedal boards are initialised using `modep-mod-host` and `modep-mod-ui`.  The simulators are initialised and the Jack pipes between them set up.

The Jack conections that connect audio input and output into particular pedal boards are placed in files in "pedal/PEDALS/".  Here they are read by the pedal driver (driver.c) or other software as yet unwritten to swap pedal boards in real time.


1. Use [`modep`](https://blokas.io/modep) to build some simulated pedal boards using LV2 effects.

Set them up for the audio interface (simulating an effects unit) or for MIDI instruments.

2. Run `bin/InitialiseModHost` that sets up `mod-host` simulators, Jack
   pipes and the files (in pedal/PEDALS) to be read by the pedal
   driver.  It reads `modep_commands.txt`.

	* It runs `ExtractModep` to create a file (modep_commands.txt) that lists the mod-host commands to set up the LV2 simulations, the JACK connections between them, and the JACK connections (TODO: and MIDI connections) to activate that pedal board

	* In the file `modep_commands.txt` each pedal board definition starts with a `NAME ` line

  1. Each line that starts with a `mh ` is a command for mod-host

  2. Each line that starts with a `jack ` defines a Jack connection

  3. In the `Instruments/` directory create a file for an instrument.
     It will be an executable so after it is run the instrument is set
     up

	* It then starts a local version or `mod-host`  (at `~/mod-host/mod-host`)

3. `InitialiseModHost` then runs `ModhostSimulators`.  Its job is to set up the LV2 simulators  using `mod-host`,  Jack pipes between them, and the files in `pedal/PEDALS` that the pedal driver will read to change the effects in use.

4. Start `lpx_control`.  This monitors the MIDI control signals from the LPX and runs programmes in response that set up the system for use.  `lpx_control` monitors control signals: 19, 29,...,89.  When one of thos buttons s pressed, say 39, the scipt `subs/ON-CTL.39` is run.  Whe another button, say 59 is pressed, `subs/OFF-CTL.39` is run then `subs/ON-CTL.59`.  In this way configurations can be put up and torn down, and there can be up to eight configurations ready to use, live.

The sorts of jobs that `subs/ON-CTL.N` do are:

  * Set up `lpx_manager` to process NIDI from the LPX.
  
  * Set up links in `peal/PEDALS` to establish what effects the pedal controls.
  
  * Set up `yoshimi` (or another synthesiser) to with controls and Jack for audio generation


   This completes the general set up
   
## Setting up a Particular Configuration

Currently there are two MIDI inputs in use and a stereo audio input.  

The stereo input/outputs are to be separated into two separate channels, not done yet.

### Requirements

The MIDI interfaces must be defined.  Currently there are two input
instruments in use, and two possible sinks for MIDI to use to create
initial audio.  The connections are definied in a file that is read by `InitialiseMidi` after the audio generators set up.

The two MIDI controllers are:

1. Lanchpad X.  64 velocity sensitive LED pads.  It appears in the
   MIDI config file as: `Launchpad X:Launchpad X MIDI 2`.

2. Nektar 88 key keyboard.  It appears in the MIDI config file as:
   `Impact LX88+:Impact LX88+ MIDI 1` and `Impact LX88+:Impact LX88+ MIDI 2`

The two MIDI sinks (that produce audio output into Jack) are:

1. Pure Data.  Started with `bin/InitialisePd <instrunent>.pd` where "instrument" defined and is in the file: `pd_patches/instruments/<instrument>.pd`.  For example: `bin/InitialisePd poly_harp_cello.pd`.   There are two MIDI inputs to Pure Data, and the instrument  can use one or both.  For example `pd_patches/instruments/poly_harp_cello.pd` has two simulated instruments:
  1. A harp on input 1
  
  2. A cello on input 2 (17 in Pure Data)
  
The inputs appear in the MIDI configuration file as: `Pure Data:Pure Data Midi-In 1` and `Pure Data:Pure Data Midi-In 1`

2. Yoshimi.  Started with `bin/InitialiseYos <name> <path to
   instrument>`.  Where "name" is an identifier that will be used to
   connect from MIDI and "path to instrument" is the path to an
   instrument definition file.  For examle: `bin/InitialiseYos Midi01
   "/usr/share/yoshimi/banks/Choir_and_Voice/0037-Voiced Synth.xiz"`.
   In this case the MIDI input connection appears in the configuration
   file as: `yoshimi-Midi01:input`.  The Jack output will be (in this
   case) `yoshimi-Midi01`.
   

   
### LPX Setup

* Set mode with: `bin/lpx_mode 127`

* `bin/lpx_manager` needs to know three things:

1. What MIDI devices to send controls to, to receive controls from, and send notes to

2. What the root note is

3. What the scale is

MIDI devices are specified in a file the path to which is the first argument,  The root note (in MIDI, middle C is 60) and a list of integers between 1 and 12, starting with 1, in ascending order, unique as the remaining arguments defines the scale.

### `mod-host`

`InitialiseModHost` starts a local copy of `mod-host` build from Git in `~/mod-host/` running on port 9116.  This means that `MODEP/mod-host` does not need to be stopped.

  <!-- 2. A way to define pedal boards that can be used to assign them to instruments and the audio input -->

  <!-- 3 Set up an instrument: -->

`/usr/local/bin/pd  -jack -path /home/patch/120Proof/pd_patches/ -send "; pd dsp 1" -stdpath  -nogui  pd_patches/instruments/HarpPoly.pd &`
`sleep 2`
`bin/bin/InitialiseMidi`

# Source

## novation_lpx

Collection of parts for controlling the Novation Launchpad X

The tools are:

* lpx_mode
* lpx_control
* lpx_colour
* lpx_manager



# Links

	https://blokas.io/pisound/docs/specs/#pinout-of-pisound-header/ <-  GPIO pins on Pisound

## LV2 Looper Pedal
https://github.com/stevie67/loopor/tree/master/loopor-lv2

## Curated List of LV2 Plugins

Links to other repositories.  No actual useful list, yet

https://github.com/sadko4u/lsp-plugins

## Rust LV2 Library

https://github.com/sadko4u/lsp-plugins


# Instruments

##  zynaddsubfx

Start headless with Jack audio and Alsa MIDI input.  Auto connect.  Load instrument (-L)

zynaddsubfx  --no-gui  -O jack -I alsa    -a -L /usr/share/zynaddsubfx/banks/the_mysterious_bank/0021-rock_organ+distorsion.xiz
