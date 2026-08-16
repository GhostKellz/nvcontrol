# GeForce NOW Linux

NVIDIA's native GeForce NOW Linux client is distributed through Flathub as
`com.nvidia.geforcenow`. This is a cloud-gaming client, not a Linux port of the
Windows NVIDIA App control panel.

```bash
flatpak install --user flathub com.nvidia.geforcenow
flatpak run com.nvidia.geforcenow
nvctl driver info
```

nvcontrol integration is intentionally read-only:

- detect system or user installation of the official Flatpak;
- report `VK_KHR_video_decode_h265` on the active NVIDIA Vulkan device;
- include the result in driver diagnostics without making GFN mandatory;
- leave authentication, streaming configuration, updates, and game launching to
  the NVIDIA client.

The Flatpak NVIDIA GL extension must match the host driver. If the client stops
launching after a driver update, update Flatpak runtimes and confirm the matching
`org.freedesktop.Platform.GL.nvidia-*` extension is installed.

`VK_EXT_descriptor_heap` is reported separately because it is relevant to modern
VKD3D-Proton Direct3D 12 paths. It is not a switch nvcontrol should force globally
and it is not the GeForce NOW H.265 readiness signal.

References:

- [NVIDIA GeForce NOW Linux announcement](https://blogs.nvidia.com/blog/geforce-now-thursday-linux-native-app/)
- [NVIDIA GeForce NOW downloads](https://www.nvidia.com/en-us/geforce-now/download/)
- [ArchWiki GeForce NOW](https://wiki.archlinux.org/title/NVIDIA_GeForce_NOW)
