
1) nvctl — per-game launch optimizer (rust)

A single CLI that launches games/Proton with the right knobs, writes per-title profiles, primes caches, and pins CPU/IRQs.

What it sets (examples):

Driver/env tuning

__GL_SHADER_DISK_CACHE=1

__GL_SHADER_DISK_CACHE_PATH=/fastcache/$GAMEID

__GL_SYNC_TO_VBLANK=0 (unless you need VSync)

__GL_YIELD=USLEEP (test vs NOTHING)

__GL_GSYNC_ALLOWED=1/__GL_VRR_ALLOWED=1

VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/nvidia_icd.json

Proton/vkd3d-proton toggles

DXVK_ASYNC=1 (if you’re okay with the legal gray; otherwise off)

DXVK_STATE_CACHE=1, DXVK_STATE_CACHE_PATH=/fastcache/$GAMEID

VKD3D_SHADER_CACHE_PATH=/fastcache/$GAMEID

PROTON_ENABLE_NGX_UPDATER=1 (DLSS components)

Optional: WINE_FULLSCREEN_FSR=1, DXVK_FRAME_RATE=0

Wayland/Compositor

Prefer gamescope wrapper with explicit sync; set gamescope -H <res> -r <hz> --hdr-enabled where useful.

Enforce __GLX_VENDOR_LIBRARY_NAME=nvidia when Xwayland is in play.

CPU/IRQ affinity (7950X3D-aware)

Detect L3-cache CCD and pin game + Proton workers there.

Set IRQ affinity for nvidia, nv-drm, snd_hda_intel to the same CCD.

Set process scheduler hints: SCHED_FIFO bursts for gamescope, SCHED_ISO/SCHED_BATCH for shader workers.

Cache priming

“Warm start” pass: run the game to main menu in a hidden/offscreen or low-res mode for ~30–60s to populate shader caches, then relaunch “for real”.

Tech: Zig for the CLI (fast, static, great for spawning & env), a tiny C helper for sched_setaffinity, and an optional Rust micro-service for eBPF (below).

2) nvmon — timing & hitch profiler (Rust + eBPF)

A background daemon to measure where time is lost:

Use aya-rs (eBPF) to sample CPU runqueue latency, softirqs, and per-thread stalls of the game, Proton workers, and nvidia IRQs.

LD_PRELOAD shim (C) that wraps vkQueueSubmit/vkQueuePresentKHR to timestamp submits/presents; export to a ring buffer.

Simple Web UI: frame-time histogram, submit→present delta, shader-compile spikes, IRQ latency.

Feeds nvctl suggestions (“pin to CCD0”, “move cache to NVMe”, “enable gamescope HDR path”, etc.).
