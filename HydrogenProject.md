# Make a system that can take a Hydrogen songe (*.h2song) and a Jack
  Timebase feed and play the Hydrogen song into a Jack audio pipe

# Use: ~/hydrogen/src/player/

# hydrogen/src/core/Hydrogen.h defines `Hydrogen::tempo` enum that defines `Jack`:

```
		/** Hydrogen will disregard all internal tempo settings and
			uses the ones provided by the JACK server instead. This
			mode is only used in case the JACK audio driver is used,
			JACK timebase support is activated in the Preferences, and
			an external timebase master is registered to the JACK
			server.*/
		Jack = 2
```

It looks like this is set in the song file.

If the song mode is `Song::Mode::Song` then if `getJackTimebaseState()
== JackAudioDriver::Timebase::Slave` tempo is `Tempo::Jack`. (See
Hydrogen.cpp:1340)

The default mode for a song is: `Song::Mode::Pattern` 

The mode of a song is set by: `std::shared_ptr<Song> Song::loadFrom( XMLNode* pRootNode, bool bSilent )` Song.cpp:257 by calling `void Song::setMode( Song::Mode mode )`

# Jack Timebase

[Thie link
indicates](http://hydrogen-music.org/documentation/manual/manual_en_chunked/ch07s05.html)
that there is a master and there are slaves



## Properties

[From
this](https://github.com/SpotlightKid/jack-audio-tools/blob/master/transport/timebase_master.py)
reading the Python source code


* Beats per bar

* Beat type (?)

* BPM

* Ticks per beat

## Timebase Master

[From this
website](http://www.crudebyte.com/jack-ios/sdk/doc/transport-design.html):

The timebase master continuously updates extended position
information, counting beats, timecode, etc.  If no [Jack] client is
registered as timebase master, frame numbers will be the only position
information available.


The timebase master registers a callback that updates position
information while the transport is rolling.

Its [the calback?] output affects the following process cycle.
[What is a process cycle?]


This function [the callback?] is called immediately after the process
callback in the same thread whenever the transport is rolling, or when
any client has set a new position in the previous cycle.
[What is the "process callback"?]

The first cycle after `jack_set_timebase_callback()` is also treated as
a new position, or the first cycle after `jack_activate()` if the client
had been inactive.

```
typedef int  (*JackTimebaseCallback)(jack_transport_state_t state,
                                     jack_nframes_t nframes,
                                     jack_position_t *pos,
                                     int new_pos,
                                     void *arg);
```


When a new client takes over, the former timebase callback is no
longer called. Taking over the timebase may be done conditionally, in
which case the takeover fails when there is a master already. The
existing master can release it voluntarily, if desired.

```
int  jack_set_timebase_callback (jack_client_t *client,
                                 int conditional,
                                 JackTimebaseCallback timebase_callback,
                                 void *arg);
int  jack_release_timebase(jack_client_t *client);
```


## Timebase Client

