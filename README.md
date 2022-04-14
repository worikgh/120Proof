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

..2. Each line that starts with a `jack ` defines a Jack connection

..3. IN the `Instruments/` directory create a file for an instrument.  It will be an executable so after it is run the instrument is set up

Examples of commands sent to a running `mod-host` listeniong on port 5555


```

echo -n add http://moddevices.com/plugins/mda/Ambience 1 | nc -CN localhost 5555
echo -n param_set 1 hf_damp 70.000000| nc -CN localhost 5555
echo -n param_set 1 mix 90.000000| nc -CN localhost 5555
echo -n param_set 1 output 0.000000| nc -CN localhost 5555
echo -n param_set 1 size 7.000000| nc -CN localhost 5555
echo -n add http://moddevices.com/plugins/caps/AmpVTS 2| nc -CN localhost 5555
echo -n param_set 2 attack 0.502232| nc -CN localhost 5555
echo -n param_set 2 bass 0.250000| nc -CN localhoocalhost 5555

```


3027. Not yet implemented:

..1. `InitialiseModHost` to start a mod-host instance

..2. A way to define pedal boards that can be used to assign them to instruments and the audio input
