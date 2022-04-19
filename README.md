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


# Workflow

1. Use [`modep`](https://blokas.io/modep) to build some simulated pedal bards using LV2 effects.

Set them up for the audio interface (simulating an effects unit) or for MIDI instruments.

2. Use `ExtractModep` to create a file (modep_commands.txt) that lists the mod-host commands to set up the LV2 simulations, the JACK connections between them, and the JACK connections (TODO: and MIDI connections) to activate that pedal board

3. In the file `modep_commands.txt` each pedal board definition starts with a `NAME ` line

..1. Each line that starts with a `mh ` is a command for mod-host

⋅⋅2. Each line that starts with a `jack ` defines a Jack connection

..3. IN the `Instruments/` directory create a file for an instrument.  It will be an executable so after it is run the instrument is set up

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

To connect Pure Data instruments cionnecvt `pure_data:output_1` and
`pure_data:output_2` to the same ports `system:capture_1` and
`system:capture_2` are connected to.

```
jack pure_data:output_1 effect_6:in
jack pure_data:output_2 effect_6:in
```

Disconecty Pure Data from the stdout:
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

..1. `InitialiseModHost` to start a mod-host instance

..2. A way to define pedal boards that can be used to assign them to instruments and the audio input

..3 Set up an instrument:

`/usr/local/bin/pd  -jack -path /home/patch/120Proof/pd_patches/ -send "; pd dsp 1" -stdpath  -nogui  pd_patches/instruments/HarpPoly.pd &`
`sleep 2`
`./InitialiseMidi`
