# neon_al
OpenAL bindings for node using Neon. This library was built for use with the ffmpeg library. You also need [neon_ffmpeg](https://github.com/kyeongwoon/neon_ffmpeg) for handling media files.


This project was bootstrapped by [create-neon](https://www.npmjs.com/package/create-neon).

## Installing neon_al

Installing neon_al requires a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support).

You can install the project with npm. In the project directory, run:

```sh
$ brew openal-soft
$ npm install
```

This fully installs the project, including installing any dependencies and running the build.

## Building neon_al

If you have already installed the project and only want to run the build, run:

```sh
$ npm run build
```
If you cannot find the openal-soft library when building, you can edit the following part in build.rs.
```rust
#[cfg(any(target_os = "macos", target_os = "macos"))]
println!(r"cargo:rustc-link-search=/opt/homebrew/opt/openal-soft/lib");
```

This command uses the [cargo-cp-artifact](https://github.com/neon-bindings/cargo-cp-artifact) utility to run the Rust build and copy the built library into `./index.node`.

## Usage

**Open audio device using Open AL**
```javascript
import { createRequire } from 'module';
const AV = createRequire(import.meta.url)('./neon_al.node')

// open al handling...
aDevice = AL.alcOpenDevice();
aContext = AL.alcCreateContext(aDevice);

// make active context
AL.alcMakeContextCurrent(aContext);
alSource = AL.alGenSources(1);
AL.alGenBuffers();
AL.alSourcei(alSource, AL.AL_LOOPING, AL.AL_FALSE);
AV.avcodec_resampler(audioCtx);
```

**Play audio stream**
```javascript
let a_count = 0;
function play_audio(context) {
	let ret = AV.avcodec_decode(ic, null, -1, audioCtx, audioStream);
	if(ret.type != AV.AVMEDIA_TYPE_AUDIO) return;

	// Prefill all of the buffers
    let sample_rate = AV.avcodec_sample_rate(audioCtx);
    if(a_count < 3) {
        AL.alBufferData(a_count, AL.AL_FORMAT_STEREO16, ret.buffer, sample_rate);
        a_count++;
        ret = ret.buffer = null;
        
        if(a_count == 3) {
            AL.alSourceQueueBuffers(alSource, 3);
            AL.alSourcePlay(alSource);
        }
    } else {
    	let val = 0;
            /* Check if OpenAL is done with any of the queued buffers */
        do {
            val = AL.alGetSourcei(alSource, AL.AL_BUFFERS_PROCESSED);
            AV.av_usleep(10);
        } while(val <= 0);
        
        /* Pop the oldest queued buffer from the source, fill it
         * with the new data, then requeue it */
        //let buffer;
        while(val--) {
        	AL.alFillData(alSource, ret.buffer, sample_rate);
        }
        ret = ret.buffer = null;
        
        /* Make sure the source is still playing, and restart it if needed. */
        val = AL.alGetSourcei(alSource, AL.AL_SOURCE_STATE);
        if(val != AL.AL_PLAYING)
            AL.alSourcePlay(alSource);
    }
}
```
## Caveat
- Only part of open al API is supported.
- Currently tested only on macos