use clap_sys::{
    entry::clap_plugin_entry,
    ext::audio_ports::{
        CLAP_AUDIO_PORT_IS_MAIN, CLAP_EXT_AUDIO_PORTS, CLAP_PORT_STEREO, clap_audio_port_info,
        clap_plugin_audio_ports,
    },
    factory::plugin_factory::clap_plugin_factory,
    host::clap_host,
    id::CLAP_INVALID_ID,
    plugin::{clap_plugin, clap_plugin_descriptor},
    plugin_features::{CLAP_PLUGIN_FEATURE_INSTRUMENT, CLAP_PLUGIN_FEATURE_STEREO},
    process::{CLAP_PROCESS_CONTINUE, clap_process, clap_process_status},
    version::CLAP_VERSION,
};
use fundsp::hacker::*;
use std::ffi::{c_char, c_void};
use std::mem::ManuallyDrop;
use std::ptr::null;
use std::slice;

struct Plugin {
    plugin: Option<clap_plugin>,
    patch: Option<Box<dyn AudioUnit>>,
}

impl Plugin {
    fn new() -> Self {
        Self {
            plugin: None,
            patch: None,
        }
    }
}

static PLUGIN_DESTRIPTOR: clap_plugin_descriptor = clap_plugin_descriptor {
    clap_version: CLAP_VERSION,
    id: c"com.plugggs.clap_sys_saw".as_ptr(),
    name: c"Saw".as_ptr(),
    vendor: c"Plugggs".as_ptr(),
    url: c"".as_ptr(),
    manual_url: c"".as_ptr(),
    support_url: c"".as_ptr(),
    version: c"".as_ptr(),
    description: c"".as_ptr(),
    features: &[
        CLAP_PLUGIN_FEATURE_STEREO.as_ptr(),
        CLAP_PLUGIN_FEATURE_INSTRUMENT.as_ptr(),
        null(),
    ] as *const _,
};

extern "C" fn plugin_init(_plugin: *const clap_plugin) -> bool {
    true
}

extern "C" fn plugin_destroy(_plugin: *const clap_plugin) {
    if _plugin.is_null() {
        panic!("")
    }
    let plug = unsafe { (*_plugin).plugin_data } as *mut Plugin;
    let _ = unsafe { Box::from_raw(plug) };
}

extern "C" fn plugin_activate(
    _plugin: *const clap_plugin,
    sample_rate: f64,
    _min_frames_count: u32,
    _max_frames_count: u32,
) -> bool {
    let c = 0.2 * (organ_hz(midi_hz(57.0)) + organ_hz(midi_hz(61.0)) + organ_hz(midi_hz(64.0)));
    let c = c >> pan(0.0);

    // Add chorus.
    let c = c >> (chorus(0, 0.0, 0.01, 0.2) | chorus(1, 0.0, 0.01, 0.2));

    let mut c = c >> (declick() | declick()) >> (dcblock() | dcblock()) >> limiter_stereo(1.0, 5.0);

    c.set_sample_rate(sample_rate);
    c.allocate();

    let plug = unsafe { (*_plugin).plugin_data as *mut Plugin };
    let plug = unsafe { &mut *plug };
    let _ = plug.patch.insert(Box::new(c));

    true
}

extern "C" fn plugin_deactivate(_plugin: *const clap_plugin) {}

extern "C" fn plugin_start_processing(_plugin: *const clap_plugin) -> bool {
    true
}

extern "C" fn plugin_stop_processing(_plugin: *const clap_plugin) {}

extern "C" fn plugin_reset(_plugin: *const clap_plugin) {}

extern "C" fn plugin_process(
    plugin: *const clap_plugin,
    process: *const clap_process,
) -> clap_process_status {
    let plug = unsafe { (*plugin).plugin_data as *mut Plugin };
    let plug = unsafe { &mut *plug };

    let proccess = unsafe { &*process };
    let out_l = unsafe { *(*proccess.audio_outputs).data32 };
    let out_l = unsafe { slice::from_raw_parts_mut(out_l, proccess.frames_count as usize) };
    let out_r = unsafe { *(*proccess.audio_outputs).data32.offset(1) };
    let out_r = unsafe { slice::from_raw_parts_mut(out_r, proccess.frames_count as usize) };

    if let Some(patch) = &mut plug.patch {
        for (sample_l, sample_r) in out_l.iter_mut().zip(out_r) {
            let (in_l, in_r) = patch.get_stereo();
            *sample_l = in_l;
            *sample_r = in_r;
        }
    }
    CLAP_PROCESS_CONTINUE
}

extern "C" fn plugin_audio_ports_count(_plugin: *const clap_plugin, _is_input: bool) -> u32 {
    1
}

extern "C" fn plugin_audio_ports_get(
    _plugin: *const clap_plugin,
    index: u32,
    _is_input: bool,
    info: *mut clap_audio_port_info,
) -> bool {
    if index > 0 {
        return false;
    }

    let info = unsafe { &mut *info };
    info.id = 0;
    info.channel_count = 2;
    unsafe { libc::snprintf(info.name.as_mut_ptr(), 1, c"AB".as_ptr()) };
    info.flags = CLAP_AUDIO_PORT_IS_MAIN;
    info.port_type = CLAP_PORT_STEREO.as_ptr();
    info.in_place_pair = CLAP_INVALID_ID;

    true
}

static PLUG_AUDIO_PORTS: clap_plugin_audio_ports = clap_plugin_audio_ports {
    count: Some(plugin_audio_ports_count),
    get: Some(plugin_audio_ports_get),
};

extern "C" fn plugin_get_extension(
    _plugin: *const clap_plugin,
    id: *const c_char,
) -> *const c_void {
    if unsafe { libc::strcmp(id, CLAP_EXT_AUDIO_PORTS.as_ptr()) } == 0 {
        return &raw const PLUG_AUDIO_PORTS as *const _;
    }
    null()
}

extern "C" fn plugin_on_main_thread(_plugin: *const clap_plugin) {}

extern "C" fn plugin_factory_get_plugin_count(_factory: *const clap_plugin_factory) -> u32 {
    1
}

extern "C" fn plugin_factory_get_plugin_descriptor(
    _factory: *const clap_plugin_factory,
    _index: u32,
) -> *const clap_plugin_descriptor {
    &raw const PLUGIN_DESTRIPTOR
}

extern "C" fn plugin_factory_get_create_plugin(
    _factory: *const clap_plugin_factory,
    _host: *const clap_host,
    _plugin_id: *const c_char,
) -> *const clap_plugin {
    let mut plug = ManuallyDrop::new(Box::new(Plugin::new()));
    let p = clap_plugin {
        desc: &raw const PLUGIN_DESTRIPTOR,
        plugin_data: &raw mut **plug as *mut _,
        init: Some(plugin_init),
        destroy: Some(plugin_destroy),
        activate: Some(plugin_activate),
        deactivate: Some(plugin_deactivate),
        start_processing: Some(plugin_start_processing),
        stop_processing: Some(plugin_stop_processing),
        reset: Some(plugin_reset),
        process: Some(plugin_process),
        get_extension: Some(plugin_get_extension),
        on_main_thread: Some(plugin_on_main_thread),
    };
    let _ = plug.plugin.insert(p);
    plug.plugin.as_ref().unwrap()
}

static PLUGIN_FACTORY: clap_plugin_factory = clap_plugin_factory {
    get_plugin_count: Some(plugin_factory_get_plugin_count),
    get_plugin_descriptor: Some(plugin_factory_get_plugin_descriptor),
    create_plugin: Some(plugin_factory_get_create_plugin),
};

extern "C" fn entry_init(_plugin_path: *const c_char) -> bool {
    true
}

extern "C" fn entry_deinit() {}

extern "C" fn entry_get_factory(_factory_id: *const c_char) -> *const c_void {
    &raw const PLUGIN_FACTORY as *const _
}

#[allow(warnings, unused)]
#[allow(non_upper_case_globals)]
#[unsafe(no_mangle)]
static clap_entry: clap_plugin_entry = clap_plugin_entry {
    clap_version: CLAP_VERSION,
    init: Some(entry_init),
    deinit: Some(entry_deinit),
    get_factory: Some(entry_get_factory),
};
