# 120Proof

Idiosyncratic music making system, mine.

## Software

### Pure Data


### Mistress

The boss.  Calls scripts in the right order to initialise instruments, control, and audio paths.

## Patches

In `pd_patches` 

### polysynth.pd

Simple polyphonic synthesiser


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

3027. Not yet implemented:

..1. `InitialiseModHost` to start a mod-host instance

..2. A way to define pedal boards that can be used to assign them to instruments and the audio input
