# 120Proof

Idiosyncratic music making system.

## Instruments

* Launchpad LPX

	Made by Novation.  A nice grid of LED buttons 

* WORLDE midi keyboard

	Cheap from Ali Express

	Keys work OK

	Pitch bend works

    Controls do not properly work (knobs and sliders)

* Pedal

	Essentially a three key USB-keyboard configured as a pedal

* Berringer XR18

	Can only be controlled by software

	~/X-Air-LiveToolbox-132-source/X-AIR-Edit_RASPI_1.5/X-AIR-Edit

# Tools

## InitialisePd

Sets up Pure Data running in the background.

Kills any instances of Pure Data already  running.

### Invocation

Pass the name of a patch (including the `.pd` suffix) as an argument.  No argument just kills Pure Data. 

Example: `./InitialisePd poly_harp~.pd`

## InitialiseMidi

Reads a configuration file, the `midi_Connections` section.  The string `/^MIDI_Connections$/` defines the start of the section.

Each non-cmment line is a tab seperated list of two MIDI devices.  

Connection from first to second.  

Use `aconnect` to ensure that these connections are made and all other cnnections broken

### Invocation

`./InitialiseMidi <Config file>`

## lpx_mode

Sets the mode of the `lpx`.  Most useful mode is `127` "Programmer Mode".

## lpx_control

Runs programmes based on the control pad pressed on the LPX 

Programmes (executable files) are placed in the sub directrory `subs/`.  Each control signal can trigger execution of two programmes in `subs/`.  When a control signal is received (say `29`) programme `subs/ON-CTL.29` is run.  When any other control is received, say `39`, `subs/OFF-CTL.29` is run then `subs/ON-39`.

## lpx_colour

Sets the colour of a pad on the `LPX`.

`./lpx_colour <Pad> <red> <green> <blue>` where `Pad` in `11..98` and `red`, `green`, and `blue` are in `1..127`. 

## lpx_manager

    Sets up LPX buttons for melodic use. Colouring them with three colours: One for root notes, one for notes on scale, and one for all other notes.
	
	Assign the pads to MIDI notes so that they are aligned in five columns.  This leads to duplication where pads in three leftmost and rightmost columns play the same notes.
	
	When a pad is pressed change its colour (to a fourth colour).  Also change the colour of the other pad that can play this note.

### Invocation

`./lpx_manager <Root note MIDI> <[1-12]>`

Where `<Root note MIDI>` is the MIDI value for the note the center pad (r4, c5) is assigned to.

`<[1-12]>` is the scale defined as one to twelve integers in the range 1 - 12 inclusive, and ordered, that define the notes of the scale.  Always starts with `1`

* Example

	`./lpx_manager 60 1 4 6 8 11` 


# Workflow

1. Use [`modep`](https://blokas.io/modep) to build some simulated pedal bards using LV2 effects.

Set them up for the audio interface (simulating an effects unit) or for MIDI instruments.

2. Use `ExtractModep` to create a file (modep_commands.txt) that lists the mod-host commands to set up the LV2 simulations, the JACK connections between them, and the JACK connections (TODO: and MIDI connections) to activate that pedal board

3. In the file `modep_commands.txt` each pedal board definition starts with a `NAME ` line

  1. Each line that starts with a `mh ` is a command for mod-host

  2. Each line that starts with a `jack ` defines a Jack connection

  3. IN the `Instruments/` directory create a file for an instrument.  It will be an executable so after it is run the instrument is set up

Copy the lines for the pedal board into the new file.  Complete with `mh` and `jack` prefixes

For example the following `Harp_Sweetner` is designed to be used by the `Harp~` instrument in `pd_patches/instruments`

```

NAME Harp_Sweetner
mh add http://gareus.org/oss/lv2/b_reverb 6
mh param_set 6 gain_in 0.040000
mh param_set 6 mix 0.300000
mh add http://moddevices.com/plugins/tap/doubler 7
mh param_set 7 DryLeftPosition 0.003348
mh param_set 7 DryLevelDb -51.082589
mh param_set 7 DryRightPosition 1.000000
mh param_set 7 PitchTracking 0.641741
mh param_set 7 TimeTracking 0.592634
mh param_set 7 WetLeftPosition 0.000000
mh param_set 7 WetLevelDb 1.000000
mh param_set 7 WetRightPosition 1.000000
mh add http://guitarix.sourceforge.net/plugins/gxechocat#echocat 8
mh param_set 8 bpm 120.000000
mh param_set 8 drive 0.273438
mh param_set 8 gain 0.496652
mh param_set 8 head1 0.000000
mh param_set 8 head2 0.000000
mh param_set 8 head3 0.000000
mh param_set 8 sustain 0.704241
mh param_set 8 swell 0.870536
jack effect_6:out effect_8:in
jack effect_8:out effect_7:Input_L
jack effect_8:out effect_7:Input_R
ACTIVATE
jack system:capture_1 effect_6:in
jack system:capture_2 effect_6:in
jack effect_7:Output_L system:playback_1
jack effect_7:Output_R system:playback_2

```

To connect Pure Data instruments connect `pure_data:output_1` and
`pure_data:output_2` to the same ports `system:capture_1` and
`system:capture_2` are connected to.

```
jack pure_data:output_1 effect_6:in
jack pure_data:output_2 effect_6:in
```

Disconect Pure Data from the stdout:
```
mh disconnect pure_data:output_1 system:playback_1
mh disconnect pure_data:output_2 system:playback_2
```

Examples of commands sent to a running `mod-host` listening on port 5555
```

echo -n add http://moddevices.com/plugins/mda/Ambience 1 | nc -N localhost 5555
echo -n param_set 1 hf_damp 70.000000| nc -N localhost 5555
echo -n param_set 1 mix 90.000000| nc -N localhost 5555
echo -n param_set 1 output 0.000000| nc -N localhost 5555
echo -n param_set 1 size 7.000000| nc -N localhost 5555
echo -n add http://moddevices.com/plugins/caps/AmpVTS 2| nc -N localhost 5555
echo -n param_set 2 attack 0.502232| nc -N localhost 5555
echo -n param_set 2 bass 0.250000| nc -N localhoocalhost 5555

```


3027. Not yet implemented:

  1. `InitialiseModHost` to start a mod-host instance

  2. A way to define pedal boards that can be used to assign them to instruments and the audio input

  3 Set up an instrument:

`/usr/local/bin/pd  -jack -path /home/patch/120Proof/pd_patches/ -send "; pd dsp 1" -stdpath  -nogui  pd_patches/instruments/HarpPoly.pd &`
`sleep 2`
`./InitialiseMidi`

# Source

## novation_lpx

Collection of parts for controlling the Novation Launchpad X

The tools are:

* lpx_mode
* lpx_control
* lpx_colour
* lpx_manager



# Links

https://github.com/sadko4u/lsp-plugins
https://blokas.io/pisound/docs/specs/#pinout-of-pisound-header/ <-  GPIO pins on Pisound

# Instruments

##  zynaddsubfx

Start headless with Jack audio and Alsa MIDI input.  Auto connect.  Load instrument (-L)
zynaddsubfx  --no-gui  -O jack -I alsa    -a -L /usr/share/zynaddsubfx/banks/the_mysterious_bank/0021-rock_organ+distorsion.xiz
