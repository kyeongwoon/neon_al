#![allow(non_snake_case)]
use neon::{prelude::*, types::buffer::TypedArray};
use std::ffi::*;
extern crate openal_sys as al;

static mut SAMPLE_SET: [u32; 4] = [0; 4];

fn alcOpenDevice(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let dev = unsafe { al::alcOpenDevice(std::ptr::null()) };
    if dev.is_null() {
        println!("failed to open device");
    } else {
        //println!("device opened");
    }
    Ok(cx.number(dev as usize as f64))
}

fn alGetError(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let err = unsafe { al::alGetError() };
    Ok(cx.number(err as usize as f64))
}

fn alcCreateContext(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let x = cx.argument::<JsNumber>(0)?.value(&mut cx) as usize;
    let context = unsafe { al::alcCreateContext(x as *mut c_void, std::ptr::null()) };
    if context.is_null() {
        println!("failed to create context");
    } else {
        //println!("context created");
    }
    Ok(cx.number(context as usize as f64))
}

fn alcMakeContextCurrent(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let context = cx.argument::<JsNumber>(0)?.value(&mut cx) as usize;
    unsafe {
        al::alcMakeContextCurrent(context as *mut c_void);
    }
    Ok(cx.undefined())
}

fn alGenSources(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let n = cx.argument::<JsNumber>(0)?.value(&mut cx) as i32;

    let mut alSource = 0;
    unsafe {
        al::alGenSources(n, &mut alSource);
    }
    Ok(cx.number(alSource as f64))
}

fn alGenBuffers(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    unsafe {
        al::alGenBuffers(4, SAMPLE_SET.as_mut_ptr() as *mut u32);
    }
    Ok(cx.undefined())
}

fn alListener3f(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let param = cx.argument::<JsNumber>(0)?.value(&mut cx) as i32;
    let v1 = cx.argument::<JsNumber>(1)?.value(&mut cx) as f32;
    let v2 = cx.argument::<JsNumber>(2)?.value(&mut cx) as f32;
    let v3 = cx.argument::<JsNumber>(3)?.value(&mut cx) as f32;
    unsafe {
        al::alListener3f(param, v1, v2, v3);
    }
    Ok(cx.undefined())
}

fn alSourcePlay(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let source = cx.argument::<JsNumber>(0)?.value(&mut cx) as u32;
    unsafe {
        al::alSourcePlay(source);
    }
    Ok(cx.undefined())
}

fn alSourceStop(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let source = cx.argument::<JsNumber>(0)?.value(&mut cx) as u32;
    unsafe {
        al::alSourceStop(source);
    }
    Ok(cx.undefined())
}

fn alSourcei(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let source = cx.argument::<JsNumber>(0)?.value(&mut cx) as u32;
    let param = cx.argument::<JsNumber>(1)?.value(&mut cx) as i32;
    let value = cx.argument::<JsNumber>(2)?.value(&mut cx) as i32;

    unsafe {
        al::alSourcei(source, param, value);
    }
    Ok(cx.undefined())
}

fn alGetSourcei(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let source = cx.argument::<JsNumber>(0)?.value(&mut cx) as u32;
    let pname = cx.argument::<JsNumber>(1)?.value(&mut cx) as i32;

    let mut val = 0;
    unsafe {
        al::alGetSourcei(source, pname, &mut val);
    }

    Ok(cx.number(val as f64))
}

fn alBufferData(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let buffer = cx.argument::<JsNumber>(0)?.value(&mut cx) as usize;
    let format = cx.argument::<JsNumber>(1)?.value(&mut cx) as i32;
    let audio_len = cx.argument::<JsNumber>(3)?.value(&mut cx) as i32;
    let freq = cx.argument::<JsNumber>(4)?.value(&mut cx) as i32;

    let buf = cx.argument::<JsTypedArray<u8>>(2)?;
    let _size = buf.len(&mut cx);
    let data = buf.as_slice(&cx);
    let len = audio_len;

    //println!("alBufferData {} {} {} {}", buffer, format, len, freq);

    unsafe {
        al::alBufferData(
            SAMPLE_SET[buffer],
            format,
            data.as_ptr() as *const al::ALvoid,
            len as i32,
            freq,
        );
    }
    Ok(cx.undefined())
}

fn alSourceQueueBuffers(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let source = cx.argument::<JsNumber>(0)?.value(&mut cx) as u32;
    let n = cx.argument::<JsNumber>(1)?.value(&mut cx) as i32;

    //let mut buffers = 0;
    unsafe {
        al::alSourceQueueBuffers(source, n, SAMPLE_SET.as_ptr());
    }

    Ok(cx.undefined())
}

fn alSourceUnqueueBuffers(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let source = cx.argument::<JsNumber>(0)?.value(&mut cx) as u32;
    let n = cx.argument::<JsNumber>(1)?.value(&mut cx) as i32;

    let mut buffers = 0;
    unsafe {
        al::alSourceUnqueueBuffers(source, n, &mut buffers);
    }

    Ok(cx.undefined())
}

fn alDeleteSources(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let n = cx.argument::<JsNumber>(0)?.value(&mut cx) as i32;
    let mut sources = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;

    unsafe {
        al::alDeleteSources(n, &mut sources);
    }

    Ok(cx.undefined())
}

fn alDeleteBuffers(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let n = cx.argument::<JsNumber>(0)?.value(&mut cx) as i32;
    //let buffers = cx.argument::<JsNumber>(1)?.value(&mut cx) as usize as *const u32;

    unsafe {
        al::alDeleteBuffers(n, SAMPLE_SET.as_ptr());
    }
    Ok(cx.undefined())
}

fn alcDestroyContext(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let context = cx.argument::<JsNumber>(0)?.value(&mut cx) as usize as *mut c_void;
    unsafe {
        al::alcDestroyContext(context);
    }

    Ok(cx.undefined())
}

fn alcCloseDevice(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let device = cx.argument::<JsNumber>(0)?.value(&mut cx) as usize as *mut c_void;
    let ret = unsafe { al::alcCloseDevice(device) };
    Ok(cx.boolean(ret != 0))
}

// custom convienient function
fn alFillData(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let source = cx.argument::<JsNumber>(0)?.value(&mut cx) as u32;
    let buf = cx.argument::<JsTypedArray<u8>>(1)?;

    let audio_len = cx.argument::<JsNumber>(2)?.value(&mut cx) as i32;
    let freq = cx.argument::<JsNumber>(3)?.value(&mut cx) as i32;
    let _size = buf.len(&mut cx);
    let data = buf.as_slice(&cx);
    //println!("size is {}", size);
    let len = audio_len;

    let mut buffer = 0;
    unsafe {
        // one bye one
        al::alSourceUnqueueBuffers(source, 1, &mut buffer);
        al::alBufferData(
            buffer,
            al::AL_FORMAT_STEREO16,
            data.as_ptr() as *const al::ALvoid,
            len as al::ALsizei,
            freq as al::ALsizei,
        );
        al::alSourceQueueBuffers(source, 1, &mut buffer);
    }

    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    let val = cx.number(al::AL_LOOPING);
    cx.export_value("AL_LOOPING", val)?;
    let val = cx.number(al::AL_PLAYING);
    cx.export_value("AL_PLAYING", val)?;
    let val = cx.number(al::AL_BUFFER);
    cx.export_value("AL_BUFFER", val)?;
    let val = cx.number(al::AL_NO_ERROR);
    cx.export_value("AL_NO_ERROR", val)?;
    let val = cx.number(al::AL_FALSE);
    cx.export_value("AL_FALSE", val)?;
    let val = cx.number(al::AL_FORMAT_MONO8);
    cx.export_value("AL_FORMAT_MONO8", val)?;
    let val = cx.number(al::AL_FORMAT_MONO16);
    cx.export_value("AL_FORMAT_MONO16", val)?;
    let val = cx.number(al::AL_FORMAT_STEREO8);
    cx.export_value("AL_FORMAT_STEREO8", val)?;
    let val = cx.number(al::AL_FORMAT_STEREO16);
    cx.export_value("AL_FORMAT_STEREO16", val)?;
    let val = cx.number(al::AL_SOURCE_STATE);
    cx.export_value("AL_SOURCE_STATE", val)?;
    let val = cx.number(al::AL_BUFFERS_PROCESSED);
    cx.export_value("AL_BUFFERS_PROCESSED", val)?;
    let val = cx.number(al::AL_BUFFERS_QUEUED);
    cx.export_value("AL_BUFFERS_QUEUED", val)?;

    cx.export_function("alcOpenDevice", alcOpenDevice)?;
    cx.export_function("alGetError", alGetError)?;
    cx.export_function("alcCreateContext", alcCreateContext)?;
    cx.export_function("alcMakeContextCurrent", alcMakeContextCurrent)?;
    cx.export_function("alGenSources", alGenSources)?;
    cx.export_function("alGenBuffers", alGenBuffers)?;
    cx.export_function("alSourcePlay", alSourcePlay)?;
    cx.export_function("alSourceStop", alSourceStop)?;
    cx.export_function("alSourcei", alSourcei)?;
    cx.export_function("alListener3f", alListener3f)?;
    cx.export_function("alGetSourcei", alGetSourcei)?;
    cx.export_function("alBufferData", alBufferData)?;
    cx.export_function("alSourceQueueBuffers", alSourceQueueBuffers)?;
    cx.export_function("alSourceUnqueueBuffers", alSourceUnqueueBuffers)?;
    cx.export_function("alDeleteSources", alDeleteSources)?;
    cx.export_function("alDeleteBuffers", alDeleteBuffers)?;
    cx.export_function("alcDestroyContext", alcDestroyContext)?;
    cx.export_function("alcCloseDevice", alcCloseDevice)?;
    cx.export_function("alFillData", alFillData)?;
    Ok(())
}
